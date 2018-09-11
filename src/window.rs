use sdl2;
use sdl2::video;
use sdl2::render;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Window {
    sdl: sdl2::Sdl,
    video: sdl2::VideoSubsystem,
    canvas: Option<render::Canvas<video::Window>>,
}

impl Window {
    pub fn new() -> Self {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        Window {
            sdl: sdl,
            video: video,
            canvas: None
        }
    }

    pub fn open(&mut self) {
        let win = self.video.window("gblite", 160, 144).
                           resizable().
                           build().
                           unwrap();
        let mut can = win.into_canvas().build().unwrap();

        can.set_draw_color(Color::RGB(0, 255, 255));

        self.canvas = Some(can);
    }

    pub fn draw(&mut self) {
        if let Some(c) = self.canvas.as_mut() {
            (*c).clear();
            (*c).present();
        }
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
        self.canvas.is_some()
    }

    pub fn close(&mut self) {
        self.canvas = None;
    }
}
