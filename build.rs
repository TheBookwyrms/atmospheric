extern crate gl_generator;

use gl_generator::{Registry, Api, Profile, Fallbacks};
use std::env;
use std::env::VarError;
use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::io::Read;

#[derive(Debug)]
pub enum BuildError {
    VarError(VarError),
    IOError(std::io::Error),
    OSStringError(OsString),
}
impl From<VarError> for BuildError {
    fn from(value: VarError) -> Self {
        Self::VarError(value)
    }
}
impl From<std::io::Error> for BuildError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}
impl From<OsString> for BuildError {
    fn from(value: OsString) -> Self {
        Self::OSStringError(value)
    }
}


fn main() -> Result<(), BuildError> {


    println!("cargo:rustc-link-lib=static=glfw3");
    

    let project_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let out_dir = env::var("OUT_DIR")?;
    
    let gl_destination_path = Path::new(&out_dir).join("gl_bindings.rs");
    
    let mut opengl_api_file = File::create(&gl_destination_path)?;


    Registry::new(Api::Gl, (3, 3), Profile::Core, Fallbacks::All, [])
    .write_bindings(gl_generator::StructGenerator, &mut opengl_api_file)?;


    

    let shaders_destination_path = Path::new(&out_dir).join("shaders_glsl.rs");
    
    let mut shaders_file = File::create(&shaders_destination_path)?;


    let mut shaders_dir = out_dir.clone();
    shaders_dir.push_str(r#"\shaders_glsl"#);
    let project_folder = Path::new(project_dir.as_str());

    let paths = fs::read_dir(&project_folder.join("shaders_glsl"))?;
    for path in paths {

        let path_buffer = path?.path();

        let shader_path_string = path_buffer.clone().into_os_string().into_string()?;
        let shader_path_str = shader_path_string.as_str();

        let split = shader_path_str.split(r#"\"#).collect::<Vec<&str>>();
        let shader_name = split[split.len()-1].split(".").collect::<Vec<&str>>()[0];
        let shader_name_upper = shader_name.to_uppercase();
        let shader_name_upper_str = shader_name_upper.as_str();



        let shader_let_statement = format!(r##"pub const {shader_name_upper_str} : &'static str = ""##);
        shaders_file.write(shader_let_statement.as_bytes())?;

        let mut shader_text = String::new();
        File::open(shader_path_str)?.read_to_string(&mut shader_text)?;
        shaders_file.write(shader_text.as_bytes())?;

        shaders_file.write(r##"";"##.as_bytes())?;
    }

    Ok(())
}