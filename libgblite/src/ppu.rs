// PPU abstracts the details of the PPU from the CPU. It's different from the Window struct because
// the window abstracts platform-specific details related to operating the window.

use crate::util;
use crate::memory::Memory;
use crate::memory::MemClient;
use crate::window::Window;

use std::fmt::{Display, Formatter, Result};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;

#[derive(Copy, Clone, PartialEq)]
enum PPUState {
    HBlank    = 0, // HBlank is the LCD idle period after each line is drawn.
    VBlank    = 1, // VBlank is the LCD idle period after the final line is drawn.
    OAMSearch = 2, // OAM Search is the initial linear scan of objects on a given line.
    Draw      = 3, // Draw is the lookup and transfer period of pixels to the LCD.
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum PPUReg {
    Lcdc = 0xFF40,
    Stat = 0xFF41,
    Scy  = 0xFF42,
    Scx  = 0xFF43,
    Ly   = 0xFF44,
    Lyc  = 0xFF45,
    Dma  = 0xFF46,
    Bgp  = 0xFF47,
    Obp0 = 0xFF48,
    Obp1 = 0xFF49,
    Wy   = 0xFF4A,
    Wx   = 0xFF4B,
    Vbk  = 0xFF4F
}

impl Display for PPUReg {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            PPUReg::Lcdc => write!(f, "LCDC"),
            PPUReg::Stat => write!(f, "STAT"),
            PPUReg::Scy  => write!(f, "SCY"),
            PPUReg::Scx  => write!(f, "SCX"),
            PPUReg::Ly   => write!(f, "LY"),
            PPUReg::Lyc  => write!(f, "LYC"),
            PPUReg::Dma  => write!(f, "DMA"),
            PPUReg::Bgp  => write!(f, "BGP"),
            PPUReg::Obp0 => write!(f, "OBP0"),
            PPUReg::Obp1 => write!(f, "OBP1"),
            PPUReg::Wy   => write!(f, "WY"),
            PPUReg::Wx   => write!(f, "WX"),
            PPUReg::Vbk  => write!(f, "VBK"),
        }
    }
}

struct PPUConfig {
    regs: Vec<PPUReg>,
    lcd_enabled: bool,       // LCDC bit 7 - Enables the LCD
    win_map_high_bank: bool, // LCDC bit 6 - Changes window map start address to high bank
    win_en: bool,            // LCDC bit 5 - Enables window rendering
    bg_data_low_bank: bool,  // LCDC bit 4 - Changes BG/Window data start address to low bank
    bg_map_high_bank: bool,  // LCDC bit 3 - Changes BG map start address to high bank
    tall_objs: bool,         // LCDC bit 2 - Enables tall sprites
    obj_en: bool,            // LCDC bit 1 - Enables sprite rendering
    bg_priority: bool,       // LCDC bit 0 - Forces BG pixels to highest priority (over OBJs)
    ly_eq_lyc_intr: bool,    // STAT bit 6 - Enable the LY==LYC coincidence interrupt
    oam_intr: bool,          // STAT bit 5 - Enable the OAM interrupt
    vblank_intr: bool,       // STAT bit 4 - Enable the VBLANK interrupt
    hblank_intr: bool,       // STAT bit 3 - Enable the HBLANK interrupt
    ly_eq_lyc: bool,         // STAT bit 2 - LY==LYC if true
    state: PPUState,         // STAT bit 0-1 - LCD state mode flag
    scy: u8,                 // SCY - the scroll X offset
    scx: u8,                 // SCX - the scroll Y offset
    ly:  u8,                 // LY register - the current Y line we're rendering.
    lx:  u8,                 // The X pixel we're rendering - this doesn't map to a hardware register.
    lyc: u8,                 // LYC - line Y compare value, used for the LYC interrupt.
    dma: u8,                 // DMA - function to DMA from generic memory point to OAM RAM.
    bgp: u8,                 // BGP - background palette
    obp0: u8,                // OBP0 - object palette 0
    obp1: u8,                // OBP1 - object palette 1
    wy: u8,                  // WY - the window Y offset
    wx: u8,                  // WX - the window X offset
    vbk_enable: bool,        // VBK bit 0 - enable VRAM bank 1, CGB only
}

#[derive(Copy, Clone, PartialEq)]
struct PPUDebug {
    enabled:    bool,        // True if debug logging is enabled
    last_frame: Instant,     // Timestamp of last frame rendered, to calculate framerate.
}

pub struct PPU {
    lcd: Window,             // The actual graphics window, not to be confused with a Game Boy window map/tile.
    mem: Arc<Mutex<Memory>>, // Reference to our Memory object.
    pixels: Vec<u8>,         // Vector containing pixel data. Currently UINT RGB8 format.
    cfg: PPUConfig,          // Struct containing all PPU register config values
    dbg: PPUDebug,           // Struct containing debug information and statistics
    lclk: u32,               // The machine cycle for this line, from [0, 113].
    alive: bool,             // Whether or not the application should continue running. This is != LCD disabled.
}

