use crate::context::Context;
use crate::enums::ContextError;
use glfw::{WindowEvent, Action};

pub(crate) fn mouse_scroll_callback(event:&WindowEvent, render:&mut Context) -> Result<(), ContextError> {
    match event {
        WindowEvent::Scroll(_xoffset, yoffset) => {
            // z = z - 0.24*y*z*0.25
            // z = z*(1-0.06*y)
            render.camera.offset_zoom(-0.06 * *yoffset as f32 * render.camera.get_zoom());
            //render.camera.offset_zoom(-1.0*((0.24*yoffset) as f32) * render.camera.get_zoom()*0.25);
            //render.camera.get_zoom() -= ((0.24*yoffset) as f32) * render.camera.get_zoom()*0.25
        },
        _ => {},
    }
    Ok(())
}

pub(crate) fn mouse_pan_left_click(event:&WindowEvent, render:&mut Context) -> Result<(), ContextError> {
    match event {
        WindowEvent::MouseButton(button, action, _mods) => {
            match action {
                Action::Press => {
                    match button {
                        glfw::MouseButton::Button1 => {render.camera.panning = true}, // left button
                        _ => {},
                    }
                },
                Action::Release => {
                    match button {
                        glfw::MouseButton::Button1 => {render.camera.panning = false}, // left button
                        _ => {},
                    }
                },
                Action::Repeat => {},
            };
            Ok::<(), ContextError>(())
        },

        WindowEvent::CursorPos(xpos, ypos) => {
            let last_cursor_pos = render.window.get_last_cursor_pos();
            let dx = *xpos as f32 - last_cursor_pos[0];
            let dy = *ypos as f32 - last_cursor_pos[1];

            if render.camera.panning {
                let sensitivity = render.camera.get_pan_sensitivity() * render.camera.get_zoom();
                render.camera.translation_by_internal_axes(0.0, -dx*sensitivity, dy*sensitivity)?;
            }

            Ok(())
        }
        _ => Ok(()),
    }
}

pub(crate) fn mouse_rotate_right_click(event:&WindowEvent, render:&mut Context) -> Result<(), ContextError> {
    match event {
        WindowEvent::MouseButton(button, action, _mods) => {
            match action {
                Action::Press => {
                    match button {
                        glfw::MouseButton::Button2 => {render.camera.angling = true}, // right button
                        _ => {},
                    }
                },
                Action::Release => {
                    match button {
                        glfw::MouseButton::Button2 => {render.camera.angling = false}, // right button
                        _ => {},
                    }
                },
                Action::Repeat => {},
            };
            Ok::<(), ContextError>(())
        },


        WindowEvent::CursorPos(xpos, ypos) => {
            let last_cursor_pos = render.window.get_last_cursor_pos();
            let dx = *xpos as f32 - last_cursor_pos[0];
            let dy = *ypos as f32 - last_cursor_pos[1];

            if render.camera.angling {
                let sensitivity = render.camera.get_angle_sensitivity() * render.camera.get_zoom();
                render.camera.translate_relative_to_the_target(0.0, -dx*sensitivity, dy*sensitivity)?;
            }

            Ok(())
        }
        _ => Ok(()),
    }
}