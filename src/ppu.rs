// PPU abstracts the details of the PPU from the CPU. It's different from the Window struct because
// the window abstracts platform-specific details related to operating the window.

use window::Window;
use memory::Memory;
use memory::MemClient;

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
        ppu.reg_set(PPUReg::LCDC, 0x91);
        ppu.reg_set(PPUReg::BGP, 0xFC);
        ppu.reg_set(PPUReg::OBP0, 0xFF);
        ppu.reg_set(PPUReg::OBP1, 0xFF);
        ppu.pull_registers();

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

        // Check window events and for register changes
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

        self.pull_registers();
    }

    // Check for register changes, and apply the corresponding settings differences.
    fn pull_registers(&mut self) {

        // Check LCDC for status changes.
        let lcdc = self.reg_get(PPUReg::LCDC);
        let lcdc_on = (lcdc & 0x80) != 0;
        if lcdc_on && !self.is_rendering() {
            self.start();
        } else if !lcdc_on && self.is_rendering() {
            self.stop();
        }
        self.cfg.win_map_off = if (lcdc & 0x40) != 0 { 0x9800 } else { 0x9C00 };
        self.cfg.bgr_dat_off = if (lcdc & 0x10) != 0 { 0x8800 } else { 0x8000 };
        self.cfg.bgr_map_off = if (lcdc & 0x08) != 0 { 0x9800 } else { 0x9C00 };
        self.cfg.win_en = (lcdc & 0x20) != 0;
        self.cfg.obj_en = (lcdc & 0x02) != 0;
        self.cfg.bgr_en = (lcdc & 0x01) != 0;
        self.cfg.tall_objs = (lcdc & 0x04) != 0;

    }

    fn reg_get(&self, reg: PPUReg) -> u8 {
        let val = self.mem_get(reg as u16);

        val
    }

    fn reg_set(&mut self, reg: PPUReg, val: u8) {
        self.mem_set(reg as u16, val);
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