impl PPU {

    const WIDTH:  usize = 160;
    const HEIGHT: usize = 144;

    pub fn new(mem: Arc<Mutex<Memory>>) -> Self {
        let lcd = Window::new(PPU::WIDTH, PPU::HEIGHT);

        let regs: Vec<PPUReg> = [
            PPUReg::Lcdc,
            PPUReg::Stat,
            PPUReg::Scy,
            PPUReg::Scx,
            PPUReg::Ly,
            PPUReg::Lyc,
            PPUReg::Dma,
            PPUReg::Bgp,
            PPUReg::Obp0,
            PPUReg::Obp1,
            PPUReg::Wy,
            PPUReg::Wx,
            PPUReg::Vbk,
        ].iter().cloned().collect();

        let cfg = PPUConfig {
            regs: regs,
            lcd_enabled: true,
            win_map_high_bank: false,
            win_en: false,
            bg_data_low_bank: true,
            bg_map_high_bank: false,
            tall_objs: false,
            obj_en: false,
            bg_priority: true,
            ly_eq_lyc_intr: false,
            oam_intr: false,
            vblank_intr: false,
            hblank_intr: false,
            ly_eq_lyc: true,
            state: PPUState::VBlank,
            scy: 0,
            scx: 0,
            ly: 0,
            lx: 0,
            lyc: 0,
            dma: 0,
            bgp: 0xfc,
            obp0: 0xff,
            obp1: 0xff,
            wy: 0,
            wx: 0,
            vbk_enable: false,
        };

        let dbg = PPUDebug {
            enabled: false,
            last_frame: Instant::now(),
        };

        let mut ppu = PPU {
            lcd: lcd,
            mem: mem,
            pixels: vec![0; PPU::WIDTH*PPU::HEIGHT*3],
            cfg: cfg,
            dbg: dbg,
            lclk: 0,
            alive: true,
        };

        // Initialize PPU config registers
        ppu.push_registers();

        ppu
    }

    // Tick performs the appropriate PPU action for this machine cycle.
    // TODO: Adjust cycle accuracy of Draw state, timings can vary slightly.
    pub fn tick(&mut self) {

        /*
         * PPU clock cycle overview
         * 1. Pull PPU CFG registers and modify settings accordingly
         * 2. Check for window events
         * 3. Determine the current PPUState
         * 4. Do the appropriate work for this state
         * 5. Flush register changes
         */

        // Check window events and for register changes
        self.pull_registers();
        self.check_events();

        if !self.alive { return; }

        if self.cfg.lcd_enabled {
            match self.cfg.state {
                PPUState::HBlank => {
                    if self.lclk == 63 {
                        self.render_line();
                        if self.cfg.ly == 143 {
                            self.present();
                        }
                    }
                    if self.lclk == 113 {
                        if self.cfg.ly == 143 {
                            self.cfg.state = PPUState::VBlank;
                        } else {
                            self.cfg.state = PPUState::Draw;
                        }
                        self.cfg.ly += 1;
                        self.lclk = 0;
                    } else {
                        self.lclk += 1;
                    }
                },
                PPUState::VBlank => {
                    if self.lclk == 113 {
                        if self.cfg.ly == 153 {
                            self.cfg.state = PPUState::OAMSearch;
                            self.cfg.ly = 0;
                        } else {
                            self.cfg.ly += 1;
                        }
                        self.lclk = 0;
                    } else {
                        self.lclk += 1;
                    }
                },
                PPUState::OAMSearch => {
                    if self.lclk == 19 {
                        self.cfg.state = PPUState::Draw;
                    }
                    self.lclk += 1;
                },
                PPUState::Draw => {
                    if self.lclk == 62 {
                        self.cfg.state = PPUState::HBlank;
                    }
                    self.lclk += 1;
                }
            }
        }

        self.push_registers();
    }

    fn render_line(&mut self) {
        // For each scanline...
        let wt = PPU::WIDTH / 8;
        for _w in 0..wt {
            self.get_chunk();
        }
    }

