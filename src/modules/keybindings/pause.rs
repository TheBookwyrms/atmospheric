use std::time::{Duration, Instant};
use crate::context::Context;
use crate::enums::ContextError;
use glfw::{WindowEvent, Action, Key};

pub(crate) fn pause_space(event:&WindowEvent, render:&mut Context) -> Result<(), ContextError> {
    match event {
        WindowEvent::Key(Key::Space, _, Action::Press, _) => {
            match render.assorted_details.paused {
                false => {
                    render.assorted_details.paused = true;
                    render.assorted_details.pause_time = Instant::now()
                },
                true => {
                    if Instant::now().duration_since(render.assorted_details.pause_time) > Duration::from_millis(render.assorted_details.pause_minimum) {
                        render.assorted_details.paused = false
                    }
                }
            };
        },
        _ => {},
    }
    Ok(())
}