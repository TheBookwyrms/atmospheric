use crate::camera::Camera;
use crate::context::AssortedContextDetails;
use crate::glfw::window::Window;
use crate::enums::ContextError;
use crate::modules::keybindings::KeybindingModule;
use crate::opengl::intermediate_opengl;
use glfw::{WindowEvent, Action, Key};

pub struct DefaultWindowKeybindings {}
impl KeybindingModule for DefaultWindowKeybindings {
    fn call_keybindings(&self, event:&WindowEvent, window:&mut Window, _:&mut Camera, _:&mut AssortedContextDetails) -> Result<(), ContextError> {
        match event {
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.window.set_should_close(true)
            },
            WindowEvent::Close => {
                window.window.set_should_close(true)
            },
            WindowEvent::Size(width, height) => {
                let (width, height) = (*width, *height);
                match (width==0) || (height==0) {
                    true => {
                        window.window.iconify()
                    },
                    false => {
                        window.aspect_ratio = width as f32/height as f32;
                        intermediate_opengl::viewport(&window.opengl, width, height)
                    },
                }
            },
            _ => {},
        }
        Ok(())
    }
}