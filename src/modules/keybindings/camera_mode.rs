use crate::context::Context;
use crate::enums::ContextError;
use glfw::{WindowEvent, Action, Key};


pub(crate) fn change_camera_mode_e(events:&Vec<WindowEvent>, render:&mut Context) -> Result<(), ContextError> {
    for event in events {
        match event {
            WindowEvent::Key(Key::E, _, Action::Press, _) => render.camera.swap_camera_modes(),
            _ => {},
        }
    }
    Ok(())
}