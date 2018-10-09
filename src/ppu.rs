// PPU abstracts the details of the PPU from the CPU. It's different from the Window struct because
// the window abstracts platform-specific details related to operating the window.

use window::Window;
use memory::Memory;
use memory::MemClient;

use std::sync::Arc;
use std::sync::Mutex;

#[derive(Copy, Clone, PartialEq)]
enum PPUState {
    Quit,        // Quit is a signal from the OS window indicating to terminate gblite.
    Off,         // Off keeps the application open, but leaves the LCD inactive.
    HBlank,      // HBlank is the LCD idle period after each line is drawn.
    VBlank,      // VBlank is the LCD idle period after the final line is drawn.
    OAMSearch,   // OAM Search is the initial linear scan of objects on a given line.
    Draw         // Draw is the lookup and transfer period of pixels to the LCD.
}

#[derive(Copy, Clone, PartialEq)]
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

pub struct PPU {
    lcd: Window,             // The actual graphics window, not to be confused with a Game Boy window map/tile.
    state: PPUState,         // Current PPU state, non-off is STAT[0:1], OFF is controlled by LCDC bit 7.
    mem: Arc<Mutex<Memory>>, // Reference to our Memory object.
    width: u32,              // Width of the virtual window, fixed at 160.
    height: u32,             // Height of the virtual window, fixed at 144.
    lclk: u32,               // The machine cycle for this line, from [0, 113].
    bgr_map_off: u16,        // Offset to BG Map start address in VRAM, adjustble by LCDC bit 3.
    win_map_off: u16,        // Offset to Window map start address in VRAM, adjustable by LCDC bit 6.
    bgr_dat_off: u16         // Offset to BG/Window data start address in VRAM, adjustable by LCDC bit 4.
}

impl PPU {
    pub fn new(mem: Arc<Mutex<Memory>>) -> Self {
        let (w, h) = (160, 144);
        let lcd = Window::new(w, h);

        let mut ppu = PPU {
            lcd: lcd,
            state: PPUState::OAMSearch,
            mem: mem,
            width: w,
            height: h,
            lclk: 0,
            bgr_map_off: 0,
            win_map_off: 0,
            bgr_dat_off: 0
        };

        // Initialize PPU config registesr
        ppu.reg_set(PPUReg::LCDC, 0x91);
        ppu.reg_set(PPUReg::BGP, 0xFC);
        ppu.reg_set(PPUReg::OBP0, 0xFF);
        ppu.reg_set(PPUReg::OBP1, 0xFF);

        ppu
    }

    // Tick performs the appropriate PPU action for this machine cycle.
    // TODO: Adjust cycle accuracy of Draw state, timings can vary slightly.
    pub fn tick(&mut self) {

        // Check window events, close if necessary
        if self.state == PPUState::Quit {
            return;
        } else {
            self.lcd.get_events();
            if !self.lcd.is_open() {
                self.terminate();
                return;
            }
        }

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

    pub fn start(&mut self) {
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
            let pcolor = (w as f32 * 255f32 / self.width as f32) as u8;
            for h in 0..self.height {
                pixels.push(pcolor);
                pixels.push(pcolor);
                pixels.push(pcolor);
            }
        }

        self.lcd.draw(&pixels);
    }

    pub fn stop(&mut self) {
        self.state = PPUState::Off;
    }

    pub fn terminate(&mut self) {
        self.state = PPUState::Quit;
    }

    pub fn is_alive(&self) -> bool {
        self.state != PPUState::Quit
    }

    fn check_events(&mut self) {
        // Check window for termination events
        self.lcd.get_events();
        if !self.lcd.is_open() {
            self.terminate();
        }

        // Check for register changes
    }

    fn reg_get(&self, reg: PPUReg) -> u8 {
        self.mem_get(reg as u16)
    }

    fn reg_set(&mut self, reg: PPUReg, val: u8) {
        self.mem_set(reg as u16, val);
    }

    // VRAM data access, given absolute memory address
    // VRAM [0x8000, 0xa000) -> [0x0, 0x2000]
    // OAM RAM access [0xFE00, 0xFEA0) -> []
    fn mem_get(&self, addr: u16) -> u8 {
        let mut mref = self.mem.lock().unwrap();
        (*mref).get(addr, MemClient::PPU)
    }

    fn mem_set(&mut self, addr: u16, val: u8) {
        let mut mref = self.mem.lock().unwrap();
        (*mref).set(val, addr, MemClient::PPU)
    }
}
