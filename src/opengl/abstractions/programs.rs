use crate::config::RenderInitialConfig;
use crate::lighting::LightCounter;
use crate::opengl::gl::Gl;
use crate::enums::{
    DataFormat, DrawCall, DrawMode, GlError, LightSourceForm, ProgramSelect, ShaderType, UniformType
};
use crate::opengl::intermediate_opengl;

use crate::opengl::abstractions::WithObject;

//use numeracy::matrices::Matrix;
use numeracy::matrices2::Matrix;


include!(concat!(env!("OUT_DIR"), "\\shaders_glsl.rs"));



#[derive(Clone, Copy)]
pub struct Programs {
    pub simple_orthographic_shader:u32,
    pub blinn_phone_orthographic_shader:u32,
    pub simple_texture_shader:u32,
    pub phong_texture_shader:u32,
    pub current_program:Option<u32>,
    pub current_program_type:Option<ProgramSelect>,
}
impl Programs {

    pub fn compile_program_from_text(opengl:&Gl, vertex_text:&str, fragment_text:&str) -> Result<u32, GlError> {
        let vertex_id = intermediate_opengl::create_shader_variant(opengl, vertex_text, ShaderType::VertexShader)?;
        let fragment_id = intermediate_opengl::create_shader_variant(opengl, fragment_text, ShaderType::FragmentShader)?;

        let program_id = intermediate_opengl::create_shader_program(opengl, vertex_id, fragment_id)?;

        intermediate_opengl::remove_shader_variant(opengl, program_id, vertex_id);
        intermediate_opengl::remove_shader_variant(opengl, program_id, fragment_id);

        Ok(program_id)
    }

    pub fn compile_program_from_select(opengl:&Gl, program_type:ProgramSelect, max_lights:&LightCounter) -> Result<u32, GlError> {
        
        let dir_max   = max_lights.get_light_count(LightSourceForm::Directional);
        let point_max = max_lights.get_light_count(LightSourceForm::Point);
        let spot_max  = max_lights.get_light_count(LightSourceForm::Spot);

        match program_type {
            ProgramSelect::SelectBlinnPhongOrthographic => {
                let vertex_text   = BLINN_PHONG_ORTHOGRAPHIC_VERTEX;
                let fragment_text = BLINN_PHONG_ORTHOGRAPHIC_FRAGMENT
                .replace("find_and_replace_with_max_number_of_point_lights", &point_max.to_string())
                .replace("find_and_replace_with_max_number_of_directional_lights", &dir_max.to_string())
                .replace("find_and_replace_with_max_number_of_spot_lights", &spot_max.to_string());
                let shader_id = Programs::compile_program_from_text(
                    opengl, vertex_text, &fragment_text
                )?;
                Ok(shader_id)
            },
            ProgramSelect::SelectSimpleOrthographic => {
                let vertex_text   = SIMPLE_ORTHOGRAPHIC_VERTEX;
                let fragment_text = SIMPLE_ORTHOGRAPHIC_FRAGMENT;
                let shader_id = Programs::compile_program_from_text(
                    opengl, vertex_text, fragment_text
                )?;
                Ok(shader_id)
            },
            ProgramSelect::SelectSimpleTexture => {
                let vertex_text   = SIMPLE_TEXTURE_VERTEX;
                let fragment_text = SIMPLE_TEXTURE_FRAGMENT;
                let shader_id = Programs::compile_program_from_text(
                    opengl, vertex_text, fragment_text
                )?;
                Ok(shader_id)
            },
            ProgramSelect::SelectPhongTexture => {
                let vertex_text   = PHONG_TEXTURE_VERTEX;
                let fragment_text = PHONG_TEXTURE_FRAGMENT
                .replace("find_and_replace_with_max_number_of_point_lights", &point_max.to_string())
                .replace("find_and_replace_with_max_number_of_directional_lights", &dir_max.to_string())
                .replace("find_and_replace_with_max_number_of_spot_lights", &spot_max.to_string());
                let shader_id = Programs::compile_program_from_text(
                    opengl, vertex_text, &fragment_text
                )?;
                Ok(shader_id)
            },
            ProgramSelect::Custom(_) => Err(GlError::InvalidCustomProgramSelect)
        }
    }

