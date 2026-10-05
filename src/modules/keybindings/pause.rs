use std::time::{Duration, Instant};
use crate::camera::Camera;
use crate::context::AssortedContextDetails;
use crate::glfw::window::Window;
use crate::{enums::ContextError, modules::keybindings::KeybindingModule};
use glfw::{WindowEvent, Action, Key};

pub struct PauseKeybindings {}
impl KeybindingModule for PauseKeybindings {
    fn call_keybindings(&self, event:&WindowEvent, _:&mut Window, _:&mut Camera, details:&mut AssortedContextDetails) -> Result<(), ContextError> {
        match event {
            WindowEvent::Key(Key::Space, _, Action::Press, _) => {
                match details.paused {
                    false => {
                        details.paused=true;
                        details.pause_time=Instant::now()
                    },
                    true => {
                        if Instant::now().duration_since(details.pause_time) > Duration::from_millis(details.pause_minimum) {
                            details.paused=false
                        }
                    }
                };
            },
            _ => {},
        }
        Ok(())
    }
}