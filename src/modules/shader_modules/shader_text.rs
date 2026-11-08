use crate::objects::lighting::LightCounter;
use crate::opengl::gl::Gl;
use crate::enums::{GlError, LightSourceForm, ShaderType};
use crate::opengl::intermediate_opengl;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShaderText<'a> {
    vertex_text:&'a str,
    fragment_text:&'a str,
}
impl<'a> ShaderText<'a> {
    pub fn get_vertex_text(&self) -> &str { self.vertex_text }
    pub fn get_fragment_text(&self) -> &str { self.fragment_text }

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