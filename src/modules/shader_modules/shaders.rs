use crate::camera::Camera;
use crate::glfw::window::Window;
use crate::modules::uniform_modules::uniform::UniformModule;
use crate::objects::lighting::{Light, LightCounter};
use crate::opengl::gl::Gl;
use crate::enums::{
    CameraVector, ContextError, DataFormat, DrawMode, GlError, UniformType
};
use crate::opengl::intermediate_opengl;
use crate::opengl::abstractions::WithVao;
use crate::modules::shader_modules::shader_module_enum::ShaderModule;

use numeracy::matrices::{Matrix, S2, GraphicsMatrices, Grapics, RowMajor};
use numeracy::vectors::Vector;


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

    pub fn disuse_program(&mut self, opengl:&Gl) {
        intermediate_opengl::disuse_program(opengl);
        self.current_program = None;
        self.current_program_type = None;
    }

    pub fn set_uniform_int(&self, opengl:&Gl, uniform_name:&str, value:i32
    ) -> Result<(), GlError> {
        match self.current_program {
            Some(id) => intermediate_opengl::set_uniform_int(opengl, id, uniform_name, value),
            None => Err(GlError::NoProgramBound),
        }
    }

    pub fn set_uniform_float(&self, opengl:&Gl, uniform_name:&str, value:f32
    ) -> Result<(), GlError> {
        match self.current_program {
            Some(id) => intermediate_opengl::set_uniform_float(opengl, id, uniform_name, value),
            None => Err(GlError::NoProgramBound),
        }
    }

    pub fn set_uniform_vec3(&self, opengl:&Gl, uniform_name:&str, value:Vector<f32, 3>
    ) -> Result<(), GlError> {
        match self.current_program {
            Some(id) => intermediate_opengl::set_uniform_vec3(opengl, id, uniform_name, value),
            None => Err(GlError::NoProgramBound),
        }
    }

    pub fn set_uniform_mat4(&self, opengl:&Gl, uniform_name:&str, value:Matrix<f32, 2, S2<4, 4>>
    ) -> Result<(), GlError> {
        match self.current_program {
            Some(id) => intermediate_opengl::set_uniform_mat4(opengl, id, uniform_name, value),
            None => Err(GlError::NoProgramBound),
        }
    }

    pub fn draw<T:Clone, const ITEMS_PER_VERTEX:usize, const NUM_VERTICES:usize>(
        &self, objects:WithVao,
        mode:DrawMode, data:&Matrix<T, 2, S2<ITEMS_PER_VERTEX, NUM_VERTICES>>,
        format:DataFormat
    ) -> Result<(), GlError> {

        if let Some(program) = self.current_program_type {
            program.check_data_format(format)?
        } else {
            Err(GlError::InvalidProgramType)?
        }

        Ok(objects.draw(mode, data))

    }

    pub fn set_uniforms_for_program(&self, shader:ShaderModule<'a>, window:&Window, camera:&Camera, lights:Vec<Light>) -> Result<(), ContextError> {
        match shader {
            ShaderModule::InstancingBlinnPhong => {
                self.set_orthographic_camera_uniforms(window, camera)?;
                self.set_blinn_phong_uniforms(window, camera)?;
                for light in lights {
                    light.set_lighting_uniforms(window, &self)?
                }
            },
            ShaderModule::SimpleTransform => {
                self.set_orthographic_camera_uniforms(window, camera)?;
            },
            ShaderModule::PhongOrthographic | ShaderModule::PhongTexture => {
                self.set_orthographic_camera_uniforms(window, camera)?;
                self.set_blinn_phong_uniforms(window, camera)?;
            },
            ShaderModule::SimpleOrthographic => self.set_orthographic_camera_uniforms(window, camera)?,
            ShaderModule::SimpleTexture | ShaderModule::TwoTexture => self.set_orthographic_camera_uniforms(window, camera)?,
            ShaderModule::Custom(_) => {},
        }
        Ok(())
    }
    pub fn set_world_transform_uniform(&self, opengl:&Gl, transform:Matrix<f32, 2, S2<4, 4>>) -> Result<(), ContextError> {
        let model_transform = GraphicsMatrices::<RowMajor>::opengl_to_right_handed().matmul(&transform);

        self.set_uniform_mat4(opengl, "world_transform", model_transform)?;
        Ok(())
    }

    pub fn set_orthographic_camera_uniforms(&self, window:&Window, camera:&Camera) -> Result<(), ContextError> {
        // model
        self.set_world_transform_uniform(window.get_opengl_handle(), Matrix::identity())?;

        // view
        self.set_uniform_mat4(window.get_opengl_handle(), "camera_transformation", camera.get_camera_view_matrix())?;

        // projection
        self.set_uniform_mat4(window.get_opengl_handle(), "orthographic_projection",
            camera.get_orthographic_projection(window.get_aspect_ratio()))?;
        Ok(())
    }

    pub fn set_blinn_phong_uniforms(&self, window:&Window, camera:&Camera) -> Result<(), ContextError> {
        self.set_uniform_vec3(window.get_opengl_handle(),"camera_viewpos",
                camera.camera_info.get_camera(CameraVector::Position)
            )?;
        Ok(())
    }



    //pub fn set_custom_uniform<T:Clone, const N:usize, U:ShapeTrait<N>>(&self, program_id:u32, uniform:Uniform, value:Matrix<T, N, U>) -> Result<(), GlError> {
    //    intermediate_opengl::set_uniform(&self.window.get_opengl_handle(), program_id, uniform.name, uniform.uniform_type, value.as_ptr())
    //}

    pub fn get_current_program(&self) -> Option<u32> {
        self.current_program
    }

    pub fn get_current_program_type(&self) -> Option<ShaderModule<'a>> {
        self.current_program_type
    }

}