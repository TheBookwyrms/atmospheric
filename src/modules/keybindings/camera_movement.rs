use crate::enums::CameraAxis::Right;
use crate::{context::Context, enums::CameraAxis};
use crate::enums::ContextError;
use glfw::{Action, Key, Modifiers, MouseButton, Scancode, WindowEvent};

pub(crate) fn camera_zoom_mouse_scroll(events:&Vec<WindowEvent>, render:&mut Context) -> Result<(), ContextError> {
    for event in events {
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
    }
    Ok(())
}

pub(crate) fn camera_pan_left_click(render:&mut Context) {
    if render.window.get_window().get_mouse_button(MouseButton::Button1) == Action::Press {
        let (xpos, ypos) = render.window.get_window().get_cursor_pos();

        let last_cursor_pos = render.window.get_last_cursor_pos();
        let dx = xpos as f32 - last_cursor_pos[0];
        let dy = ypos as f32 - last_cursor_pos[1];

        let sensitivity = render.camera.get_pan_sensitivity() * render.camera.get_zoom();
        render.camera.translation_by_internal_axes(0.0, -dx*sensitivity, dy*sensitivity);
    }
}

pub(crate) fn camera_rotate_right_click(render:&mut Context) -> Result<(), ContextError> {
    if render.window.get_window().get_mouse_button(MouseButton::Button2) == Action::Press {
        let (xpos, ypos) = render.window.get_window().get_cursor_pos();

        let last_cursor_pos = render.window.get_last_cursor_pos();
        let dx = xpos as f32 - last_cursor_pos[0];
        let dy = ypos as f32 - last_cursor_pos[1];

        let sensitivity = render.camera.get_angle_sensitivity() * render.camera.get_zoom();
        render.camera.translate_relative_to_the_target(0.0, -dx*sensitivity, dy*sensitivity)?;
    }
    Ok(())
}

pub(crate) fn camera_pan_wasd(render:&mut Context) {
    if render.window.get_window().get_key(Key::W) == Action::Press {
        render.camera.translation_by_internal_axes(-10.0*render.camera.get_pan_sensitivity(), 0., 0.)
    }
    if render.window.get_window().get_key(Key::A) == Action::Press {
        render.camera.translation_by_internal_axes(0., -10.0*render.camera.get_pan_sensitivity(), 0.)
    }
    if render.window.get_window().get_key(Key::S) == Action::Press {
        render.camera.translation_by_internal_axes(10.0*render.camera.get_pan_sensitivity(), 0., 0.)
    }
    if render.window.get_window().get_key(Key::D) == Action::Press {
        render.camera.translation_by_internal_axes(0., 10.0*render.camera.get_pan_sensitivity(), 0.)
    }
}

pub(crate) fn camera_rotate_updownleftright(render:&mut Context){
    if render.window.get_window().get_key(Key::Up) == Action::Press {
        render.camera.rotation_about_internal_axes(CameraAxis::Right, 1.)
    }
    if render.window.get_window().get_key(Key::Down) == Action::Press {
        render.camera.rotation_about_internal_axes(CameraAxis::Right, -1.)
    }
    if render.window.get_window().get_key(Key::Left) == Action::Press {
        render.camera.rotation_about_internal_axes(CameraAxis::Up, 1.)
    }
    if render.window.get_window().get_key(Key::Right) == Action::Press {
        render.camera.rotation_about_internal_axes(CameraAxis::Up, -1.)
    }
}