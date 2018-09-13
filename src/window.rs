use sdl2;
use sdl2::video;
use sdl2::render;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Window {
    sdl: sdl2::Sdl,
    canvas: render::Canvas<video::Window>,
    open: bool
}

impl Window {
    pub fn new(w: u32, h: u32) -> Self {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let win = video.window("gblite", w, h).
                           resizable().
                           build().
                           unwrap();

        let mut can = win.into_canvas().build().unwrap();
        can.set_draw_color(Color::RGB(0, 255, 255));

        Window {
            sdl: sdl,
            canvas: can,
            open: true
        }
    }

    pub fn draw(&mut self, pixels: &[u8]) {
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn get_events(&mut self) {
        let mut events = self.sdl.event_pump().unwrap();
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.close();
                },
                _ => ()
            }
        }
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn close(&mut self) {
        self.open = false;
    }
}
