use std::sync::Arc;

use glfw::WindowEvent;
use crate::context::Context;
use crate::enums::ContextError;
use crate::modules::keybinding_modules::camera_movement::{
    camera_pan_left_click, camera_rotate_right_click, camera_zoom_mouse_scroll,
    camera_pan_wasd, camera_rotate_updownleftright,
};
use crate::modules::keybinding_modules::pause::pause_space;
use crate::modules::keybinding_modules::screenshot::screenshot_ctrl_k;
use crate::modules::keybinding_modules::window::{close_window_escape, necessary_window_stuff};
use crate::modules::keybinding_modules::camera_mode::change_camera_mode_e;



#[derive(Clone)]
pub struct Keybindings {
    keybindings:Vec<KeybindingModule>,
}
impl Keybindings {
    pub fn using(bindings:Vec<KeybindingModule>) -> Self {
        Self { keybindings: bindings }
    }
    pub fn invoke(&self, events:Vec<WindowEvent>, render:&mut Context) -> Result<(), ContextError> {
        for module in &self.keybindings {
            match module {
                KeybindingModule::CameraZoomScroll            => camera_zoom_mouse_scroll(&events, render),
                KeybindingModule::CameraPanLeftClick          => {camera_pan_left_click(render); Ok(())},
                KeybindingModule::CameraRotateRightClick      => camera_rotate_right_click(render),
                KeybindingModule::CameraPanWASD               => {camera_pan_wasd(render); Ok(())},
                KeybindingModule::CameraRotateUpDownLeftRight => {camera_rotate_updownleftright(render); Ok(())},
                KeybindingModule::PauseSpace                  => {pause_space(&events, render); Ok(())},
                KeybindingModule::ScreenshotCtrlK             => screenshot_ctrl_k(&events, render),
                KeybindingModule::CloseWindowEscape           => {close_window_escape(&events, render); Ok(())},
                KeybindingModule::NecessaryWindowStuff        => {necessary_window_stuff(&events, render); Ok(())},
                KeybindingModule::ChangeCameraModeE           => change_camera_mode_e(&events, render),
                KeybindingModule::Custom(callback) => callback(&events, render),
            }?;
        }
        Ok(())
    }
}

#[derive(Clone)]
pub enum KeybindingModule {
    CameraZoomScroll,
    CameraPanLeftClick,
    CameraRotateRightClick,
    CameraPanWASD,
    CameraRotateUpDownLeftRight,
    PauseSpace,
    ScreenshotCtrlK,
    CloseWindowEscape,
    NecessaryWindowStuff,
    ChangeCameraModeE,
    Custom(Arc<dyn Fn(&Vec<WindowEvent>, &mut Context)->Result<(), ContextError>>),
}