use crate::{enums::{CameraMode, LightSourceForm}, lighting::LightCounter};

#[derive(Clone, Copy)]
pub struct RenderInitialConfig {
    pub window_name:&'static str,
    pub window_height:u32,
    pub window_width:u32,

    pub camera_mode:CameraMode,

    pub max_lights:LightCounter,
}

impl RenderInitialConfig {
    /// in format (max_directional_lighs, max_point_lighs, max_spot_lighs)
    pub fn get_max_lights(&self) -> (u16, u16, u16) {
        (
            self.max_lights.get_light_count(LightSourceForm::Directional),
            self.max_lights.get_light_count(LightSourceForm::Point),
            self.max_lights.get_light_count(LightSourceForm::Spot),
        )
    }
}