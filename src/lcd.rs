// LCD abstracts the details of the PPU from the CPU. It's different from the Window struct because
// the window abstracts platform-specific details related to operating the window.

use window::Window;

pub struct LCD {
    running: bool,
    win: Window
}

impl LCD {
    pub fn new() -> Self {
        LCD {
            running: false,
            win: Window::new()
        }
    }

    pub fn start(&mut self) {
        // Open our window class
        self.running = true;
        self.win.open();
    }

    pub fn stop(&mut self) {
        self.running = false;
    }
}
