use numeracy::{matrices::{Matrix, S2}, vectors::Vector};

use crate::{enums::{GlError, UniformType}, opengl::{self, gl::Gl, intermediate_opengl}};

pub struct Uniform<'a> {
    pub name:&'a str,
    pub uniform_type:UniformType,
}


pub enum UniformModule<'a> {
    WorldTransform { model_transform : Matrix<f32, 2, S2<4, 4>> },
    CameraTransformation { camera_view_matrix : Matrix<f32, 2, S2<4, 4>> },
    OrthographicProjection { orthographic_projection : Matrix<f32, 2, S2<4, 4>> },
    CameraViewpos { camera_viewpos : Vector<f32, 3> },
    CustomInt { val: i32, name:&'a str },
    CustomFloat { val: f32, name:&'a str },
    CustomVec3 { val: Vector<f32, 3>, name:&'a str },
    CustomMat4 { val: Matrix<f32, 2, S2<4, 4>>, name:&'a str },
}
impl<'a> UniformModule<'a> {
    pub fn set_uniform(self, opengl:&Gl, id:u32) -> Result<(), GlError> {
        match self {
            Self::WorldTransform { model_transform } => intermediate_opengl::set_uniform_mat4(opengl, id, "model_transform", model_transform),
            Self::CameraTransformation { camera_view_matrix } => intermediate_opengl::set_uniform_mat4(opengl, id, "camera_view_matrix", camera_view_matrix),
            Self::OrthographicProjection { orthographic_projection } => intermediate_opengl::set_uniform_mat4(opengl, id, "orthographic_projection", orthographic_projection),
            Self::CameraViewpos { camera_viewpos } => intermediate_opengl::set_uniform_vec3(opengl, id, "camera_viewpos", camera_viewpos),
            Self::CustomInt { val, name } => intermediate_opengl::set_uniform_int(opengl, id, name, val),
            Self::CustomFloat { val, name } => intermediate_opengl::set_uniform_float(opengl, id, name, val),
            Self::CustomVec3 { val, name } => intermediate_opengl::set_uniform_vec3(opengl, id, name, val),
            Self::CustomMat4 { val, name } => intermediate_opengl::set_uniform_mat4(opengl, id, name, val),
        }
    }
}