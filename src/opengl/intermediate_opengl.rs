use crate::gl;
use crate::gl::Gl;
use crate::raw_opengl;
use crate::enums::{
    ArrayObject, BlendFunc, BufferBit, BufferObject, DrawMode, DrawType, GlEnable, GlError, InternalFormat, Object, ShaderType, TextureFilter, TextureMagFilter, TextureMinFilter, TextureTarget, TextureWrap, TextureWrapping, UniformType
};

use std::ffi::CString;
use std::os::raw::{self, c_void};




pub fn load_opengl_with<T:FnMut(&'static str) -> *const c_void>(loadfn: T) -> Gl {
    gl::Gl::load_with(loadfn)
}

pub fn generate(opengl:&Gl, object:Object) -> u32 {
    match object {
        Object::VBO => { raw_opengl::gen_buffers(opengl) },
        Object::VAO => { raw_opengl::gen_vertex_arrays(opengl) },
        Object::EBO => { raw_opengl::gen_buffers(opengl) },
        Object::Texture2D => { raw_opengl::gen_textures(opengl) }
    }
}


pub fn clear_colour(opengl:&Gl, r:f32, g:f32, b:f32, a:f32) -> Result<(), GlError> {
    let validity = vec![r, g, b, a].into_iter().filter(|c| 0.0<=*c && *c<=1.0).count();
    match validity {
        4 => Ok(raw_opengl::clear_colour(opengl, r, g, b, a)),
        _ => Err(GlError::InvalidColour(r, g, b, a)),
    }    
}

pub fn clear(opengl:&Gl, masks:Vec<BufferBit>) {
    let mut start = 0;
    for mask in masks.clone() {
        start = match mask {
            BufferBit::ColourBufferBit => start | gl::COLOR_BUFFER_BIT,
            BufferBit::DepthBufferBit  => start | gl::DEPTH_BUFFER_BIT,
        };
    }
    raw_opengl::clear(opengl, start);
}

pub fn gl_enable(opengl:&Gl, setting:GlEnable) {
    match setting {
        GlEnable::DepthTest => raw_opengl::enable(opengl, gl::DEPTH_TEST),
        GlEnable::Multisample => raw_opengl::enable(opengl, gl::MULTISAMPLE),
        GlEnable::Blend => raw_opengl::enable(opengl, gl::BLEND),
    }
}

