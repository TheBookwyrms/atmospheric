use crate::opengl::abstractions::Programs;
use crate::opengl::gl::Gl;
use crate::enums::GlError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShaderProgram {
    shader_id:u32,
}
impl ShaderProgram {
    pub fn compile(opengl:&Gl, vertex_text:&str, fragment_text:&str) -> Result<Self, GlError> {
        let id = Programs::compile_program_from_text(opengl, vertex_text, fragment_text)?;

        Ok( Self { shader_id:id } )
    }
    pub fn get_id(&self) -> u32 {
        self.shader_id
    }
}