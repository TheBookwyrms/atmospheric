pub mod opengl;


mod opengl_helpers;
pub use opengl_helpers::camera;
pub use opengl_helpers::enums;
pub use opengl_helpers::image_processing;
pub use opengl_helpers::materials;

//pub mod enums;

//pub mod image_processing;

pub mod objects;

pub mod context;
pub mod glfw;
//pub mod materials;
//pub mod config;

pub mod modules;