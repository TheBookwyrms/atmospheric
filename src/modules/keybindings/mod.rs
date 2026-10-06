use std::sync::Arc;

pub mod camera_movement;
pub mod window;
pub mod pause;
pub mod screenshot;
pub mod camera_mode;

use glfw::WindowEvent;
use crate::context::Context;
use crate::enums::ContextError;
use crate::modules::keybindings::camera_movement::{mouse_pan_left_click, mouse_rotate_right_click, mouse_scroll_callback};
use crate::modules::keybindings::pause::pause_space;
use crate::modules::keybindings::screenshot::screenshot_ctrl_k;
use crate::modules::keybindings::window::{close_window_escape, necessary_window_stuff};
use crate::modules::keybindings::camera_mode::change_camera_mode_e;



#[derive(Clone)]
pub enum KeybindingModule {
    MouseScroll,
    MousePanLeftClick,
    MouseRotateRightClick,
    PauseSpace,
    ScreenshotCtrlK,
    CloseWindowEscape,
    NecessaryWindowStuff,
    ChangeCameraModeE,
    Custom(Arc<dyn Fn(&WindowEvent, &mut Context)->Result<(), ContextError>>),
}
impl KeybindingModule {
    pub fn keybinding_callback(&self, event:&WindowEvent, render:&mut Context) -> Result<(), ContextError> {
        match self {
            KeybindingModule::MouseScroll => mouse_scroll_callback(event, render),
            KeybindingModule::MousePanLeftClick => mouse_pan_left_click(event, render),
            KeybindingModule::MouseRotateRightClick => mouse_rotate_right_click(event, render),
            KeybindingModule::PauseSpace => pause_space(event, render),
            KeybindingModule::ScreenshotCtrlK => screenshot_ctrl_k(event, render),
            KeybindingModule::CloseWindowEscape => close_window_escape(event, render),
            KeybindingModule::NecessaryWindowStuff => necessary_window_stuff(event, render),
            KeybindingModule::ChangeCameraModeE => change_camera_mode_e(event, render),
            KeybindingModule::Custom(callback) => callback(event, render),
        }
    }
}