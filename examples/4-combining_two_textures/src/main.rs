#![allow(warnings)]

// uncomment for release
// #![windows_subsystem = "windows"]


mod cube;


use atmospheric::image_processing;
use atmospheric::enums;
use atmospheric::opengl;
use atmospheric::opengl::abstractions::{self, TextureSetup, Textures, WithObject};
use atmospheric::enums::{InternalFormat, TextureMagFilter, TextureMinFilter, TextureWrapping};
use atmospheric::opengl::{gl, intermediate_opengl, raw_opengl};
use atmospheric::enums::ContextError;
use atmospheric::enums::ImageFormat;
use atmospheric::context::Context;
use atmospheric::enums::{DataFormat, DrawCall, DrawMode, GlError, OpenglTexture, ProgramSelect, TextureTarget, UniformType};
//use numeracy::matrices::Matrix;
use numeracy::matrices2::Matrix;


use std::ffi::{CStr, CString};
use std::io::Read;
use std::os::raw::c_void;

use crate::image_processing::Image;

fn error(msg:String) {
    let a = true;
    let _b = match a {
        true =>Err(msg),
        false =>Ok(msg),
    }.unwrap();
}


fn main() -> Result<(), ContextError> {

    let texture_triangle = Matrix::from_2darray([
          // positions      // texture coords
        [ 35.,  35., 0.0,   5.0, 5.0],   // top right
        [ 35., -35., 0.0,   5.0, 0.0],   // bottom right
        [-35., -35., 0.0,   0.0, 0.0],   // bottom left
        [-35.,  35., 0.0,   0.0, 5.0],   // top left 
    ]);
    let triangle_indices = Matrix::from_2darray([
        [0, 1, 3],
        [1, 2, 3],
    ]);


    let mut render = Context::default()?;
    render.setup_render();


    let (tex_vao, tex_vbo, tex_ebo) = render.create_vao_vbo_ebo(&texture_triangle, &triangle_indices, DataFormat::Position3Texture2)?;
    
    

    let awesomeface = Image::decode_from_path("images/awesomeface.png", ImageFormat::PNG, true);
    let bluefaces   = Image::decode_from_path("images/bluefaces.png", ImageFormat::PNG, true);


    let prepared_bluefaces = TextureSetup::get(
            &render.window.opengl, TextureTarget::Texture2D,
            bluefaces)
            .set_texture_image_and_mipmap(0)
            .set_filters(TextureMinFilter::LinearMipmapNearest, TextureMagFilter::Linear)
            .set_st_wrapping(TextureWrapping::MirroredRepeat, TextureWrapping::MirroredRepeat)
            .get_prepared_texture()?;

    let prepared_awesomeface = TextureSetup::get(
            &render.window.opengl, TextureTarget::Texture2D,
            awesomeface)
            .set_texture_image_and_mipmap(0)
            .set_filters(TextureMinFilter::LinearMipmapNearest, TextureMagFilter::Linear)
            .set_st_wrapping(TextureWrapping::Repeat, TextureWrapping::Repeat)
            .get_prepared_texture()?;


        let vertex_text   = std::fs::read("src/two_texture_vertex.glsl").unwrap().iter().map(|a| *a as char).collect::<String>();
        let fragment_text = std::fs::read("src/two_texture_fragment.glsl").unwrap().iter().map(|a| *a as char).collect::<String>();
        let shader_id = render.compile_custom_program(vertex_text.as_str(), fragment_text.as_str())?;

    while !render.render_over() {
        render.begin_render_actions()?;

        render.use_custom_program(shader_id);
        render.set_orthographic_camera_uniforms()?;

        
        &render.textures.activate(
            &render.window.opengl, OpenglTexture::Texture0, &prepared_bluefaces, &render.programs
        )?;
        &render.textures.activate(
            &render.window.opengl, OpenglTexture::Texture1, &prepared_awesomeface, &render.programs
        )?;
    
        let with_relevant = WithObject::existing(&render.window.opengl, enums::Object::VAO, tex_vao, DataFormat::Position3Texture2)
                                                        .add(enums::Object::EBO, tex_ebo)?;
        render.programs.draw(with_relevant, DrawCall::Elements, DrawMode::GlTriangles, &triangle_indices)?;


        render.end_render_actions()?;
    }

    Ok(())
}