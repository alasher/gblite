use sdl2;
use sdl2::video;
use sdl2::render;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;

pub struct Window {
    sdl: sdl2::Sdl,
    canvas: render::Canvas<video::Window>,
    width: u32,
    height: u32,
    event_cnt: u32,
    open: bool,
}

impl Window {
    pub fn new(w: usize, h: usize) -> Self {
        let (wi, hi) = (w as u32, h as u32);
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let win = video.window("gblite", wi, hi)
                       .resizable()
                       .build()
                       .unwrap();

        let mut can = win.into_canvas().build().unwrap();
        can.set_draw_color(Color::RGB(0, 255, 255));

        Window {
            sdl: sdl,
            canvas: can,
            width: wi,
            height: hi,
            event_cnt: 0,
            open: true,
        }
    }

    pub fn draw(&mut self, pixels: &[u8]) {
        let tex_creator = self.canvas.texture_creator();
        let mut tex = tex_creator.create_texture_streaming(
            PixelFormatEnum::RGB24, self.width, self.height).unwrap();
        tex.update(None, &pixels, 3 * self.width as usize).unwrap();

        self.canvas.clear();
        self.canvas.copy(&tex, None, None).unwrap();
        self.canvas.present();
    }

    // TODO: Move this to another thread. Maybe the entire window could be run in a binary package
    // on a separate thread? It could set up channels to communicate with the PPU/CPU.
    pub fn get_events(&mut self) {
        self.event_cnt += 1;
        if self.event_cnt < 250 {
            return;
        } else {
            self.event_cnt = 0;
        }

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
