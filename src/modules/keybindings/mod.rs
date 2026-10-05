use glfw::WindowEvent;
use crate::camera::Camera;
use crate::context::AssortedContextDetails;
use crate::enums::ContextError;
use crate::glfw::window::Window;

pub mod camera_movement;
pub mod window;
pub mod pause;
pub mod screenshot;


pub trait KeybindingModule {
    fn call_keybindings(&self, event:&WindowEvent, window:&mut Window, camera:&mut Camera, details:&mut AssortedContextDetails) -> Result<(), ContextError>;
}