    // A "chunk" is a group of 8 horizontal pixels.
    fn get_chunk(&mut self) {
        let global_pixel_y = self.cfg.ly.wrapping_add(self.cfg.scy);
        let global_pixel_x = self.cfg.lx.wrapping_add(self.cfg.scx);

        // Get the tile coordinates, and the offset within each tile.
        let tile_y = global_pixel_y / 8;
        let tile_x = global_pixel_x / 8;
        let tile_y_offset = global_pixel_y % 8;
        let tile_x_offset = global_pixel_x % 8;

        // We export 8 pixels here, so the data could come from two adjacent tiles (due to scrolling).
        // So we get the data for both this tile and next horizontally adjacent tile.
        let data_line_ptr_cur = self.get_bg_data_ptr(tile_x, tile_y) + tile_y_offset as u16 * 2;
        let data_line_ptr_nxt = self.get_bg_data_ptr((tile_x + 1) % 32, tile_y) + tile_y_offset as u16 * 2;

        // From Pan Docs:
        // "For each line, the first byte defines the least significant bits of the color numbers
        //  for each pixel, and the second byte defines the upper bits of the color numbers. In
        //  either case, Bit 7 is the leftmost pixel, and Bit 0 the rightmost."
        let data_line_cur = util::join_u8((self.mem_get(data_line_ptr_cur), self.mem_get(data_line_ptr_cur+1)));
        let data_line_nxt = util::join_u8((self.mem_get(data_line_ptr_nxt), self.mem_get(data_line_ptr_nxt+1)));

        let hi_bits = (data_line_cur & 0xFF00) | (data_line_nxt >> 8);
        let lo_bits = (data_line_cur << 8) | (data_line_nxt & 0xFF);

        let mut hi_bits = hi_bits.reverse_bits() >> tile_x_offset;
        let mut lo_bits = lo_bits.reverse_bits() >> tile_x_offset;

        // We're almost there!
        for _x in 0..8 {
            let val: u8 = ((hi_bits & 0x1) as u8) << 1 | (lo_bits & 0x1) as u8;
            let write_addr = ((self.cfg.ly as usize * PPU::WIDTH) + self.cfg.lx as usize) * 3;
            hi_bits = hi_bits >> 1;
            lo_bits = lo_bits >> 1;

            // TODO: Map this value to a palette value
            let (r,g,b) = match val {
                0 => { (0xFF, 0xFF, 0xFF) },
                1 => { (0xAA, 0xAA, 0xAA) },
                2 => { (0x55, 0x55, 0x55) },
                3 => { (0x00, 0x00, 0x00) },
                _ => { (0xFF, 0x00, 0x00) },
            };

            self.pixels[write_addr+0] = r;
            self.pixels[write_addr+1] = g;
            self.pixels[write_addr+2] = b;
            self.cfg.lx = (self.cfg.lx + 1) % PPU::WIDTH as u8;
        }
    }

    // Given the coordinates of a BG map tile, return the start address of that tile's data.
    fn get_bg_data_ptr(&self, tx: u8, ty: u8) -> u16 {
        let base_bg_map_addr: u16 = if self.cfg.bg_map_high_bank { 0x9c00 } else { 0x9800 };
        let base_bg_data_addr: u16 = if self.cfg.bg_data_low_bank { 0x8000 } else { 0x9000 };
        let bg_map_ptr = base_bg_map_addr + (ty as u16)*32 + tx as u16;
        let bg_data_offset = self.mem_get(bg_map_ptr);

        // Depending on the bank location, the addressing mode is different.
        // High-bank config uses a signed integer offset, low-bank is unsigned.
        let bg_data_offset = if self.cfg.bg_data_low_bank {
            bg_data_offset as i16
        } else {
            (bg_data_offset as i8) as i16
        };

        // We multiply the offset by 16 because that's the number of bytes per-tile.
        (base_bg_data_addr as i16 + bg_data_offset * 16) as u16
    }

    fn present(&mut self) {
        self.lcd.draw(self.pixels.as_slice());

        if self.dbg.enabled {
            let now = Instant::now();
            let frame_time = now.duration_since(self.dbg.last_frame).as_micros();
            self.dbg.last_frame = now;
            println!("Render time for this frame: {} us, or {:.2} fps.", frame_time, (1.0 / frame_time as f64) * 1000000.0);
        }
    }

