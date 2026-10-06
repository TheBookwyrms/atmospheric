use crate::enums::{BlendFunc, BufferBit, ContextError, GlEnable, GlError, InternalFormat};
use crate::opengl::gl::Gl;
use crate::opengl::{self, intermediate_opengl};

use glfw::Glfw;
use glfw::{Context, WindowEvent};
use glfw::{PWindow, GlfwReceiver};
use glfw::fail_on_errors;


pub struct Window {
    glfw : Glfw,
    window : PWindow,
    events : GlfwReceiver<(f64, WindowEvent)>,
    pub opengl : Gl,
    last_cursor_pos : [f32; 2],
    aspect_ratio : f32,
    background_colour : [f32; 3],
}

impl Window {
    pub fn new_opengl(window_name:&'static str, width:u32, height:u32) -> Result<Window, ContextError> {
        //let (width, height) = (450, 450);
        //let (width, height) = (1920, 1080);
        //let window_name = "hello, window!";

        match glfw::init(fail_on_errors!()) {
            Ok(mut glfw) => {
                match glfw.create_window(width, height, window_name, glfw::WindowMode::Windowed) {
                    Some((mut window, events)) => {
                        let opengl = opengl::intermediate_opengl::load_opengl_with(get_glfw_loadfn(&mut window));
                        Ok(
                            Window {
                                glfw,
                                window,
                                events,
                                opengl,
                                last_cursor_pos:[0.0, 0.0],
                                aspect_ratio:width as f32 / height as f32, 
                                background_colour:[0.5, 0.5, 0.5],
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

    pub fn set_polling(&mut self)  { self.window.set_all_polling(true) }
    pub fn swap_buffers(&mut self) { self.window.swap_buffers() }
    pub fn make_current(&mut self) { self.window.make_current() }
    pub fn set_should_close(&mut self, value:bool) { self.window.set_should_close(value) }
    pub fn should_close(&self) -> bool { self.window.should_close() }
    pub fn iconify(&mut self) { self.window.iconify() }

    pub fn width(&self) -> i32 { self.window.get_size().0 }
    pub fn height(&self) -> i32 { self.window.get_size().1 }
    /// gets (width, height) of window, converted to usize
    pub fn wh_usize(&self) -> (usize, usize) { let wh = self.window.get_size(); (wh.0 as usize, wh.1 as usize) }

    pub fn clear(&self, masks:Vec<BufferBit>) { opengl::intermediate_opengl::clear(&self.opengl, masks) }
    pub fn clear_to_colour(&self, rgb:[f32; 3], a:f32) -> Result<(), GlError> {
        opengl::intermediate_opengl::clear_colour(&self.opengl, rgb[0], rgb[1], rgb[2], a)
    }    

    pub fn default_gl_settings(&self) {
        opengl::intermediate_opengl::gl_enable(&self.opengl, GlEnable::DepthTest);
        opengl::intermediate_opengl::gl_enable(&self.opengl, GlEnable::Multisample);
        opengl::intermediate_opengl::gl_enable(&self.opengl, GlEnable::Blend);
        opengl::intermediate_opengl::gl_blendfunc(&self.opengl, BlendFunc::SRCAlphaOneMinusSRCAlpha);
    }

    pub fn read_pixels_full_window(&self, opengl:&Gl, colour_format:InternalFormat) -> Vec<u8> {
        intermediate_opengl::read_pixels(opengl, 0, 0, self.width(), self.height(), colour_format)
    }

    pub fn get_cursor_pos(&self) -> [f32; 2] {
        let pos = self.window.get_cursor_pos();
        [pos.0 as f32, pos.1 as f32]
    }
    pub fn get_last_cursor_pos(&self) -> [f32; 2] { self.last_cursor_pos }
    pub fn set_last_cursor_pos(&mut self, last:[f32; 2]) { self.last_cursor_pos = last }
    pub fn get_aspect_ratio(&self) -> f32 { self.aspect_ratio }
    pub fn set_aspect_ratio(&mut self, ratio:f32) { self.aspect_ratio = ratio }
    pub fn get_background_colour(&self) -> [f32; 3] { self.background_colour }
    pub fn set_background_colour(&mut self, colour:[f32; 3]) { self.background_colour = colour }
    pub fn get_opengl_handle(&self) -> &Gl { &self.opengl }

    pub fn flush_messages(&self) -> Vec<WindowEvent> {
        glfw::flush_messages(&self.events).map(|(_, e)| e).collect::<Vec<WindowEvent>>()

    }
}

fn get_glfw_loadfn<T>(window:&mut PWindow)
                -> impl FnMut(&'static str) -> *const T {
    |window_name: &'static str| window.get_proc_address(&window_name).unwrap() as *const _
}