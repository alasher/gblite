// LCD abstracts the details of the PPU from the CPU. It's different from the Window struct because
// the window abstracts platform-specific details related to operating the window.

use window::Window;

pub struct LCD {
    pub running: bool, // TODO: Right now "running" corresponds to the program status, but on a real game boy
                       //       the LCD can be disabled while the CPU is active.
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

    pub fn render(&mut self) {
        if self.running {
            self.win.get_events();
            if self.win.is_open() {
                // Set LY = 0
                self.win.draw();
            } else {
                self.stop();
            }
        }
    }

    pub fn stop(&mut self) {
        self.running = false;
    }
}
