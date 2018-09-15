// PPU abstracts the details of the PPU from the CPU. It's different from the Window struct because
// the window abstracts platform-specific details related to operating the window.

use window::Window;

pub struct PPU {
    pub running: bool, // TODO: Right now "running" corresponds to the program status, but on a real game boy
                       //       the PPU can be disabled while the CPU is active.
    width: u32,
    height: u32,
    win: Window
}

impl PPU {
    pub fn new() -> Self {
        let (w, h) = (160, 144);
        let win = Window::new(w, h);
        PPU {
            running: true,
            width: w,
            height: h,
            win: win
        }
    }

    pub fn start(&mut self) {
        self.running = true;
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

        if self.running {
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
        self.running = false;
    }
}
