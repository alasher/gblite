use sdl2;
use sdl2::video;

pub struct Window {
    sdl: sdl2::Sdl,
    video: sdl2::VideoSubsystem,
    window: Option<video::Window>
}

impl Window {
    pub fn new() -> Self {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        Window {
            sdl: sdl,
            video: video,
            window: None
        }
    }

    pub fn open(&mut self) {
        self.window = Some(self.video.window("gblite", 160, 144).resizable().build().unwrap());
    }
}
