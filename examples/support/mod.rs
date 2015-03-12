extern crate gl;
extern crate sdl2;
extern crate log;

use super::std::mem;

use self::sdl2::video;
use self::sdl2::video::GLAttr;

pub enum GLVersion {
    Core((i32, i32)),
}

pub struct WindowOptions {
    pub gl_version: GLVersion,
    pub title: String,
    pub initial_size: (i32, i32),
}

pub struct Window {
    sdl_context: self::sdl2::sdl::Sdl,
    sdl_window: self::sdl2::video::Window,
    _sdl_gl_context: self::sdl2::video::GLContext,
    pub size: (i32, i32),
}

impl Window {
    pub fn new(options: WindowOptions) -> Window {
        let sdl_context = sdl2::init(sdl2::INIT_VIDEO).unwrap();

        match options.gl_version {
            GLVersion::Core((major, minor)) => {
                video::gl_set_attribute(GLAttr::GLContextProfileMask, video::GLProfile::GLCoreProfile as i32);
                video::gl_set_attribute(GLAttr::GLContextMajorVersion, major);
                video::gl_set_attribute(GLAttr::GLContextMinorVersion, minor);
            }
        }

        video::gl_set_attribute(GLAttr::GLAcceleratedVisual, 1);
        video::gl_set_attribute(GLAttr::GLDoubleBuffer, 1);

        let (window_width, window_height) = options.initial_size;

        let window = match video::Window::new(
            &options.title,
            video::WindowPos::PosCentered,
            video::WindowPos::PosCentered,
            window_width,
            window_height,
            video::RESIZABLE | video::OPENGL
        ) {
            Ok(window) => window,
            Err(err) => panic!("failed to create window: {}", err),
        };

        window.set_bordered(true);

        let gl_context = match window.gl_create_context() {
            Err(err) => panic!("failed to create GL context: {}", err),
            Ok(gl_context) => {
                gl::load_with(|s| unsafe {
                    mem::transmute(sdl2::video::gl_get_proc_address(s))
                });

                gl_context
            },
        };

        Window {
            sdl_context: sdl_context,
            sdl_window: window,
            _sdl_gl_context: gl_context,
            size: options.initial_size,
        }
    }

    pub fn run<F: FnMut()>(&mut self, mut render: F) {
        let mut event_pump = self.sdl_context.event_pump();
        let mut is_closed = false;

        while !is_closed {
            for event in event_pump.poll_iter() {
                use self::sdl2::event::Event;
                use self::sdl2::event::WindowEventId;

                match event {
                    Event::Quit {..} => { is_closed = true; },
                    Event::Window { win_event_id: WindowEventId::Resized, data1: w, data2: h, .. } => {
                        self.size = (w, h);
                    },
                    _ => ()
                }
            }

            render();

            self.sdl_window.gl_swap_window();
        }
    }
}
