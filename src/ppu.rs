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
    LCDC = 0xFF40,
    STAT = 0xFF41,
    SCY  = 0xFF42,
    SCX  = 0xFF43,
    LY   = 0xFF44,
    LYC  = 0xFF45,
    DMA  = 0xFF46,
    BGP  = 0xFF47,
    OBP0 = 0xFF48,
    OBP1 = 0xFF49,
    WY   = 0xFF4A,
    WX   = 0xFF4B,
    VBK  = 0xFF4F
}

enum PPURegField {
    LCDC_ENABLE,
    LCDC_WIN_MAP_POS,
    LCDC_WIN_ENABLE,
    LCDC_BG_WIN_DATA_POS,
    LCDC_BG_MAP_POS,
    LCDC_OBJ_SIZE,
    LCDC_OBJ_ENABLE,
    LCDC_OBJ_WIN_PRIO,
    STAT_COINC_INT,
    STAT_OAM_INT,
    STAT_VBLANK_INT,
    STAT_HBLANK_INT,
    STAT_COINCIDENCE,
    STAT_MODE,
    BGP_SHADE_COLOR3,
    BGP_SHADE_COLOR2,
    BGP_SHADE_COLOR1,
    BGP_SHADE_COLOR0,
}

impl Display for PPUReg {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            PPUReg::LCDC => write!(f, "LCDC"),
            PPUReg::STAT => write!(f, "STAT"),
            PPUReg::SCY  => write!(f, "SCY"),
            PPUReg::SCX  => write!(f, "SCX"),
            PPUReg::LY   => write!(f, "LY"),
            PPUReg::LYC  => write!(f, "LYC"),
            PPUReg::DMA  => write!(f, "DMA"),
            PPUReg::BGP  => write!(f, "BGP"),
            PPUReg::OBP0 => write!(f, "OBP0"),
            PPUReg::OBP1 => write!(f, "OBP1"),
            PPUReg::WY   => write!(f, "WY"),
            PPUReg::WX   => write!(f, "WX"),
            PPUReg::VBK  => write!(f, "VBK"),
            _            => write!(f, "UNKNOWN"),
        }
    }
}

