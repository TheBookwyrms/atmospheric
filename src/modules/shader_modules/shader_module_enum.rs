use crate::enums::{DataFormat, GlError};
use crate::modules::shader_modules::shader_text::ShaderText;


#[cfg(target_os = "linux")]
include!(concat!(env!("OUT_DIR"), "/shaders_glsl.rs"));
#[cfg(target_os = "windows")]
include!(concat!(env!("OUT_DIR"), "\\shaders_glsl.rs"));

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShaderModule<'a> {
    InstancingBlinnPhong,
    SimpleTransform,
    PhongOrthographic,
    PhongTexture,
    SimpleOrthographic,
    SimpleTexture,
    TwoTexture,
    Custom(ShaderText<'a>)
}
impl<'a> ShaderModule<'a> {
    pub fn get_shader_text(&self) -> ShaderText<'_> {
        match self {
            ShaderModule::InstancingBlinnPhong => ShaderText::text(INSTANCING_BLINN_PHONG_VERTEX, INSTANCING_BLINN_PHONG_FRAGMENT),
            ShaderModule::SimpleTransform      => ShaderText::text(SIMPLE_TRANSFORM_VERTEX, SIMPLE_TRANSFORM_FRAGMENT),
            ShaderModule::PhongOrthographic    => ShaderText::text(PHONG_ORTHOGRAPHIC_VERTEX, PHONG_ORTHOGRAPHIC_FRAGMENT),
            ShaderModule::PhongTexture         => ShaderText::text(PHONG_TEXTURE_VERTEX, PHONG_TEXTURE_FRAGMENT),
            ShaderModule::SimpleOrthographic   => ShaderText::text(SIMPLE_ORTHOGRAPHIC_VERTEX, SIMPLE_ORTHOGRAPHIC_FRAGMENT),
            ShaderModule::SimpleTexture        => ShaderText::text(SIMPLE_TEXTURE_VERTEX, SIMPLE_TEXTURE_FRAGMENT),
            ShaderModule::TwoTexture           => ShaderText::text(TWO_TEXTURE_VERTEX, TWO_TEXTURE_FRAGMENT),
            ShaderModule::Custom(shader_text) => *shader_text,
        }
    }

    pub fn check_data_format(&self, data_format:DataFormat) -> Result<(), GlError> {
        match self {
            Self::SimpleOrthographic => if data_format == DataFormat::Position3Colour3Alpha1 { Ok(()) } else { Err(GlError::InvalidDataFormat) },
            Self::PhongOrthographic => if data_format == DataFormat::Position3Colour3Alpha1Normal3 { Ok(()) } else { Err(GlError::InvalidDataFormat) },
            Self::SimpleTexture | Self::TwoTexture => if data_format == DataFormat::Position3Texture2 { Ok(()) } else { Err(GlError::InvalidDataFormat) },
            Self::PhongTexture => if data_format == DataFormat::Position3Colour3Alpha1Normal3Texture2 { Ok(()) } else { Err(GlError::InvalidDataFormat) },
            //Self::InstancingBlinnPhong => if matches!(data_format, DataFormat::Position3Colour4Normal3Texture2Material4TranformationMat4 { .. }) { Ok(()) } else { Err(GlError::InvalidDataFormat) },
            Self::InstancingBlinnPhong => Ok(()),
            Self::SimpleTransform => Ok(()),
            Self::Custom(_u32) => Ok(()),
        }
    }
    


}