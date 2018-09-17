// PPU abstracts the details of the PPU from the CPU. It's different from the Window struct because
// the window abstracts platform-specific details related to operating the window.

use window::Window;

#[derive(Copy, Clone, PartialEq)]
enum PPUState {
    Off,
    HBlank,
    VBlank,
    OAMSearch,
    Draw
}

pub struct PPU {
    state: PPUState,
    line: u32,       // The line we're currently on.
    lclk: u32,       // The machine cycle for this line, from [0, 113].
    width: u32,      // Width of the virtual window, fixed at 160.
    height: u32,     // Height of the virtual window, fixed at 144.
    win: Window
}

impl PPU {
    pub fn new() -> Self {
        let (w, h) = (160, 144);
        let win = Window::new(w, h);
        PPU {
            state: PPUState::Off,
            line: 0,
            lclk: 0,
            width: w,
            height: h,
            win: win
        }
    }

    // Tick operates on a machine clock cycle, and does the appropriate action for this cycle.
    pub fn tick(&mut self) {
        match self.state {
            PPUState::Off => (),
            PPUState::HBlank => {},
            PPUState::VBlank => {},
            PPUState::OAMSearch => {},
            PPUState::Draw => {}
        }

        self.lclk += 1;
    }

    pub fn start(&mut self) {
        self.state = PPUState::OAMSearch;
        self.lclk = 0;
        self.line = 0;
    }

    pub fn render(&mut self) {
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

        if self.is_running() {
            self.win.get_events();
            if self.win.is_open() {
                // Set LY = 0
                self.win.draw(&pixels);
            } else {
                self.stop();
            }
        }
    }

    pub fn stop(&mut self) {
        self.state = PPUState::Off;
    }

    pub fn is_running(&self) -> bool {
        self.state == PPUState::Off
    }
}