struct PPUConfig {
    cache: HashMap<PPUReg, u8>,
    dirty: HashMap<PPUReg, bool>,
    bgr_map_off: u16,        // Offset to BG Map start address in VRAM, adjustble by LCDC bit 3.
    win_map_off: u16,        // Offset to Window map start address in VRAM, adjustable by LCDC bit 6.
    bgr_dat_off: u16,        // Offset to BG/Window data start address in VRAM, adjustable by LCDC bit 4.
    win_en: bool,            // True if window is enabled. Note this window is drawn, it's not about the SDL window.
    obj_en: bool,            // True if sprites (or OBJs) are enabled.
    bgr_en: bool,            // True if background rendering enabled. Always enabled on CGB.
    tall_objs: bool          // If false, an 8x8 OBJ is used. Otherwise, an 8x16 OBJ is used.
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
            (PPUReg::LCDC, 0),
            (PPUReg::STAT, 0),
            (PPUReg::SCY,  0),
            (PPUReg::SCX,  0),
            (PPUReg::LY,   0),
            (PPUReg::LYC,  0),
            (PPUReg::DMA,  0),
            (PPUReg::BGP,  0),
            (PPUReg::OBP0, 0),
            (PPUReg::OBP1, 0),
            (PPUReg::WY,   0),
            (PPUReg::WX,   0),
            (PPUReg::VBK,  0),
        ].iter().cloned().collect();

        let dirty: HashMap<PPUReg, bool> = [
            (PPUReg::LCDC, true),
            (PPUReg::STAT, true),
            (PPUReg::SCY,  true),
            (PPUReg::SCX,  true),
            (PPUReg::LY,   true),
            (PPUReg::LYC,  true),
            (PPUReg::DMA,  true),
            (PPUReg::BGP,  true),
            (PPUReg::OBP0, true),
            (PPUReg::OBP1, true),
            (PPUReg::WY,   true),
            (PPUReg::WX,   true),
            (PPUReg::VBK,  true),
        ].iter().cloned().collect();

        let cfg = PPUConfig {
            cache: cache,
            dirty: dirty,
            bgr_map_off: 0,
            win_map_off: 0,
            bgr_dat_off: 0,
            win_en: false,
            obj_en: false,
            bgr_en: false,
            tall_objs: false,
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
        ppu.reg_set(PPUReg::LCDC, 0x91);
        ppu.reg_set(PPUReg::BGP, 0xFC);
        ppu.reg_set(PPUReg::OBP0, 0xFF);
        ppu.reg_set(PPUReg::OBP1, 0xFF);
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

        let ly = self.reg_get(PPUReg::LY);

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
                    self.reg_set(PPUReg::LY, ly+1);
                    self.lclk = 0;
                } else {
                    self.lclk += 1;
                }
            },
            PPUState::VBlank => {
                if self.lclk == 113 {
                    if ly == 153 {
                        self.state = PPUState::OAMSearch;
                        self.reg_set(PPUReg::LY, 0);
                    } else {
                        self.reg_set(PPUReg::LY, ly+1);
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
        self.reg_set(PPUReg::LY, 0);
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

        if (self.dbg.enabled) {
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
        let lcdc = self.reg_get(PPUReg::LCDC);
        let lcdc_on = (lcdc & 0x80) != 0;
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
        let dirty_regs: Vec<(&PPUReg, &u8)> = self.cfg.cache.iter().filter(|&(r, d)| self.cfg.dirty.get(r) == Some(&true)).collect();
        let dirty_regs: Vec<(PPUReg, u8)> = dirty_regs.iter().map(|&(r, d)| (r.clone(), d.clone())).collect();
        for (reg, val) in dirty_regs {
            self.mem_set(reg as u16, val);
        }
    }

    fn reg_field_parent(&self, field: PPURegField) -> Option<PPUReg> {
        match (field) {
            PPURegField::LCDC_ENABLE            => Some(PPUReg::LCDC),
            PPURegField::LCDC_WIN_MAP_POS       => Some(PPUReg::LCDC),
            PPURegField::LCDC_WIN_ENABLE        => Some(PPUReg::LCDC),
            PPURegField::LCDC_BG_WIN_DATA_POS   => Some(PPUReg::LCDC),
            PPURegField::LCDC_BG_MAP_POS        => Some(PPUReg::LCDC),
            PPURegField::LCDC_OBJ_SIZE          => Some(PPUReg::LCDC),
            PPURegField::LCDC_OBJ_ENABLE        => Some(PPUReg::LCDC),
            PPURegField::LCDC_OBJ_WIN_PRIO      => Some(PPUReg::LCDC),
            PPURegField::STAT_COINC_INT         => Some(PPUReg::STAT),
            PPURegField::STAT_OAM_INT           => Some(PPUReg::STAT),
            PPURegField::STAT_VBLANK_INT        => Some(PPUReg::STAT),
            PPURegField::STAT_HBLANK_INT        => Some(PPUReg::STAT),
            PPURegField::STAT_COINCIDENCE       => Some(PPUReg::STAT),
            PPURegField::STAT_MODE              => Some(PPUReg::STAT),
            PPURegField::BGP_SHADE_COLOR3       => Some(PPUReg::BGP),
            PPURegField::BGP_SHADE_COLOR2       => Some(PPUReg::BGP),
            PPURegField::BGP_SHADE_COLOR1       => Some(PPUReg::BGP),
            PPURegField::BGP_SHADE_COLOR0       => Some(PPUReg::BGP),
            _ => None,
        }
    }

    fn reg_field_offset_size(&self, field: PPURegField) -> (u8, u8) {
        match (field) {
            PPURegField::LCDC_ENABLE            => (7, 1),
            PPURegField::LCDC_WIN_MAP_POS       => (6, 1),
            PPURegField::LCDC_WIN_ENABLE        => (5, 1),
            PPURegField::LCDC_BG_WIN_DATA_POS   => (4, 1),
            PPURegField::LCDC_BG_MAP_POS        => (3, 1),
            PPURegField::LCDC_OBJ_SIZE          => (2, 1),
            PPURegField::LCDC_OBJ_ENABLE        => (1, 1),
            PPURegField::LCDC_OBJ_WIN_PRIO      => (0, 1),
            PPURegField::STAT_COINC_INT         => (6, 1),
            PPURegField::STAT_OAM_INT           => (5, 1),
            PPURegField::STAT_VBLANK_INT        => (4, 1),
            PPURegField::STAT_HBLANK_INT        => (3, 1),
            PPURegField::STAT_COINCIDENCE       => (2, 1),
            PPURegField::STAT_MODE              => (0, 2),
            PPURegField::BGP_SHADE_COLOR3       => (6, 2),
            PPURegField::BGP_SHADE_COLOR2       => (4, 2),
            PPURegField::BGP_SHADE_COLOR1       => (2, 2),
            PPURegField::BGP_SHADE_COLOR0       => (0, 2),
            _ => (0, 8),
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

        self.cfg.dirty.insert(reg, (old_val != val));
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
