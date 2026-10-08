use crate::context::Context;
use crate::enums::ContextError;
use crate::opengl::intermediate_opengl;
use glfw::{WindowEvent, Action, Key};

pub(crate) fn close_window_escape(events:&Vec<WindowEvent>, render:&mut Context) {
    for event in events {
        match event {
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                render.window.set_should_close(true)
            },
            _ => {},
        }
    }
}

pub(crate) fn necessary_window_stuff(events:&Vec<WindowEvent>, render:&mut Context) {

    for event in events {
        match event {
            WindowEvent::Close => {
                render.window.set_should_close(true)
            },
            WindowEvent::Size(width, height) => {
                let (width, height) = (*width, *height);
                match (width==0) || (height==0) {
                    true => {
                        render.window.iconify()
                    },
                    false => {
                        render.window.set_aspect_ratio(width as f32/height as f32);
                        intermediate_opengl::viewport(&render.window.get_opengl_handle(), width, height)
                    },
                }
            },
            _ => {},
        }
    }
}