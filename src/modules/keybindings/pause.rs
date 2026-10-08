use std::time::{Duration, Instant};
use crate::context::Context;
use crate::enums::ContextError;
use glfw::{WindowEvent, Action, Key};

pub(crate) fn pause_space(events:&Vec<WindowEvent>, render:&mut Context) {

    if render.window.get_window().get_key(Key::Space) == Action::Press {
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
    }
}