use crate::camera::Camera;
use crate::context::AssortedContextDetails;
use crate::glfw::window::Window;
use crate::enums::{ContextError, InternalFormat};
use crate::image_processing;
use crate::modules::keybindings::KeybindingModule;
use glfw::{Modifiers, WindowEvent, Action, Key};

pub struct ScreenshotKeybindings {}
impl KeybindingModule for ScreenshotKeybindings {
    fn call_keybindings(&self, event:&WindowEvent, window:&mut Window, _:&mut Camera, details:&mut AssortedContextDetails) -> Result<(), ContextError> {
        match event {
            WindowEvent::Key(Key::K, _, Action::Press, Modifiers::Control) => {
                let pixels = window.read_pixels_full_window(&window.opengl, InternalFormat::RGBA);
                let name = &format!("{}.png", (details.screenshot_naming_convention)());
                image_processing::Image::save_png(pixels, name, window.wh_usize(), InternalFormat::RGBA, true)
            },
            _ => Ok(()),
        }?;
        Ok(())
    }
}