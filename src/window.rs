use sdl2;
use sdl2::video;
use gl;
use std;

pub struct Window {
    sdl: sdl2::Sdl,
    video: sdl2::VideoSubsystem,
    window: Option<video::Window>,
    context: Option<video::GLContext>
}

impl Window {
    pub fn new() -> Self {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        Window {
            sdl: sdl,
            video: video,
            window: None,
            context: None
        }
    }

    pub fn open(&mut self) {
        let win = self.video.window("gblite", 160, 144).
                           resizable().
                           opengl().
                           build().
                           unwrap();
        let ctx = win.gl_create_context().unwrap();

        gl::load_with(|s| self.video.gl_get_proc_address(s) as *const std::os::raw::c_void);

        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
        }

        self.window = Some(win);
        self.context = Some(ctx);
    }

    pub fn draw(&mut self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        match &self.window {
            Some(win) => win.gl_swap_window(),
            None => panic!("No window to draw in!")
        }
    }
}
