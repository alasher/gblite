// PPU abstracts the details of the PPU from the CPU. It's different from the Window struct because
// the window abstracts platform-specific details related to operating the window.

use crate::window::Window;
use crate::memory::Memory;
use crate::memory::MemClient;

use std::fmt::{Display, Formatter, Result};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;

#[derive(Copy, Clone, PartialEq)]
enum PPUState {
    Quit,        // Quit is a signal from the OS window indicating to terminate gblite.
    Off,         // Off keeps the application open, but leaves the LCD inactive.
    HBlank,      // HBlank is the LCD idle period after each line is drawn.
    VBlank,      // VBlank is the LCD idle period after the final line is drawn.
    OAMSearch,   // OAM Search is the initial linear scan of objects on a given line.
    Draw         // Draw is the lookup and transfer period of pixels to the LCD.
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum PPUReg {
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
    stat: u8,
    scy: u8,
    scx: u8,
    ly:  u8,
    lyc: u8,
    dma: u8,
    bgp: u8,
    obp0: u8,
    obp1: u8,
    wy: u8,
    wx: u8,
    vbk: u8,
}

#[derive(Copy, Clone, PartialEq)]
struct PPUDebug {
    enabled:    bool,        // True if debug logging is enabled
    last_frame: Instant,     // Timestamp of last frame rendered, to calculate framerate.
}

pub struct PPU {
    lcd: Window,             // The actual graphics window, not to be confused with a Game Boy window map/tile.
    state: PPUState,         // Current PPU state, non-off is STAT[0:1], OFF is controlled by LCDC bit 7.
    mem: Arc<Mutex<Memory>>, // Reference to our Memory object.
    cfg: PPUConfig,          // Struct containing all PPU register config values
    dbg: PPUDebug,           // Struct containing debug information and statistics
    width: u32,              // Width of the virtual window, fixed at 160.
    height: u32,             // Height of the virtual window, fixed at 144.
    lclk: u32,               // The machine cycle for this line, from [0, 113].
}

impl PPU {
    pub fn new(mem: Arc<Mutex<Memory>>) -> Self {
        let (w, h) = (160u8, 144u8);
        let lcd = Window::new(w.into(), h.into());

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
            lcd_enabled: true,         // LCDC bit 7
            win_map_high_bank: false,  // LCDC bit 6
            win_en: false,             // LCDC bit 5
            bg_data_low_bank: true,    // LCDC bit 4
            bg_map_high_bank: false,   // LCDC bit 3
            tall_objs: false,          // LCDC bit 2
            obj_en: false,             // LCDC bit 1
            bg_priority: true,         // LCDC bit 0
            stat: 0,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            dma: 0,
            bgp: 0xfc,
            obp0: 0xff,
            obp1: 0xff,
            wy: 0,
            wx: 0,
            vbk: 0,
        };

        let dbg = PPUDebug {
            enabled: false,
            last_frame: Instant::now(),
        };

        let mut ppu = PPU {
            lcd: lcd,
            state: PPUState::Off,
            mem: mem,
            cfg: cfg,
            dbg: dbg,
            width: w,
            height: h,
            lclk: 0,
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

        match self.state {
            PPUState::Quit => (),
            PPUState::Off => (),
            PPUState::HBlank => {
                if self.lclk == 113 {
                    if self.cfg.ly == 143 {
                        self.state = PPUState::VBlank;
                        self.render();
                    } else {
                        self.state = PPUState::Draw;
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
                        self.state = PPUState::OAMSearch;
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
                    self.state = PPUState::Draw;
                }
                self.lclk += 1;
            },
            PPUState::Draw => {
                if self.lclk == 62 {
                    self.state = PPUState::HBlank;
                }
                self.lclk += 1;
            }
        }

        self.push_registers();
    }

    // Start and stop are not public, they must be activated by LCDC.
    fn start(&mut self) {
        self.state = PPUState::OAMSearch;
        self.lclk = 0;
        self.cfg.ly = 0;
        self.render();
    }

    fn render(&mut self) {
        // TODO: Right now pixel format is RGB8 (8 bits for each component)
        // This can probably be lowered once I know more about the CGB.
        let mut pixels = Vec::new();
        for w in 0..self.width {
            let start_nums = (61.0,  74.0, 130.0);
            let end_nums   = (72.0, 210.0, 219.0);
            let pratio = (w as f32) / (self.width as f32);
            for _h in 0..self.height {
                pixels.push((start_nums.0 + pratio*(end_nums.0 - start_nums.0)) as u8);
                pixels.push((start_nums.1 + pratio*(end_nums.1 - start_nums.1)) as u8);
                pixels.push((start_nums.2 + pratio*(end_nums.2 - start_nums.2)) as u8);
            }
        }

        self.lcd.draw(&pixels[..]);

        if self.dbg.enabled {
            let now = Instant::now();
            let frame_time = now.duration_since(self.dbg.last_frame);
            self.dbg.last_frame = now;
            println!("Render time for this frame: {} ms, or {} fps.", frame_time.as_millis(), 1.0 / (frame_time.as_millis() as u32) as f32 * 1000.0);
        }
    }

    // Right now, just get the whole scanline at one time.
    // TODO: In the future, I should probably make sure this is cycle-accurate
    // fn get_pixels(&mut self) -> &[u8] {
    // }

    fn stop(&mut self) {
        self.state = PPUState::Off;
    }

    pub fn terminate(&mut self) {
        self.state = PPUState::Quit;
    }

    pub fn is_rendering(&self) -> bool {
        self.state != PPUState::Off && self.is_alive()
    }

    pub fn is_alive(&self) -> bool {
        self.state != PPUState::Quit
    }

    fn check_events(&mut self) {
        // Do nothing if we've terminated the application.
        if !self.is_alive() {
            return;
        }

        // Check window for termination events
        self.lcd.get_events();
        if !self.lcd.is_open() {
            self.terminate();
            return;
        }

        // Check LCDC for status changes.
        if self.cfg.lcd_enabled && !self.is_rendering() {
            self.start();
        } else if !self.cfg.lcd_enabled && self.is_rendering() {
            self.stop();
        }
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
                    self.cfg.lcd_enabled         = (val & 0x80) != 0;
                    self.cfg.win_map_high_bank   = (val & 0x40) != 0;
                    self.cfg.win_en              = (val & 0x20) != 0;
                    self.cfg.bg_data_low_bank    = (val & 0x10) != 0;
                    self.cfg.bg_map_high_bank    = (val & 0x08) != 0;
                    self.cfg.tall_objs           = (val & 0x04) != 0;
                    self.cfg.obj_en              = (val & 0x02) != 0;
                    self.cfg.bg_priority         = (val & 0x01) != 0;
                },
                PPUReg::Stat => {
                    self.cfg.stat = val; // TODO: split this up
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
                PPUReg::Vbk  => self.cfg.vbk  = val,
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
                    self.cfg.stat //TODO: split this up
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
                PPUReg::Vbk  => self.cfg.vbk,
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
