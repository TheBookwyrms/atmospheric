use crate::camera::Camera;
use crate::context::AssortedContextDetails;
use crate::glfw::window::Window;
use crate::modules::keybindings::KeybindingModule;
use crate::enums::ContextError;
use glfw::{WindowEvent, Action};


pub struct MouseOnlyMovement {}
impl KeybindingModule for MouseOnlyMovement {
    fn call_keybindings(&self, event:&WindowEvent, window:&mut Window, camera:&mut Camera, _:&mut AssortedContextDetails) -> Result<(), ContextError> {
        match event {
            WindowEvent::MouseButton(button, action, _mods) => {
                match action {
                    Action::Press => {
                        match button {
                            glfw::MouseButton::Button1 => {camera.panning = true}, // left button
                            glfw::MouseButton::Button2 => {camera.angling = true}, // right button
                            _ => {},
                        }
                    },
                    Action::Release => {
                        match button {
                            glfw::MouseButton::Button1 => {camera.panning = false}, // left button
                            glfw::MouseButton::Button2 => {camera.angling = false}, // right button
                            _ => {},
                        }
                    },
                    Action::Repeat => {},
                };
                Ok::<(), ContextError>(())
            },

            WindowEvent::Scroll(_xoffset, yoffset) => {
                {camera.zoom -= ((0.24*yoffset) as f32) * camera.zoom*0.25; Ok(())}
            },

            WindowEvent::CursorPos(xpos, ypos) => {
                let dx = *xpos as f32 - window.last_cursor_pos[0];
                let dy = *ypos as f32 - window.last_cursor_pos[1];

                if camera.panning {
                    let sensitivity = camera.pan_sensitivity * camera.zoom;
                    camera.translation_by_internal_axes(0.0, -dx*sensitivity, dy*sensitivity)?;
                    //camera.pan_xyz += Vector::from_1darray([dx, -1.0*dy, 0.0])
                    //                                .multiply_by_constant(sensitivity);
                }
                if camera.angling {
                    let sensitivity = camera.angle_sensitivity * camera.zoom;
                    camera.translate_relative_to_the_target(0.0, -dx*sensitivity, dy*sensitivity)?;
                    //camera.angle_xyz += Vector::from_1darray([dy, dx, 0.0])
                    //                                .multiply_by_constant(sensitivity);
                }

                window.last_cursor_pos = [*xpos as f32, *ypos as f32];

                Ok(())
            }
            _ => Ok(()),
        }
    }
}