    pub fn terminate(&mut self) {
        self.alive = false;
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    fn check_events(&mut self) {
        // Do nothing if we've terminated the application.
        if !self.is_alive() {
            return;
        }

        // Check window for termination events
        if self.cfg.state == PPUState::VBlank {
            self.lcd.get_events();
        }
        if !self.lcd.is_open() {
            self.terminate();
            return;
        }

        // Check for LY==LYC
        // TODO process the LYC interrupt here?
        self.cfg.ly_eq_lyc = self.cfg.ly == self.cfg.lyc;
    }

    // Check for register changes, and apply the corresponding settings differences.
    // TODO: Some registers can't be changed halfway through a scanline, check for those here.
    fn pull_registers(&mut self) {
        // Collect the values before writing to prevent borrowing issues.
        // let regs = self.cfg.regs.cloned();
        for reg in self.cfg.regs.iter() {
            let val = self.mem_get(*reg as u16);

            match reg {
                PPUReg::Lcdc => {
                    self.cfg.lcd_enabled        = (val & 0x80) != 0;
                    self.cfg.win_map_high_bank  = (val & 0x40) != 0;
                    self.cfg.win_en             = (val & 0x20) != 0;
                    self.cfg.bg_data_low_bank   = (val & 0x10) != 0;
                    self.cfg.bg_map_high_bank   = (val & 0x08) != 0;
                    self.cfg.tall_objs          = (val & 0x04) != 0;
                    self.cfg.obj_en             = (val & 0x02) != 0;
                    self.cfg.bg_priority        = (val & 0x01) != 0;
                },
                PPUReg::Stat => {
                    self.cfg.ly_eq_lyc_intr  = (val & 0x40) != 0;
                    self.cfg.oam_intr        = (val & 0x20) != 0;
                    self.cfg.vblank_intr     = (val & 0x10) != 0;
                    self.cfg.hblank_intr     = (val & 0x08) != 0;
                },
                PPUReg::Bgp  => {
                    self.cfg.bgp  = val; // TODO: split this up
                }
                PPUReg::Scy  => self.cfg.scy  = val,
                PPUReg::Scx  => self.cfg.scx  = val,
                PPUReg::Ly   => self.cfg.ly   = val,
                PPUReg::Lyc  => self.cfg.lyc  = val,
                PPUReg::Dma  => self.cfg.dma  = val,
                PPUReg::Obp0 => self.cfg.obp0 = val,
                PPUReg::Obp1 => self.cfg.obp1 = val,
                PPUReg::Wy   => self.cfg.wy   = val,
                PPUReg::Wx   => self.cfg.wx   = val,
                PPUReg::Vbk  => self.cfg.vbk_enable = val == 1,
            }
        }
    }

    // Flush register changes to memory
    fn push_registers(&mut self) {
        let regs: Vec<PPUReg> = self.cfg.regs.iter().cloned().collect();
        for reg in regs {
            // Encode our current config state into actual register values
            let val = match reg {
                PPUReg::Lcdc => {
                    (if self.cfg.lcd_enabled        { 1 } else { 0 } << 7) |
                    (if self.cfg.win_map_high_bank  { 1 } else { 0 } << 6) |
                    (if self.cfg.win_en             { 1 } else { 0 } << 5) |
                    (if self.cfg.bg_data_low_bank   { 1 } else { 0 } << 4) |
                    (if self.cfg.bg_map_high_bank   { 1 } else { 0 } << 3) |
                    (if self.cfg.tall_objs          { 1 } else { 0 } << 2) |
                    (if self.cfg.obj_en             { 1 } else { 0 } << 1) |
                    (if self.cfg.bg_priority        { 1 } else { 0 } << 0)
                },
                PPUReg::Stat => {
                    (0x1 << 7) | // Bit 7 of STAT always returns 1
                    (if self.cfg.ly_eq_lyc_intr     { 1 } else { 0 } << 6) |
                    (if self.cfg.oam_intr           { 1 } else { 0 } << 5) |
                    (if self.cfg.vblank_intr        { 1 } else { 0 } << 4) |
                    (if self.cfg.hblank_intr        { 1 } else { 0 } << 3) |
                    (if self.cfg.ly_eq_lyc          { 1 } else { 0 } << 2) |
                    (if self.cfg.lcd_enabled { (self.cfg.state as u8) & 0x3 } else { 0 })
                },
                PPUReg::Bgp => {
                    self.cfg.bgp //TODO: split this up
                },
                PPUReg::Scy  => self.cfg.scy,
                PPUReg::Scx  => self.cfg.scx,
                PPUReg::Ly   => self.cfg.ly,
                PPUReg::Lyc  => self.cfg.lyc,
                PPUReg::Dma  => self.cfg.dma,
                PPUReg::Obp0 => self.cfg.obp0,
                PPUReg::Obp1 => self.cfg.obp1,
                PPUReg::Wy   => self.cfg.wy,
                PPUReg::Wx   => self.cfg.wx,
                PPUReg::Vbk  => if self.cfg.vbk_enable { 1 } else { 0 },
            };

            self.mem_set(reg as u16, val);
        }
    }

    // VRAM data access, given absolute memory address
    // VRAM [0x8000, 0xa000) -> [0x0, 0x2000]
    // OAM RAM access [0xFE00, 0xFEA0) -> []
    fn mem_get(&self, addr: u16) -> u8 {
        let mref = self.mem.lock().unwrap();
        (*mref).get(addr, MemClient::PPU)
    }

    fn mem_set(&mut self, addr: u16, val: u8) {
        let mut mref = self.mem.lock().unwrap();
        (*mref).set(val, addr, MemClient::PPU)
    }
}
