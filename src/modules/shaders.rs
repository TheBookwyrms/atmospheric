use crate::objects::lighting::LightCounter;
use crate::opengl::gl::Gl;
use crate::enums::{
    GlError, ShaderType, DataFormat, LightSourceForm
};
use crate::opengl::intermediate_opengl;



#[cfg(target_os = "linux")]
include!(concat!(env!("OUT_DIR"), "/shaders_glsl.rs"));
#[cfg(target_os = "windows")]
include!(concat!(env!("OUT_DIR"), "\\shaders_glsl.rs"));


#[derive(Debug, Clone)]
pub struct Shaders<'a> {
    shaders:Vec<(ShaderModule<'a>, u32)>,
    current_program:Option<u32>,
    current_program_type:Option<ShaderModule<'a>>,

}
impl<'a> Shaders<'a> {


    pub fn using(opengl:&Gl, shaders:Vec<ShaderModule<'a>>, max_lights:&LightCounter) -> Result<Self, GlError> {
        let mut shaders_compiled = vec![];
        for shader in shaders {
            shaders_compiled.push((shader, shader.get_shader_text().compile_program_from_text(opengl, max_lights)?));
        }
        Ok(Self { shaders : shaders_compiled, current_program:None, current_program_type:None })
    }

    pub fn get_id(&self, module:ShaderModule) -> Option<u32> {
        let mut shader_id = None;
        for (shader, id) in &self.shaders {
            if shader == &module {
                shader_id = Some(*id)
            }
        }
        shader_id
    }

    pub fn use_program(&mut self, opengl:&Gl, program:ShaderModule<'a>) -> Result<(), GlError> {
        if let Some(id) = self.get_id(program) {
            intermediate_opengl::use_program(opengl, id)?;
            self.current_program = Some(id);
            self.current_program_type = Some(program);
            Ok(())
        } else {
            Err(GlError::ShaderModuleIdNotFound)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShaderModule<'a> {
    InstancingBlinnPhong,
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
            ShaderModule::InstancingBlinnPhong => ShaderText { vertex_text: INSTANCING_BLINN_PHONG_VERTEX, fragment_text: INSTANCING_BLINN_PHONG_FRAGMENT },
            ShaderModule::PhongOrthographic    => ShaderText { vertex_text: PHONG_ORTHOGRAPHIC_VERTEX, fragment_text: PHONG_ORTHOGRAPHIC_FRAGMENT },
            ShaderModule::PhongTexture         => ShaderText { vertex_text: PHONG_TEXTURE_VERTEX, fragment_text: PHONG_TEXTURE_FRAGMENT },
            ShaderModule::SimpleOrthographic   => ShaderText { vertex_text: SIMPLE_ORTHOGRAPHIC_VERTEX, fragment_text: SIMPLE_ORTHOGRAPHIC_FRAGMENT },
            ShaderModule::SimpleTexture        => ShaderText { vertex_text: SIMPLE_TEXTURE_VERTEX, fragment_text: SIMPLE_TEXTURE_FRAGMENT },
            ShaderModule::TwoTexture           => ShaderText { vertex_text: TWO_TEXTURE_VERTEX, fragment_text: TWO_TEXTURE_FRAGMENT },
            ShaderModule::Custom(shader_text) => *shader_text,
        }
    }
    pub fn get_vertex_text(&self) -> &str {
        match self {
            ShaderModule::InstancingBlinnPhong => INSTANCING_BLINN_PHONG_VERTEX,
            ShaderModule::PhongOrthographic => PHONG_ORTHOGRAPHIC_VERTEX,
            ShaderModule::PhongTexture => PHONG_TEXTURE_VERTEX,
            ShaderModule::SimpleOrthographic => SIMPLE_ORTHOGRAPHIC_VERTEX,
            ShaderModule::SimpleTexture => SIMPLE_TEXTURE_VERTEX,
            ShaderModule::TwoTexture => TWO_TEXTURE_VERTEX,
            ShaderModule::Custom(shader_text) => shader_text.vertex_text,
        }
    }
    pub fn get_fragment_text(&self) -> &str {
        match self {
            ShaderModule::InstancingBlinnPhong => INSTANCING_BLINN_PHONG_FRAGMENT,
            ShaderModule::PhongOrthographic => PHONG_ORTHOGRAPHIC_FRAGMENT,
            ShaderModule::PhongTexture => PHONG_TEXTURE_FRAGMENT,
            ShaderModule::SimpleOrthographic => SIMPLE_ORTHOGRAPHIC_FRAGMENT,
            ShaderModule::SimpleTexture => SIMPLE_TEXTURE_FRAGMENT,
            ShaderModule::TwoTexture => TWO_TEXTURE_FRAGMENT,
            ShaderModule::Custom(shader_text) => shader_text.fragment_text,
        }
    }

    pub fn check_data_format(&self, data_format:DataFormat) -> Result<(), GlError> {
        match self {
            Self::SimpleOrthographic => if data_format == DataFormat::Position3Colour3Alpha1 { Ok(()) } else { Err(GlError::InvalidDataFormat) },
            Self::PhongOrthographic => if data_format == DataFormat::Position3Colour3Alpha1Normal3 { Ok(()) } else { Err(GlError::InvalidDataFormat) },
            Self::SimpleTexture | Self::TwoTexture => if data_format == DataFormat::Position3Texture2 { Ok(()) } else { Err(GlError::InvalidDataFormat) },
            Self::PhongTexture => if data_format == DataFormat::Position3Colour3Alpha1Normal3Texture2 { Ok(()) } else { Err(GlError::InvalidDataFormat) },
            Self::InstancingBlinnPhong => if matches!(data_format, DataFormat::Position3Colour4Normal3Texture2Material4TranformationMat4 { .. }) { Ok(()) } else { Err(GlError::InvalidDataFormat) },
            Self::Custom(_u32) => Ok(()),
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShaderText<'a> {
    vertex_text:&'a str,
    fragment_text:&'a str,
}
impl<'a> ShaderText<'a> {
    pub fn text(vertex:&'a str, fragment:&'a str) -> Self {
        Self { vertex_text: vertex, fragment_text: fragment }
    }

    pub fn compile_program_from_text(&self, opengl:&Gl, max_lights:&LightCounter) -> Result<u32, GlError> {

        let dir_max   = max_lights.get_light_count(LightSourceForm::Directional);
        let point_max = max_lights.get_light_count(LightSourceForm::Point);
        let spot_max  = max_lights.get_light_count(LightSourceForm::Spot);

        let fragment = self.fragment_text.replace("find_and_replace_with_max_number_of_point_lights", &point_max.to_string())
                                                 .replace("find_and_replace_with_max_number_of_directional_lights", &dir_max.to_string())
                                                 .replace("find_and_replace_with_max_number_of_spot_lights", &spot_max.to_string());

        let vertex_id = intermediate_opengl::create_shader_variant(opengl, self.vertex_text, ShaderType::VertexShader)?;
        let fragment_id = intermediate_opengl::create_shader_variant(opengl, &fragment, ShaderType::FragmentShader)?;

        let program_id = intermediate_opengl::create_shader_program(opengl, vertex_id, fragment_id)?;

        intermediate_opengl::remove_shader_variant(opengl, program_id, vertex_id);
        intermediate_opengl::remove_shader_variant(opengl, program_id, fragment_id);

        Ok(program_id)
    }
}