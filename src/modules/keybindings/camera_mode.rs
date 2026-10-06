use crate::context::Context;
use crate::enums::ContextError;
use glfw::{WindowEvent, Action, Key};


pub(crate) fn change_camera_mode_e(event:&WindowEvent, render:&mut Context) -> Result<(), ContextError> {
    match event {
        WindowEvent::Key(Key::E, _, Action::Press, _) => render.camera.swap_camera_modes(),
        _ => {},
    }
    Ok(())
}