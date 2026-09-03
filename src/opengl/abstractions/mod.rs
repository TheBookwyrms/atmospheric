mod with_object;
mod textures;
mod programs;
mod uniforms;

//pub use with_object::{WithObject, WithVao, WithVbo, WithEbo, WithVaoVbo, WithVaoEbo, WithTexture};
pub use with_object::{WithVao, WithVbo, WithEbo, WithVaoVbo, WithVaoEbo};
pub use textures::{Textures, TextureSetup, PreparedTexture};
pub use programs::{Programs, ProgramSelect};
pub use uniforms::Uniform;