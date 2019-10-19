// PPU abstracts the details of the PPU from the CPU. It's different from the Window struct because
// the window abstracts platform-specific details related to operating the window.

use window::Window;
use memory::Memory;
use memory::MemClient;

use std::fmt::{Display, Formatter, Result};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;
use std::collections::HashMap;

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

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum PPURegField {
    LcdcEnable,
    LcdcWinMapPos,
    LcdcWinEnableLE,
    LcdcBgWinDataPos,
    LcdcBgMapPos,
    LcdcObjSize,
    LcdcObjEnable,
    LcdcObjWinPrio,
    StatCoincInt,
    StatOamInt,
    StatVblankInt,
    StatHblankInt,
    StatCoincidence,
    StatMode,
    BgpShadeColor3,
    BgpShadeColor2,
    BgpShadeColor1,
    BgpShadeColor0,
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
    cache: HashMap<PPUReg, u8>,
    dirty: HashMap<PPUReg, bool>,
    _bgr_map_off: u16,        // Offset to BG Map start address in VRAM, adjustble by LCDC bit 3.
    _win_map_off: u16,        // Offset to Window map start address in VRAM, adjustable by LCDC bit 6.
    _bgr_dat_off: u16,        // Offset to BG/Window data start address in VRAM, adjustable by LCDC bit 4.
    _win_en: bool,            // True if window is enabled. Note this window is drawn, it's not about the SDL window.
    _obj_en: bool,            // True if sprites (or OBJs) are enabled.
    _bgr_en: bool,            // True if background rendering enabled. Always enabled on CGB.
    _tall_objs: bool          // If false, an 8x8 OBJ is used. Otherwise, an 8x16 OBJ is used.
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
        let (w, h) = (160, 144);
        let lcd = Window::new(w, h);

        let cache: HashMap<PPUReg, u8> = [
            (PPUReg::Lcdc, 0),
            (PPUReg::Stat, 0),
            (PPUReg::Scy,  0),
            (PPUReg::Scx,  0),
            (PPUReg::Ly,   0),
            (PPUReg::Lyc,  0),
            (PPUReg::Dma,  0),
            (PPUReg::Bgp,  0),
            (PPUReg::Obp0, 0),
            (PPUReg::Obp1, 0),
            (PPUReg::Wy,   0),
            (PPUReg::Wx,   0),
            (PPUReg::Vbk,  0),
        ].iter().cloned().collect();

        let dirty: HashMap<PPUReg, bool> = [
            (PPUReg::Lcdc, true),
            (PPUReg::Stat, true),
            (PPUReg::Scy,  true),
            (PPUReg::Scx,  true),
            (PPUReg::Ly,   true),
            (PPUReg::Lyc,  true),
            (PPUReg::Dma,  true),
            (PPUReg::Bgp,  true),
            (PPUReg::Obp0, true),
            (PPUReg::Obp1, true),
            (PPUReg::Wy,   true),
            (PPUReg::Wx,   true),
            (PPUReg::Vbk,  true),
        ].iter().cloned().collect();

        let cfg = PPUConfig {
            cache: cache,
            dirty: dirty,
            _bgr_map_off: 0,
            _win_map_off: 0,
            _bgr_dat_off: 0,
            _win_en: false,
            _obj_en: false,
            _bgr_en: false,
            _tall_objs: false,
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
        ppu.pull_registers();
        ppu.reg_set(PPUReg::Lcdc, 0x91);
        ppu.reg_set(PPUReg::Bgp, 0xFC);
        ppu.reg_set(PPUReg::Obp0, 0xFF);
        ppu.reg_set(PPUReg::Obp1, 0xFF);
        ppu.push_registers();

        ppu
    }

    // Tick performs the appropriate PPU action for this machine cycle.
    // TODO: Adjust cycle accuracy of Draw state, timings can vary slightly.
    pub fn tick(&mut self) {

        /*
         * PPU clock cycle overview
         * 1. Check for window events
         * 2. Pull PPU CFG registers and modify settings accordingly
         * 3. Determine the current PPUState
         * 4. Do the appropriate work for this state
         * 5. Flush register changes
         */

        // We want to make it so we can write self.some_field_from_register within the PPU code,
        // then after we're done we know to generate the appropriate register value for that
        // changed value and update it.

        // Check window events and for register changes
        self.pull_registers();
        self.check_events();

        let mut ly = self.reg_get(PPUReg::Ly);

        match self.state {
            PPUState::Quit => (),
            PPUState::Off => (),
            PPUState::HBlank => {
                if self.lclk == 113 {
                    if ly == 143 {
                        self.state = PPUState::VBlank;
                        self.render();
                    } else {
                        self.state = PPUState::Draw;
                    }
                    ly += 1;
                    self.lclk = 0;
                } else {
                    self.lclk += 1;
                }
            },
            PPUState::VBlank => {
                if self.lclk == 113 {
                    if ly == 153 {
                        self.state = PPUState::OAMSearch;
                        self.reg_set(PPUReg::Ly, 0);
                        ly = 0;
                    } else {
                        ly += 1;
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

        self.reg_set(PPUReg::Ly, ly);

        self.push_registers();
    }

    // Start and stop are not public, they must be activated by LCDC.
    fn start(&mut self) {
        self.state = PPUState::OAMSearch;
        self.lclk = 0;
        self.reg_set(PPUReg::Ly, 0);
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
        let lcdc_on = self.reg_field_get_bool(PPURegField::LcdcEnable);
        if lcdc_on && !self.is_rendering() {
            self.start();
        } else if !lcdc_on && self.is_rendering() {
            self.stop();
        }
    }

    // Check for register changes, and apply the corresponding settings differences.
    fn pull_registers(&mut self) {
        // Collect the values before writing to prevent borrowing issues.
        let reg_vals: Vec<(PPUReg, u8)> = self.cfg.cache.keys().map(|&r| (r.clone(), self.mem_get(r as u16))).collect();
        for (reg, val) in reg_vals {
            self.reg_set(reg, val);
            self.cfg.dirty.insert(reg, false);
        }
    }

    // Flush register changes to memory
    fn push_registers(&mut self) {
        let dirty_regs: Vec<(&PPUReg, &u8)> = self.cfg.cache.iter().filter(|&(r, _d)| self.cfg.dirty.get(r) == Some(&true)).collect();
        let dirty_regs: Vec<(PPUReg, u8)> = dirty_regs.iter().map(|&(r, d)| (r.clone(), d.clone())).collect();
        for (reg, val) in dirty_regs {
            self.mem_set(reg as u16, val);
        }
    }

    fn reg_field_parent(&self, field: PPURegField) -> PPUReg {
        match field {
            PPURegField::LcdcEnable        => PPUReg::Lcdc,
            PPURegField::LcdcWinMapPos     => PPUReg::Lcdc,
            PPURegField::LcdcWinEnableLE   => PPUReg::Lcdc,
            PPURegField::LcdcBgWinDataPos  => PPUReg::Lcdc,
            PPURegField::LcdcBgMapPos      => PPUReg::Lcdc,
            PPURegField::LcdcObjSize       => PPUReg::Lcdc,
            PPURegField::LcdcObjEnable     => PPUReg::Lcdc,
            PPURegField::LcdcObjWinPrio    => PPUReg::Lcdc,
            PPURegField::StatCoincInt      => PPUReg::Stat,
            PPURegField::StatOamInt        => PPUReg::Stat,
            PPURegField::StatVblankInt     => PPUReg::Stat,
            PPURegField::StatHblankInt     => PPUReg::Stat,
            PPURegField::StatCoincidence   => PPUReg::Stat,
            PPURegField::StatMode          => PPUReg::Stat,
            PPURegField::BgpShadeColor3    => PPUReg::Bgp,
            PPURegField::BgpShadeColor2    => PPUReg::Bgp,
            PPURegField::BgpShadeColor1    => PPUReg::Bgp,
            PPURegField::BgpShadeColor0    => PPUReg::Bgp,
        }
    }

    fn reg_field_offset_size(&self, field: PPURegField) -> (u8, u8) {
        match field {
            PPURegField::LcdcEnable         => (7, 1),
            PPURegField::LcdcWinMapPos      => (6, 1),
            PPURegField::LcdcWinEnableLE    => (5, 1),
            PPURegField::LcdcBgWinDataPos   => (4, 1),
            PPURegField::LcdcBgMapPos       => (3, 1),
            PPURegField::LcdcObjSize        => (2, 1),
            PPURegField::LcdcObjEnable      => (1, 1),
            PPURegField::LcdcObjWinPrio     => (0, 1),
            PPURegField::StatCoincInt       => (6, 1),
            PPURegField::StatOamInt         => (5, 1),
            PPURegField::StatVblankInt      => (4, 1),
            PPURegField::StatHblankInt      => (3, 1),
            PPURegField::StatCoincidence    => (2, 1),
            PPURegField::StatMode           => (0, 2),
            PPURegField::BgpShadeColor3     => (6, 2),
            PPURegField::BgpShadeColor2     => (4, 2),
            PPURegField::BgpShadeColor1     => (2, 2),
            PPURegField::BgpShadeColor0     => (0, 2),
        }
    }

    fn reg_get(&self, reg: PPUReg) -> u8 {
        let val = match self.cfg.cache.get(&reg) {
            Some(val) => val.clone(),
            None      => {
                panic!("No config register cache entry writable for this PPU register!");
            }
        };

        val
    }

    fn reg_set(&mut self, reg: PPUReg, val: u8) {
        let old_val = match self.cfg.cache.insert(reg, val) {
            Some(v) => v.clone(),
            None    => {
                panic!("Config register cache entry didn't exist for this PPU register!");
            }
        };

        self.cfg.dirty.insert(reg, old_val != val);
    }

    fn reg_field_get(&self, field: PPURegField) -> u8 {
        let parent = self.reg_field_parent(field);
        let base = self.reg_get(parent);
        let (offset, size) = self.reg_field_offset_size(field);
        let mask = (1 << size+1) - 1;
        (base >> offset) & mask
    }

    // I'd like to use generics for this, but it gave me trouble...
    // From<u8> doesn't exist for bool, and compiler wont let me add it myself.
    fn reg_field_get_bool(&self, field: PPURegField) -> bool {
        self.reg_field_get(field) != 0
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
