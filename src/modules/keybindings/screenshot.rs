use crate::context::Context;
use crate::enums::{ContextError, InternalFormat};
use crate::image_processing;
use glfw::{Action, Key, Modifiers, Scancode, WindowEvent};

pub(crate) fn screenshot_ctrl_k(events:&Vec<WindowEvent>, render:&mut Context) -> Result<(), ContextError> {
    for event in events {
        match event {
            WindowEvent::Key(Key::K, _, Action::Press, Modifiers::Control) => {
                let pixels = render.window.read_pixels_full_window(&render.window.get_opengl_handle(), InternalFormat::RGBA);
                let name = &format!("{}.png", (render.assorted_details.screenshot_naming_convention)());
                image_processing::Image::save_png(pixels, name, render.window.wh_usize(), InternalFormat::RGBA, true)
            },
            _ => Ok(()),
        }?;
    }
    Ok(())
}