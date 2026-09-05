extern crate gl_generator;

use gl_generator::{Registry, Api, Profile, Fallbacks};
use std::collections::HashSet;
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



fn get_shader_file_name(full_path:&str) -> &str {
    #[cfg(target_os = "linux")]
    let split = full_path.split(r#"/"#).collect::<Vec<&str>>();


    #[cfg(target_os = "windows")]
    let split = shader_path_str.split(r#"\"#).collect::<Vec<&str>>();

    //let split = shader_path_str.split(r#"\"#).collect::<Vec<&str>>();
    let shader_name = split[split.len()-1].split(".").collect::<Vec<&str>>()[0];

    shader_name
}

fn get_file_name_and_extension(full_path:&str) -> &str {
    #[cfg(target_os = "linux")]
    let split = full_path.split(r#"/"#).collect::<Vec<&str>>();


    #[cfg(target_os = "windows")]
    let split = shader_path_str.split(r#"\"#).collect::<Vec<&str>>();

    let file = split[split.len()-1];

    file
}

fn capitalise_first_letter(s:&str) -> String {
    format!("{}{}", (&s[..1].to_string()).to_uppercase(), &s[1..])
}

fn uppercase(s: &str) -> String {
    s.to_uppercase()
}

fn lowercase(s: &str) -> String {
    s.to_lowercase()
}

fn get_shader_name_no_type_with_fn(shader_words:&Vec<&str>, f:impl Fn(&str) -> String, sep:&str) -> String {
    let shader_name  = shader_words.iter().map(|w| f(w)).collect::<Vec<String>>().join("intermediate");
    let shader_name_no_type  = shader_name.split("intermediate").collect::<Vec<&str>>()[..shader_words.len()-1].join(sep);
    shader_name_no_type
}


fn main() -> Result<(), BuildError> {
    
    // setting output directories
    let project_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let out_dir = env::var("OUT_DIR")?;
    



    // setting file for opengl rust bindings
    let gl_destination_path = Path::new(&out_dir).join("gl_bindings.rs");
    let mut opengl_api_file = File::create(&gl_destination_path)?;

    // writing bindings for opengl rust
    Registry::new(Api::Gl, (3, 3), Profile::Core, Fallbacks::All, [])
    .write_bindings(gl_generator::StructGenerator, &mut opengl_api_file)?;


    
    // setting file for shader text
    let shaders_destination_path = Path::new(&out_dir).join("shaders_glsl.rs");
    let mut shaders_file = File::create(&shaders_destination_path)?;


    let mut shaders_dir = out_dir.clone();
    shaders_dir.push_str(r#"\shaders_glsl"#);
    let project_folder = Path::new(project_dir.as_str());

    let mut shader_names_hashset = HashSet::new();

    let paths = fs::read_dir(&project_folder.join("src").join("shaders_glsl"))?;
    for path in paths {
        let path_buffer = path?.path();

        let _shader_path_string = path_buffer.into_os_string().into_string()?;
        let shader_path_str = _shader_path_string.as_str();


        let shader_name = get_shader_file_name(shader_path_str);
        let shader_name_words = shader_name.split("_").collect::<Vec<&str>>();

        //let shader_name_upper  = shader_name_words.iter().map(|w| w.to_uppercase()).collect::<Vec<String>>().join("_");
        //let shader_upper  = shader_name_upper.split("_").collect::<Vec<&str>>()[..shader_name_words.len()-1].join("_");
        let shader_upper = get_shader_name_no_type_with_fn(&shader_name_words, uppercase, "_");

        let shader_ending  = &shader_name_words.iter().map(|w| w.to_uppercase()).collect::<Vec<String>>()[shader_name_words.len()-1];

        //let shader_name_lower  = shader_name_words.iter().map(|w| w.to_lowercase()).collect::<Vec<String>>().join("_");
        //let shader_lower_no_type  = shader_name_lower.split("_").collect::<Vec<&str>>()[..shader_name_words.len()-1].join("_");
        let shader_lower_no_type = get_shader_name_no_type_with_fn(&shader_name_words, lowercase, "_");
        let shader_lower  = [shader_lower_no_type, String::from("shader")].join("_");

        //let shader_name_pascal  = shader_name_words.iter().map(|w| capitalise_first_letter(w)).collect::<Vec<String>>().join("intermediate");
        //let shader_pascal  = shader_name_pascal.split("intermediate").collect::<Vec<&str>>()[..shader_name_words.len()-1].join("");
        let shader_pascal = get_shader_name_no_type_with_fn(&shader_name_words, capitalise_first_letter, "");

        shader_names_hashset.insert((shader_pascal, shader_upper.clone(), shader_lower));




        let shader_let_statement = format!(r##"pub const {shader_upper}_{shader_ending} : &'static str = ""##);
        //let shader_let_statement = format!(r##"pub const {shader_name_upper_str} : &'static str = ""##);
        shaders_file.write(shader_let_statement.as_bytes())?;

        let mut shader_text = String::new();
        File::open(shader_path_str)?.read_to_string(&mut shader_text)?;
        shaders_file.write(shader_text.as_bytes())?;

        shaders_file.write(r##"";"##.as_bytes())?;
    }








    // setting file for programs enum and struct
    let programs_path = Path::new(&out_dir).join("programs.rs");
    let mut programs_file = File::create(&programs_path)?;


    //programs_file.write(r##"
    //    #[cfg(target_os = "linux")]
    //    include!(concat!(env!("OUT_DIR"), "/shaders_glsl.rs"));
    //    #[cfg(target_os = "windows")]
    //    include!(concat!(env!("OUT_DIR"), "\\shaders_glsl.rs"));
    //"##.as_bytes())?;




    programs_file.write(r##"
        #[derive(Debug, PartialEq, Clone, Copy)]
        pub enum ProgramSelect {
        Custom(u32),
    "##.as_bytes())?;
    for (shader_pascal, _shader_upper, _shader_lower) in shader_names_hashset.clone() {
        programs_file.write(format!("Select{},", shader_pascal).as_bytes())?;
    }
    programs_file.write("}".as_bytes())?;

    // the old version of the above
    // #[derive(Debug, PartialEq, Clone, Copy)]
    // pub enum ProgramSelect {
    //     SelectPhongOrthographic,
    //     SelectSimpleOrthographic,
    //     SelectSimpleTexture,
    //     SelectPhongTexture,
    //     Custom(u32)
    // }












    programs_file.write(r##"
        #[derive(Clone, Copy)]
        pub struct Programs {
    "##.as_bytes())?;
    for (_shader_pascal, _shader_upper, shader_lower) in shader_names_hashset.clone() {
        programs_file.write(format!("pub {}:u32,", shader_lower).as_bytes())?;
    }
    programs_file.write(r##"
        pub current_program:Option<u32>,
        pub current_program_type:Option<ProgramSelect>,
    }
    "##.as_bytes())?;

    // the old version of the above
    // #[derive(Clone, Copy)]
    // pub struct Programs {
    //     pub simple_orthographic_shader:u32,
    //     pub blinn_phong_orthographic_shader:u32,
    //     pub simple_texture_shader:u32,
    //     pub phong_texture_shader:u32,
    //     pub current_program:Option<u32>,
    //     pub current_program_type:Option<ProgramSelect>,
    // }









    programs_file.write("impl Programs {".as_bytes())?;
    programs_file.write(r##"
        pub fn compile_all(opengl:&Gl, max_lights:&LightCounter) -> Result<Programs, GlError> {
        Ok(
        Programs {
    "##.as_bytes())?;
    for (shader_pascal, _shader_upper, shader_lower) in shader_names_hashset.clone() {
        programs_file.write(format!("
        {shader_lower}:Programs::compile_program_from_select(opengl, ProgramSelect::Select{}, max_lights)?,
        ", shader_pascal).as_bytes())?;
    }
    programs_file.write(r##"
                current_program:None, current_program_type:None
            }
        )
    }
    "##.as_bytes())?;

    // the old version of the above
    // pub fn compile_all(opengl:&Gl, max_lights:&LightCounter) -> Result<Programs, GlError> {
    //     let simple_orthographic_shader = Programs::compile_program_from_select(opengl, ProgramSelect::SelectSimpleOrthographic, max_lights)?;
    //     let blinn_phone_orthographic_shader = Programs::compile_program_from_select(opengl, ProgramSelect::SelectPhongOrthographic, max_lights)?;
    //     let simple_texture_shader = Programs::compile_program_from_select(opengl, ProgramSelect::SelectSimpleTexture, max_lights)?;
    //     let phong_texture_shader = Programs::compile_program_from_select(opengl, ProgramSelect::SelectPhongTexture, max_lights)?;
    // 
    //     Ok(Programs { simple_orthographic_shader, blinn_phong_orthographic_shader: blinn_phone_orthographic_shader,
    //                   simple_texture_shader, phong_texture_shader,
    //                   current_program:None, current_program_type:None })
    // }






    programs_file.write(r##"
        pub fn compile_program_from_select(opengl:&Gl, program_type:ProgramSelect, max_lights:&LightCounter) -> Result<u32, GlError> {
        
        let dir_max   = max_lights.get_light_count(LightSourceForm::Directional);
        let point_max = max_lights.get_light_count(LightSourceForm::Point);
        let spot_max  = max_lights.get_light_count(LightSourceForm::Spot);

        match program_type {"##.as_bytes()
    )?;
    for (shader_pascal, shader_upper, _shader_lower) in shader_names_hashset.clone() {
        //println!("{}, {}", shader_pascal_case, shader_upper_no_vertex);
        //panic!();
        let component1 = format!("ProgramSelect::Select{} ", shader_pascal);
        let component2 = "=> {";
        let component3 = format!(r##"let vertex_text   = {}_VERTEX;"##,    shader_upper);
        let component4 = format!(r##"let fragment_text   = {}_FRAGMENT"##, shader_upper);
        let component5 = r##"
        .replace("find_and_replace_with_max_number_of_point_lights", &point_max.to_string())
        .replace("find_and_replace_with_max_number_of_directional_lights", &dir_max.to_string())
        .replace("find_and_replace_with_max_number_of_spot_lights", &spot_max.to_string());
        let shader_id = Programs::compile_program_from_text(
            opengl, &vertex_text, &fragment_text
        )?;
        Ok(shader_id)
        },"##;
        programs_file.write([component1.as_str(), component2, component3.as_str(), component4.as_str(), component5].join("").as_bytes())?;
    }
    programs_file.write(r##"
            ProgramSelect::Custom(_) => Err(GlError::InvalidCustomProgramSelect)
        }
    }"##.as_bytes())?;
    //programs_file.write("}".as_bytes())?; / for impl programs wrong?
    
    // the old version of the above
    //pub fn compile_program_from_select(opengl:&Gl, program_type:ProgramSelect, max_lights:&LightCounter) -> Result<u32, GlError> {
    //    
    //    let dir_max   = max_lights.get_light_count(LightSourceForm::Directional);
    //    let point_max = max_lights.get_light_count(LightSourceForm::Point);
    //    let spot_max  = max_lights.get_light_count(LightSourceForm::Spot);
    //
    //    match program_type {
    //        ProgramSelect::SelectBlinnPhongOrthographic => {
    //            let vertex_text   = BLINN_PHONG_ORTHOGRAPHIC_VERTEX;
    //            let fragment_text = BLINN_PHONG_ORTHOGRAPHIC_FRAGMENT
    //            .replace("find_and_replace_with_max_number_of_point_lights", &point_max.to_string())
    //            .replace("find_and_replace_with_max_number_of_directional_lights", &dir_max.to_string())
    //            .replace("find_and_replace_with_max_number_of_spot_lights", &spot_max.to_string());
    //            let shader_id = Programs::compile_program_from_text(
    //                opengl, vertex_text, &fragment_text
    //            )?;
    //            Ok(shader_id)
    //        },
    //        ProgramSelect::SelectSimpleOrthographic => {
    //            let vertex_text   = SIMPLE_ORTHOGRAPHIC_VERTEX;
    //            let fragment_text = SIMPLE_ORTHOGRAPHIC_FRAGMENT;
    //            let shader_id = Programs::compile_program_from_text(
    //                opengl, vertex_text, fragment_text
    //            )?;
    //            Ok(shader_id)
    //        },
    //        ProgramSelect::SelectSimpleTexture => {
    //            let vertex_text   = SIMPLE_TEXTURE_VERTEX;
    //            let fragment_text = SIMPLE_TEXTURE_FRAGMENT;
    //            let shader_id = Programs::compile_program_from_text(
    //                opengl, vertex_text, fragment_text
    //            )?;
    //            Ok(shader_id)
    //        },
    //        ProgramSelect::SelectPhongTexture => {
    //            let vertex_text   = PHONG_TEXTURE_VERTEX;
    //            let fragment_text = PHONG_TEXTURE_FRAGMENT
    //            .replace("find_and_replace_with_max_number_of_point_lights", &point_max.to_string())
    //            .replace("find_and_replace_with_max_number_of_directional_lights", &dir_max.to_string())
    //            .replace("find_and_replace_with_max_number_of_spot_lights", &spot_max.to_string());
    //            let shader_id = Programs::compile_program_from_text(
    //                opengl, vertex_text, &fragment_text
    //            )?;
    //            Ok(shader_id)
    //        },
    //        ProgramSelect::Custom(_) => Err(GlError::InvalidCustomProgramSelect)
    //    }
    //}



    programs_file.write(r##"
        pub fn use_program(&mut self, opengl:&Gl, program:ProgramSelect) -> Result<(), GlError> {
        match program {
    "##.as_bytes())?;
    for (shader_pascal, _shader_upper, shader_lower) in shader_names_hashset.clone() {
        programs_file.write(format!(r##"
            ProgramSelect::Select{shader_pascal} => {{
                intermediate_opengl::use_program(opengl, self.{shader_lower})?;
                self.current_program = Some(self.{shader_lower});
                self.current_program_type = Some(program);
                Ok(())
            }},
        "##).as_bytes())?;
    }
    programs_file.write(r##"
        ProgramSelect::Custom(id) => {
            intermediate_opengl::use_program(opengl, id)?;
            self.current_program = Some(id);
            self.current_program_type = Some(program);
            Ok(())
        },
    "##.as_bytes())?;
    programs_file.write(r##"}"##.as_bytes())?;
    programs_file.write(r##"}"##.as_bytes())?;

    programs_file.write("}".as_bytes())?;

    // the old version of the above
    // pub fn use_program(&mut self, opengl:&Gl, program:ProgramSelect) -> Result<(), GlError> {
    //     match program {
    //         ProgramSelect::SelectSimpleOrthographic => {
    //             intermediate_opengl::use_program(opengl, self.simple_orthographic_shader)?;
    //             self.current_program = Some(self.simple_orthographic_shader);
    //             self.current_program_type = Some(program);
    //             Ok(())
    //         },
    //         ProgramSelect::SelectPhongOrthographic => {
    //             intermediate_opengl::use_program(opengl, self.blinn_phong_orthographic_shader)?;
    //             self.current_program = Some(self.blinn_phong_orthographic_shader);
    //             self.current_program_type = Some(program);
    //             Ok(())
    //         },
    //         ProgramSelect::SelectSimpleTexture => {
    //             intermediate_opengl::use_program(opengl, self.simple_texture_shader)?;
    //             self.current_program = Some(self.simple_texture_shader);
    //             self.current_program_type = Some(program);
    //             Ok(())
    //         },
    //         ProgramSelect::SelectPhongTexture => {
    //             intermediate_opengl::use_program(opengl, self.phong_texture_shader)?;
    //             self.current_program = Some(self.phong_texture_shader);
    //             self.current_program_type = Some(program);
    //             Ok(())
    //         },
    //         ProgramSelect::Custom(id) => {
    //             intermediate_opengl::use_program(opengl, id)?;
    //             self.current_program = Some(id);
    //             self.current_program_type = Some(program);
    //             Ok(())
    //         }
    //     }
    // }











    // setting file for shader text
    let compiled_assets_path = Path::new(&out_dir).join("compiled_assets.rs");
    let mut compiled_assets_file = File::create(&compiled_assets_path)?;

    compiled_assets_file.write(format!(r##"use crate::opengl_helpers::enums::ImageFormat;"##).as_bytes())?;


    let paths = fs::read_dir(&project_folder.join("src").join("compiled_assets"))?;
    for path in paths {

        
        let path_buffer = path?.path();

        let _asset_path_string = path_buffer.into_os_string().into_string()?;
        let asset_path_str = _asset_path_string.as_str();

        let _file_split = asset_path_str.split(".").collect::<Vec<&str>>();
        let _file_extension = _file_split[_file_split.len()-1].to_uppercase();
        let file_extension_str = _file_extension.as_str();

        let file_name = get_file_name_and_extension(asset_path_str)
                                    .replace(" ", "_")
                                    .replace("-", "_")
                                    .replace(".", "_");
        let file_name_caps = file_name.split("_").map(|w| uppercase(w)).collect::<Vec<String>>().join("_");


        //let mut asset = Vec::new();
        //File::open(asset_path_str)?.read_to_end(&mut asset)?;
        //let a2 = asset.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(", ");
        //compiled_assets_file.write(format!(r##"pub const VEC_{file_name_caps} : Vec<u8> = vec![{a2}];"##).as_bytes())?;


        let mut asset = String::new();
        File::open(asset_path_str)?.read_to_string(&mut asset)?;
        let s = asset.as_str();
        compiled_assets_file.write(format!(r##"pub const FILE_TEXT_{file_name_caps} : &str = "{s}";"##).as_bytes())?;
        match file_extension_str {
            "PPM" => {
                compiled_assets_file.write(format!(r##"pub static {file_name_caps} : LazyLock<Image> = LazyLock::new(|| Image::decode_from_bytes(FILE_TEXT_{file_name_caps}.as_bytes(), ImageFormat::PPMP3, true));"##).as_bytes())?;
            },
            _ => {},
        }
    }













    Ok(())
}
