use crate::enums::{BlendFunc, BufferBit, GlEnable, GlError};
use crate::opengl::gl::Gl;
use crate::opengl;

use glfw::Glfw;
use glfw::{Context, WindowEvent};
use glfw::{PWindow, GlfwReceiver};
use glfw::fail_on_errors;

use crate::enums::ContextError;

pub struct Window {
    pub glfw : Glfw,
    pub window : PWindow,
    pub events : GlfwReceiver<(f64, WindowEvent)>,
    pub opengl : Gl,
    pub last_cursor_pos : [f32; 2],
    pub aspect_ratio : f32,
    pub background_colour : (f32, f32, f32),
}

impl Window {
    pub fn new_opengl() -> Result<Window, ContextError> {
        let (width, height) = (450, 450);
        //let (width, height) = (1920, 1080);
        let window_name = "hello, window!";

        match glfw::init(fail_on_errors!()) {
            Ok(mut glfw) => {
                match glfw.create_window(
                            width, height,
                            &window_name,
                            glfw::WindowMode::Windowed
                        ) {
                    Some((mut window, events)) => {
                        let opengl = opengl::intermediate_opengl::load_opengl_with(
                                                                            get_glfw_loadfn(&mut window)
                                                                        );
                        Ok(
                            Window {
                                glfw,
                                window,
                                events,
                                opengl,
                                last_cursor_pos:[0.0, 0.0],
                                aspect_ratio:width as f32 / height as f32, 
                                background_colour:(0.5, 0.5, 0.5),
                            }
                        )
                    },
                    None => Err(ContextError::GLFWNoWindowCreated),
                }
            },
            Err(err) => Err(ContextError::GLFWinitError(err)),
        }
    }

    // relabel subaspect functions to Window functions        
    pub fn poll_events(&mut self)  { self.glfw.poll_events(); }
    pub fn get_time_since_glfw_init(&self) -> f64 { self.glfw.get_time()}

    pub fn set_polling(&mut self)  { self.window.set_all_polling(true); }
    pub fn swap_buffers(&mut self) { self.window.swap_buffers(); }
    pub fn make_current(&mut self) { self.window.make_current(); }
    pub fn width(&self) -> i32 { self.window.get_size().0 }
    pub fn height(&self) -> i32 { self.window.get_size().1 }

    pub fn clear(&self, masks:Vec<BufferBit>) { opengl::intermediate_opengl::clear(&self.opengl, masks) }
    pub fn clear_to_colour(&self, rgb:(f32, f32, f32), a:f32) -> Result<(), GlError> {
        opengl::intermediate_opengl::clear_colour(&self.opengl, rgb.0, rgb.1, rgb.2, a)
    }    

    pub fn default_gl_settings(&self) {
        opengl::intermediate_opengl::gl_enable(&self.opengl, GlEnable::DepthTest);
        opengl::intermediate_opengl::gl_enable(&self.opengl, GlEnable::Multisample);
        opengl::intermediate_opengl::gl_enable(&self.opengl, GlEnable::Blend);
        opengl::intermediate_opengl::gl_blendfunc(&self.opengl, BlendFunc::SRCAlphaOneMinusSRCAlpha);
    }
}

fn get_glfw_loadfn<T>(window:&mut PWindow)
                -> impl FnMut(&'static str) -> *const T {
    |window_name: &'static str| window.get_proc_address(&window_name).unwrap() as *const _
}