    pub fn compile(opengl:&Gl, max_lights:&LightCounter) -> Result<Programs, GlError> {
        let simple_orthographic_shader = Programs::compile_program_from_select(opengl, ProgramSelect::SelectSimpleOrthographic, max_lights)?;
        let blinn_phone_orthographic_shader = Programs::compile_program_from_select(opengl, ProgramSelect::SelectBlinnPhongOrthographic, max_lights)?;
        let simple_texture_shader = Programs::compile_program_from_select(opengl, ProgramSelect::SelectSimpleTexture, max_lights)?;
        let phong_texture_shader = Programs::compile_program_from_select(opengl, ProgramSelect::SelectPhongTexture, max_lights)?;

        Ok(Programs { simple_orthographic_shader, blinn_phone_orthographic_shader,
                      simple_texture_shader, phong_texture_shader,
                      current_program:None, current_program_type:None })
    }

    pub fn use_program(&mut self, opengl:&Gl, program:ProgramSelect) -> Result<(), GlError> {
        match program {
            ProgramSelect::SelectSimpleOrthographic => {
                intermediate_opengl::use_program(opengl, self.simple_orthographic_shader)?;
                self.current_program = Some(self.simple_orthographic_shader);
                self.current_program_type = Some(program);
                Ok(())
            },
            ProgramSelect::SelectBlinnPhongOrthographic => {
                intermediate_opengl::use_program(opengl, self.blinn_phone_orthographic_shader)?;
                self.current_program = Some(self.blinn_phone_orthographic_shader);
                self.current_program_type = Some(program);
                Ok(())
            },
            ProgramSelect::SelectSimpleTexture => {
                intermediate_opengl::use_program(opengl, self.simple_texture_shader)?;
                self.current_program = Some(self.simple_texture_shader);
                self.current_program_type = Some(program);
                Ok(())
            },
            ProgramSelect::SelectPhongTexture => {
                intermediate_opengl::use_program(opengl, self.phong_texture_shader)?;
                self.current_program = Some(self.phong_texture_shader);
                self.current_program_type = Some(program);
                Ok(())
            },
            ProgramSelect::Custom(id) => {
                intermediate_opengl::use_program(opengl, id)?;
                self.current_program = Some(id);
                self.current_program_type = Some(program);
                Ok(())
            }
        }
    }

    pub fn disuse_program(&mut self, opengl:&Gl) {
        intermediate_opengl::disuse_program(opengl);
        self.current_program = None;
        self.current_program_type = None;
    }

    pub fn set_uniform<T:Clone, const N:usize>(&self, opengl:&Gl, uniform_name:&str, uniform_type:UniformType, value:Matrix<T, N>
    ) -> Result<(), GlError> {
        match self.current_program {
            Some(id) => intermediate_opengl::set_uniform(opengl, id, uniform_name, uniform_type, value.as_ptr()),
            None => Err(GlError::InvalidProgramID),
        }
    }

    pub fn draw<T:Clone, const N:usize>(
        &self, objects:WithObject, call:DrawCall,
        mode:DrawMode, data:&Matrix<T, N>,
    ) -> Result<(), GlError> {

        let format = objects.get_data_format();

        match self.current_program_type {
            None => Err(GlError::InvalidProgramType),
            Some(program) => {
                match program {
                    ProgramSelect::SelectSimpleOrthographic => {
                        if format == DataFormat::Position3Colour3Alpha1 { Ok(()) } else { Err(GlError::InvalidDataFormat) }
                    },
                    ProgramSelect::SelectBlinnPhongOrthographic => {
                        if format == DataFormat::Position3Colour3Alpha1Normal3 { Ok(()) } else { Err(GlError::InvalidDataFormat) }
                    },
                    ProgramSelect::SelectSimpleTexture => {
                        if format == DataFormat::Position3Texture2 { Ok(()) } else { Err(GlError::InvalidDataFormat) }
                    },
                    ProgramSelect::SelectPhongTexture => {
                        if format == DataFormat::Position3Colour3Alpha1Normal3Texture2 { Ok(()) } else { Err(GlError::InvalidDataFormat) }
                    },
                    ProgramSelect::Custom(_) => Ok(())
                }
            },
        }?;

        //if format == DataFormat::Position3Texture2 {
        //    raw_opengl::bind_texture(opengl, gl::TEXTURE_2D, texture);
        //}

        Ok(objects.draw(call, mode, data)?)
        //let with_vao = WithObject::existing(opengl, Object::VAO, vao, format);
        //Ok(with_vao.draw(call, mode, data)?)

    }
}
