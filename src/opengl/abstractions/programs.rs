//use crate::config::RenderInitialConfig;
use crate::lighting::LightCounter;
use crate::opengl::gl::Gl;
use crate::opengl_helpers::enums::{
    DataFormat, DrawCall, DrawMode, GlError, LightSourceForm, ShaderType, UniformType
};
use crate::opengl::intermediate_opengl;

use crate::opengl::abstractions::{WithVao, WithVbo, programs};
//use crate::opengl::abstractions::{WithObject, WithVao, WithVbo};

//use numeracy::matrices::Matrix;
use numeracy::matrices::{Matrix, S2, ShapeTrait};



#[cfg(target_os = "linux")]
include!(concat!(env!("OUT_DIR"), "/shaders_glsl.rs"));
#[cfg(target_os = "linux")]
include!(concat!(env!("OUT_DIR"), "/programs.rs"));


#[cfg(target_os = "windows")]
include!(concat!(env!("OUT_DIR"), "\\shaders_glsl.rs"));
#[cfg(target_os = "windows")]
include!(concat!(env!("OUT_DIR"), "\\programs.rs"));


impl ProgramSelect {
    pub fn check_data_format(&self, data_format:DataFormat) -> Result<(), GlError> {
        match self {
            Self::SelectSimpleOrthographic => if data_format == DataFormat::Position3Colour3Alpha1 { Ok(()) } else { Err(GlError::InvalidDataFormat) },
            Self::SelectPhongOrthographic => if data_format == DataFormat::Position3Colour3Alpha1Normal3 { Ok(()) } else { Err(GlError::InvalidDataFormat) },
            Self::SelectSimpleTexture | Self::SelectTwoTexture => if data_format == DataFormat::Position3Texture2 { Ok(()) } else { Err(GlError::InvalidDataFormat) },
            Self::SelectPhongTexture => if data_format == DataFormat::Position3Colour3Alpha1Normal3Texture2 { Ok(()) } else { Err(GlError::InvalidDataFormat) },
            //Self::SelectInstancingPhongTexture => Err(GlError::InvalidDataFormat),
            //Self::SelectInstancingFull => if matches!(data_format, DataFormat::Position3Colour4Normal3Texture2Material4TranformationMat4 { .. }) { Ok(()) } else { Err(GlError::InvalidDataFormat) },
            Self::SelectInstancingBlinnPhong => if matches!(data_format, DataFormat::Position3Colour4Normal3Texture2Material4TranformationMat4 { .. }) { Ok(()) } else { Err(GlError::InvalidDataFormat) },
            //Self::SelectInstancingFull => if data_format == DataFormat::Position3Colour4Normal3Texture2Material4TranformationMat4([UpdateVertexAttrib; 3]) { Ok(()) } else { Err(GlError::InvalidDataFormat) },
            Self::Custom(_u32) => Ok(()),
        }
    }
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



//    pub fn use_program(&mut self, opengl:&Gl, program:ProgramSelect) -> Result<(), GlError> {
//        match program {
//            ProgramSelect::SelectSimpleOrthographic => {
//                intermediate_opengl::use_program(opengl, self.simple_orthographic_shader)?;
//                self.current_program = Some(self.simple_orthographic_shader);
//                self.current_program_type = Some(program);
//                Ok(())
//            },
//            ProgramSelect::SelectPhongOrthographic => {
//                intermediate_opengl::use_program(opengl, self.blinn_phong_orthographic_shader)?;
//                self.current_program = Some(self.blinn_phong_orthographic_shader);
//                self.current_program_type = Some(program);
//                Ok(())
//            },
//            ProgramSelect::SelectSimpleTexture => {
//                intermediate_opengl::use_program(opengl, self.simple_texture_shader)?;
//                self.current_program = Some(self.simple_texture_shader);
//                self.current_program_type = Some(program);
//                Ok(())
//            },
//            ProgramSelect::SelectPhongTexture => {
//                intermediate_opengl::use_program(opengl, self.phong_texture_shader)?;
//                self.current_program = Some(self.phong_texture_shader);
//                self.current_program_type = Some(program);
//                Ok(())
//            },
//            ProgramSelect::Custom(id) => {
//                intermediate_opengl::use_program(opengl, id)?;
//                self.current_program = Some(id);
//                self.current_program_type = Some(program);
//                Ok(())
//            }
//        }
//    }

    pub fn disuse_program(&mut self, opengl:&Gl) {
        intermediate_opengl::disuse_program(opengl);
        self.current_program = None;
        self.current_program_type = None;
    }

    pub fn set_uniform<T:Clone, const N:usize, U:ShapeTrait<N>>(&self, opengl:&Gl, uniform_name:&str, uniform_type:UniformType, value:Matrix<T, N, U>
    ) -> Result<(), GlError> {
        match self.current_program {
            Some(id) => intermediate_opengl::set_uniform(opengl, id, uniform_name, uniform_type, value.as_ptr()),
            None => Err(GlError::InvalidProgramID),
        }
    }

    pub fn draw<T:Clone, const ITEMS_PER_VERTEX:usize, const NUM_VERTICES:usize>(
        &self, objects:WithVao,
        mode:DrawMode, data:&Matrix<T, 2, S2<ITEMS_PER_VERTEX, NUM_VERTICES>>,
        format:DataFormat
    ) -> Result<(), GlError> {

        if let Some(program) = self.current_program_type {
            program.check_data_format(format)?
            //match program {
            //    ProgramSelect::SelectSimpleOrthographic => {
            //        if format == DataFormat::Position3Colour3Alpha1 { Ok(())? } else { Err(GlError::InvalidDataFormat)? }
            //    },
            //    ProgramSelect::SelectBlinnPhongOrthographic => {
            //        if format == DataFormat::Position3Colour3Alpha1Normal3 { Ok(())? } else { Err(GlError::InvalidDataFormat)? }
            //    },
            //    ProgramSelect::SelectSimpleTexture => {
            //        if format == DataFormat::Position3Texture2 { Ok(())? } else { Err(GlError::InvalidDataFormat)? }
            //    },
            //    ProgramSelect::SelectPhongTexture => {
            //        if format == DataFormat::Position3Colour3Alpha1Normal3Texture2 { Ok(())? } else { Err(GlError::InvalidDataFormat)? }
            //    },
            //    ProgramSelect::Custom(_) => Ok(())?
            //}
        } else {
            Err(GlError::InvalidProgramType)?
        }

        Ok(objects.draw(mode, data))

    }

}
