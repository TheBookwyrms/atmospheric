// gets gl from the build.rs and khronos API
pub mod gl;

// provides handles for the base opengl API
pub(crate) mod raw_opengl;

// provides safe(r) handles for the opengl API exposed in raw_opengl
pub(crate) mod intermediate_opengl;

//pub mod abstractions;

pub mod abstractions;