pub fn gl_blendfunc(opengl:&Gl, setting:BlendFunc) {
    match setting {
        BlendFunc::SRCAlphaOneMinusSRCAlpha => {
            raw_opengl::blendfunc(opengl, gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        },
    }
}

pub fn create_shader(opengl:&Gl, shader_type:ShaderType) -> Result<u32, GlError> {
    match shader_type {
        ShaderType::VertexShader   => Ok(raw_opengl::create_shader(opengl, gl::VERTEX_SHADER)),
        ShaderType::FragmentShader => Ok(raw_opengl::create_shader(opengl, gl::FRAGMENT_SHADER)),
        ShaderType::ShaderProgram => Err(GlError::InvalidShaderType(ShaderType::ShaderProgram))
    }
}


pub fn shader_source(opengl:&Gl, shader_id:u32, source:&str) -> Result<(), GlError> {
    match CString::new(source) {
        Ok(binding) => {
            let source_ptr = binding.as_c_str().as_ptr();
            raw_opengl::shader_source(opengl, shader_id, &source_ptr);
            Ok(())
        },
        Err(error) => Err(GlError::CStringError(error)),
    }
}

pub fn create_shader_variant(opengl:&Gl, str_text:&str, shader_type:ShaderType) -> Result<u32, GlError> {
    let shader_id = create_shader(opengl, shader_type)?;

    shader_source(opengl, shader_id, str_text)?;
    raw_opengl::compile_shader(opengl, shader_id);

    get_compilation_error(opengl, shader_id, shader_type)?;

    Ok(shader_id)
}

pub fn use_program(opengl:&Gl, program:u32) -> Result<(), GlError> {
    match program {
        0 => Err(GlError::InvalidProgramID),
        n => Ok(raw_opengl::use_program(opengl, n)),
    }
}

pub fn disuse_program(opengl:&Gl) {
    raw_opengl::use_program(opengl, 0);
}

pub fn remove_shader_variant(opengl:&Gl, program_id:u32, shader_id:u32) {
    raw_opengl::detach_shader(opengl, program_id, shader_id);
    raw_opengl::delete_shader(opengl, shader_id);
}

pub fn create_shader_program(opengl:&Gl, vertex_id:u32, fragment_id:u32) -> Result<u32, GlError> {
    let program_id = raw_opengl::create_program(opengl);
    
    raw_opengl::attach_shader(opengl, program_id,   vertex_id);
    raw_opengl::attach_shader(opengl, program_id, fragment_id);
    raw_opengl::link_program(opengl, program_id);

    get_compilation_error(opengl, program_id, ShaderType::ShaderProgram)?;

    Ok(program_id)
}




pub fn set_uniform<T:Clone>(opengl:&Gl, program_id:u32,
                    uniform_name:&str, uniform_type:UniformType,
                    value:*const T) -> Result<(), GlError> {
    let location_name = get_uniform_location(opengl, program_id, uniform_name)?;
    match uniform_type {
        UniformType::Int => raw_opengl::set_uniform_int(opengl, location_name, value as *const i32),
        UniformType::Float => raw_opengl::set_uniform_float(opengl, location_name, value as *const f32),
        UniformType::Vec3  => raw_opengl::set_uniform_vec3(opengl, location_name, value as *const f32),
        UniformType::Mat4  => raw_opengl::set_uniform_mat4(opengl, location_name, value as *const f32),
    }
    Ok(())
}

pub fn get_uniform_location(opengl:&Gl, program_id:u32, uniform_name:&str) -> Result<i32, GlError> {
    match CString::new(uniform_name) {
        Ok(cstring) => {
            let cname = cstring.as_bytes_with_nul().as_ptr() as *const i8;
            Ok(raw_opengl::get_uniform_location(opengl, program_id, cname))
        },
        Err(error) => Err(GlError::CStringError(error)),
    }
}


pub fn read_info_log_error(
    opengl:&Gl,
    iv_func: &dyn Fn(&Gl, u32, u32, *mut i32) -> (),
    log_func: &dyn Fn(&Gl, u32, i32, *mut i32, *mut i8) -> (),
    id:u32
) -> String {
        let mut log_len : gl::types::GLint = 0;
        iv_func(opengl, id, gl::INFO_LOG_LENGTH, &mut log_len);

        let mut buffer: Vec<u8> = Vec::with_capacity(log_len as usize + 1);
        buffer.extend([b' '].iter().cycle().take(log_len as usize));
        let error = unsafe { CString::from_vec_unchecked(buffer.clone()) };

        log_func(opengl,
                id, log_len, std::ptr::null_mut(), 
                error.as_ptr() as *mut gl::types::GLchar);
        let error_msg = error.to_string_lossy().into_owned();
        error_msg
}


pub fn get_compilation_error(opengl:&Gl, id:u32, shader_type:ShaderType) -> Result<(), GlError> {

    let mut success = 0; // this defaults error unless it worked // 1 is good, 0 is bad
    let error_msg = match shader_type {
        ShaderType::VertexShader => {
            raw_opengl::get_shader_iv(opengl, id, gl::COMPILE_STATUS, &mut success);
            read_info_log_error(opengl, &raw_opengl::get_shader_iv,
                                &raw_opengl::get_shader_info_log, id)
        },
        ShaderType::FragmentShader => {
            raw_opengl::get_shader_iv(opengl, id, gl::COMPILE_STATUS, &mut success);
            read_info_log_error(opengl, &raw_opengl::get_shader_iv,
                                &raw_opengl::get_shader_info_log, id)
        },
        ShaderType::ShaderProgram => {
            raw_opengl::get_program_iv(opengl, id, gl::LINK_STATUS, &mut success);
            read_info_log_error(opengl, &raw_opengl::get_program_iv,
                                &raw_opengl::get_program_info_log, id)
        },
    };
    match success {
        0 => Err(GlError::CompilationSuccessFailed(error_msg)),
        1 => Ok(()),
        _ => Err(GlError::CompilationSuccessFailed("compilation_success is neither 1 nor 0".to_owned())),
    }
}




pub fn bind_buffer(opengl:&Gl, target:BufferObject, buffer:u32) {
    match target {
        BufferObject::VertexBufferObject => raw_opengl::bind_buffer(opengl, gl::ARRAY_BUFFER, buffer),
        BufferObject::ElementBufferObject => raw_opengl::bind_buffer(opengl, gl::ELEMENT_ARRAY_BUFFER, buffer),
    }
}

pub fn bind_vertex_array(opengl:&Gl, target:ArrayObject, object:u32) {
    match target {
        ArrayObject::VertexArrayObject => raw_opengl::bind_vertex_array(opengl, object),
    }
}

pub fn bind_texture(opengl:&Gl, texture_type:TextureTarget, texture:u32) {
    match texture_type {
        TextureTarget::Texture2D => raw_opengl::bind_texture(opengl, gl::TEXTURE_2D, texture),
    }
}

pub fn buffer_data(
    opengl:&Gl,
    target:BufferObject,
    size:gl::types::GLsizeiptr,
    data_ptr:*const gl::types::GLvoid,
    draw_type:DrawType,
) {
    let gl_target = match target {
        BufferObject::VertexBufferObject => gl::ARRAY_BUFFER,
        BufferObject::ElementBufferObject => gl::ELEMENT_ARRAY_BUFFER,
    };

    let gl_drawtype = match draw_type {
        DrawType::StaticDraw => gl::STATIC_DRAW,
        DrawType::StreamDraw => gl::STREAM_DRAW,
        DrawType::DynamicDraw => gl::DYNAMIC_DRAW,
    };

    raw_opengl::buffer_data(opengl, gl_target, size, data_ptr, gl_drawtype);

}

pub fn set_vertex_attrib_position_3(opengl:&Gl, loc:u32, len:i32, offset:i32, dtype_size:i32) {
    set_vertex_attrib(opengl, loc, 3, len, offset, dtype_size);
}

pub fn set_vertex_attrib_normal_3(opengl:&Gl, loc:u32, len:i32, offset:i32, dtype_size:i32) {
    set_vertex_attrib(opengl, loc, 3, len, offset, dtype_size);
}

pub fn set_vertex_attrib_colour_3(opengl:&Gl, loc:u32, len:i32, offset:i32, dtype_size:i32) {
    set_vertex_attrib(opengl, loc, 3, len, offset, dtype_size);
}

pub fn set_vertex_attrib_alpha_1(opengl:&Gl, loc:u32, len:i32, offset:i32, dtype_size:i32) {
    set_vertex_attrib(opengl, loc, 1, len, offset, dtype_size);
}

pub fn set_vertex_attrib_texture_2(opengl:&Gl, loc:u32, len:i32, offset:i32, dtype_size:i32) {
    set_vertex_attrib(opengl, loc, 2, len, offset, dtype_size);
}


pub fn set_vertex_attrib(opengl:&Gl, layout_location:u32, num_items:i32, stride:i32, offset:i32, dtype_size:i32){
//) -> Result<(), GlError>{
    
    let stride = stride * dtype_size;
    let offset = (offset * dtype_size) as *const c_void;

    //let n_per_vertice : i32 = 3;
    //let n_per_colour  : i32 = 3;
    //let n_per_opacity : i32 = 1;
    //let n_per_normal  : i32 = 3;
    //let len_ptr = n_per_vertice + n_per_colour +
    //                        n_per_opacity + if store_normals
    //                        {n_per_normal} else {0};
    //let stride = len_ptr * dtype_size;
    //let (num_items, offset) = match layout_location {
    //    0 => Ok((n_per_vertice, 0 as *const c_void)),
    //    1 => Ok(( n_per_colour , ((n_per_vertice) * dtype_size) as *const c_void)),
    //    2 => Ok((n_per_opacity, ((n_per_vertice + n_per_colour) * dtype_size) as *const c_void)),
    //    3 => if store_normals {
    //            Ok((n_per_normal, ((n_per_vertice + n_per_colour + n_per_opacity) * dtype_size) as *const c_void))
    //        } else {Err(GlError::InvalidLayoutLocation(3))},
    //    n => Err(GlError::InvalidLayoutLocation(n)),
    //}?;
    raw_opengl::enable_vertex_attrib_array(opengl, layout_location);
    raw_opengl::vertex_attrib_pointer(opengl, layout_location, num_items, gl::FLOAT, gl::FALSE, stride, offset);
    //Ok(())
}


pub fn buffer_sub_data(opengl:&Gl, target:BufferObject, size:isize, data:*const c_void) {
    match target {
        BufferObject::VertexBufferObject => raw_opengl::buffer_sub_data(opengl, gl::ARRAY_BUFFER, size, data),
        BufferObject::ElementBufferObject => raw_opengl::buffer_sub_data(opengl, gl::ELEMENT_ARRAY_BUFFER, size, data),
    }
}


pub fn draw_arrays(opengl:&Gl, mode:DrawMode, num_shapes:i32) {
    raw_opengl::point_size(opengl, 10.0);
    match mode {
        DrawMode::GlPoints =>    raw_opengl::draw_arrays(opengl, gl::POINTS, num_shapes),
        DrawMode::GlLines =>     raw_opengl::draw_arrays(opengl, gl::LINES, num_shapes),
        DrawMode::GlTriangles => raw_opengl::draw_arrays(opengl, gl::TRIANGLES, num_shapes),
        DrawMode::GlTriangleStrip => raw_opengl::draw_arrays(opengl, gl::TRIANGLE_STRIP, num_shapes),
    }
}


pub fn draw_elements(opengl:&Gl, mode:DrawMode, num_indices:i32) {
    raw_opengl::point_size(opengl, 10.0);
    match mode {
        DrawMode::GlPoints =>    raw_opengl::draw_elements(opengl, gl::POINTS, num_indices),
        DrawMode::GlLines =>     raw_opengl::draw_elements(opengl, gl::LINES, num_indices),
        DrawMode::GlTriangles => raw_opengl::draw_elements(opengl, gl::TRIANGLES, num_indices),
        DrawMode::GlTriangleStrip => raw_opengl::draw_elements(opengl, gl::TRIANGLE_STRIP, num_indices),
    }
}


pub fn viewport(opengl:&Gl, width:i32, height:i32) {
    raw_opengl::viewport(opengl, 0, 0, width, height);
}



pub fn texture_wrap(opengl:&Gl, target:TextureTarget, wrap:TextureWrap, wrapping:TextureWrapping) {
    let tex_target = match target {
        TextureTarget::Texture2D => gl::TEXTURE_2D
    };

    let wrap_on = match wrap {
        TextureWrap::S => gl::TEXTURE_WRAP_S,
        TextureWrap::T => gl::TEXTURE_WRAP_T,
        TextureWrap::R => gl::TEXTURE_WRAP_R,
    };

    match wrapping {
        TextureWrapping::Repeat => raw_opengl::tex_parameter_i(opengl, tex_target, wrap_on, gl::REPEAT as i32),
        TextureWrapping::MirroredRepeat => raw_opengl::tex_parameter_i(opengl, tex_target, wrap_on, gl::MIRRORED_REPEAT as i32),
        TextureWrapping::ClampToEdge => raw_opengl::tex_parameter_i(opengl, tex_target, wrap_on, gl::CLAMP_TO_EDGE as i32),
        TextureWrapping::ClampToBorder(r, g, b, a) => {
            raw_opengl::tex_parameter_i(opengl, tex_target, wrap_on, gl::CLAMP_TO_BORDER as i32);
            raw_opengl::tex_parameter_fv(opengl, tex_target, gl::TEXTURE_BORDER_COLOR, vec![r, g, b, a].as_ptr());
        },
    }
}

pub fn texture_min_filter(opengl:&Gl, texture:TextureTarget, filter:TextureMinFilter) {
    let tex_target = match texture {
        TextureTarget::Texture2D => gl::TEXTURE_2D
    };

    let filter_level = match filter {
        TextureMinFilter::NearestMipmapLinear  => gl::NEAREST_MIPMAP_LINEAR,
        TextureMinFilter::NearestMipmapNearest => gl::NEAREST_MIPMAP_NEAREST,
        TextureMinFilter::LinearMipmapLinear  => gl::LINEAR_MIPMAP_LINEAR,
        TextureMinFilter::LinearMipmapNearest => gl::LINEAR_MIPMAP_NEAREST,
        TextureMinFilter::Linear => gl::LINEAR,
        TextureMinFilter::Nearest => gl::NEAREST,
    };
    raw_opengl::tex_parameter_i(opengl, tex_target, gl::TEXTURE_MIN_FILTER, filter_level as i32);
}

pub fn texture_mag_filter(opengl:&Gl, texture:TextureTarget, filter:TextureMagFilter) {
    let tex_target = match texture {
        TextureTarget::Texture2D => gl::TEXTURE_2D
    };

    let filter_level = match filter {
        TextureMagFilter::Linear => gl::LINEAR,
        TextureMagFilter::Nearest => gl::NEAREST,
    };
    raw_opengl::tex_parameter_i(opengl, tex_target, gl::TEXTURE_MAG_FILTER, filter_level as i32);
}

pub fn texture_filter(opengl:&Gl, texture:TextureTarget, filter:TextureFilter) {
    let tex_target = match texture {
        TextureTarget::Texture2D => gl::TEXTURE_2D
    };

    match filter {
        TextureFilter::MinFilter(min) => {
            let filter_level = match min {
                TextureMinFilter::NearestMipmapLinear  => gl::NEAREST_MIPMAP_LINEAR,
                TextureMinFilter::NearestMipmapNearest => gl::NEAREST_MIPMAP_NEAREST,
                TextureMinFilter::LinearMipmapLinear  => gl::LINEAR_MIPMAP_LINEAR,
                TextureMinFilter::LinearMipmapNearest => gl::LINEAR_MIPMAP_NEAREST,
                TextureMinFilter::Linear => gl::LINEAR,
                TextureMinFilter::Nearest => gl::NEAREST,
            };
            raw_opengl::tex_parameter_i(opengl, tex_target, gl::TEXTURE_MIN_FILTER, filter_level as i32);
        },
        TextureFilter::MagFilter(mag) => {
            let filter_level = match mag {
                TextureMagFilter::Linear => gl::LINEAR,
                TextureMagFilter::Nearest => gl::NEAREST,
            };
            raw_opengl::tex_parameter_i(opengl, tex_target, gl::TEXTURE_MAG_FILTER, filter_level as i32);
        },
    }
}

pub fn texture_image(
    opengl:&Gl,
    target:TextureTarget,
    mipmap_level:i32,
    format:InternalFormat,
    width:i32, height:i32,
    pixels: *const u8
) {
    let colour_format = match format {
        InternalFormat::RGB => gl::RGB,
        InternalFormat::RGBA => gl::RGBA,
    };

    match target {
        TextureTarget::Texture2D => {
            raw_opengl::tex_image_2d(
                opengl, gl::TEXTURE_2D, mipmap_level,
                colour_format as i32, width, height,
                colour_format, gl::UNSIGNED_BYTE, pixels as *const c_void);
        }
    }
}

pub fn generate_mipmap(opengl:&Gl, texture:TextureTarget) {
    match texture {
        TextureTarget::Texture2D => raw_opengl::generate_mipmap(opengl, gl::TEXTURE_2D),
    }
}




pub fn texture_test_1(opengl:&Gl, width:*mut i32, height:*mut i32, pixels: *const u8) -> u32 {

    //let tex_id = raw_opengl::gen_textures(opengl);
    let tex_id = generate(opengl, Object::Texture2D);
    //raw_opengl::active_texture(opengl, gl::TEXTURE0);
    bind_texture(opengl, TextureTarget::Texture2D, tex_id);
    //raw_opengl::bind_texture(opengl, gl::TEXTURE_2D, tex_id);

    //raw_opengl::tex_parameter_i(opengl, gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::MIRRORED_REPEAT.try_into().unwrap());
    //raw_opengl::tex_parameter_i(opengl, gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::MIRRORED_REPEAT.try_into().unwrap());
    
    texture_image(
        opengl, TextureTarget::Texture2D, 0,
        InternalFormat::RGBA, width as i32, height as i32, pixels);
    generate_mipmap(opengl, TextureTarget::Texture2D);
    //raw_opengl::tex_parameter_i(opengl, gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT.try_into().unwrap());
    //raw_opengl::tex_parameter_i(opengl, gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT.try_into().unwrap());
    
    //texture_wrap(opengl, TextureTarget::Texture2D, TextureWrap::S, TextureWrapping::MirroredRepeat);
    texture_wrap(opengl, TextureTarget::Texture2D, TextureWrap::S, TextureWrapping::ClampToBorder(1.0, 1.0, 1.0, 1.0));
    texture_wrap(opengl, TextureTarget::Texture2D, TextureWrap::T, TextureWrapping::ClampToEdge);
    //raw_opengl::tex_parameter_i(opengl, gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE.try_into().unwrap());
    //raw_opengl::tex_parameter_i(opengl, gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE.try_into().unwrap());

    //raw_opengl::tex_parameter_i(opengl, gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_BORDER.try_into().unwrap());
    //raw_opengl::tex_parameter_i(opengl, gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_BORDER.try_into().unwrap());
    //raw_opengl::tex_parameter_fv(opengl, gl::TEXTURE_2D, gl::TEXTURE_BORDER_COLOR, vec![1.0, 1.0, 1.0, 1.0].as_ptr());



    texture_min_filter(opengl, TextureTarget::Texture2D, TextureMinFilter::LinearMipmapNearest);
    texture_mag_filter(opengl, TextureTarget::Texture2D, TextureMagFilter::Linear);


    // texture_filter(opengl, TextureTarget::Texture2D, TextureFilter::MinFilter(TextureMinFilter::LinearMipmapNearest));
    // //raw_opengl::tex_parameter_i(opengl, gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_NEAREST.try_into().unwrap());
    // //raw_opengl::tex_parameter_i(opengl, gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR.try_into().unwrap());
    // //raw_opengl::tex_parameter_i(opengl, gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST.try_into().unwrap());
    // texture_filter(opengl, TextureTarget::Texture2D, TextureFilter::MagFilter(TextureMagFilter::Linear));
    // //raw_opengl::tex_parameter_i(opengl, gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR.try_into().unwrap());
    
    //raw_opengl::tex_image_2d(
    //    opengl, gl::TEXTURE_2D, 0,
    //    gl::RGB.try_into().unwrap(), width as i32, height as i32,
    //    gl::RGB, gl::UNSIGNED_BYTE, pixels as *const c_void);

    //texture_image(
    //    opengl, TextureTarget::Texture2D, 0,
    //    InternalFormat::RGBA, width as i32, height as i32, pixels);
    //generate_mipmap(opengl, TextureTarget::Texture2D);

    //raw_opengl::tex_image_2d(
    //    opengl, gl::TEXTURE_2D, 0,
    //    gl::RGBA.try_into().unwrap(), width as i32, height as i32,
    //    gl::RGBA, gl::UNSIGNED_BYTE, pixels as *const c_void);

    //generate_mipmap(opengl, TextureTarget::Texture2D);
    //raw_opengl::generate_mipmap(opengl, gl::TEXTURE_2D);

    tex_id
}