use std::num::TryFromIntError;

use glfw::{InitError, WindowEvent};
use numeracy::enums::MatrixError;
use crate::opengl::enums::GlError;

#[derive(Debug)]
pub enum RenderError {
    NewGLFWEventDetected(WindowEvent),
    GLFWinitError(InitError),
    GLFWNoWindowCreated,
    GLFWResizeBoundsError((i32, i32)),
    GLError(GlError),
    TryFromIntError(TryFromIntError),
    DataLengthError(usize),
    MatrixError(MatrixError),
}

impl From<GlError> for RenderError {
    fn from(value: GlError) -> Self {
        Self::GLError(value)
    }
}

impl From<MatrixError> for RenderError {
    fn from(value: MatrixError) -> Self {
        Self::MatrixError(value)
    }
}