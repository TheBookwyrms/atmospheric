use crate::gl;
use crate::gl::Gl;

use std::os::raw::c_void;

pub fn create_shader(opengl:&Gl, shader_type:gl::types::GLenum) -> u32 {
    unsafe { opengl.CreateShader(shader_type) }
}

pub fn shader_source(opengl:&Gl, shader_id:u32, source_ptr:&*const i8) {
    unsafe { opengl.ShaderSource(shader_id, 1, source_ptr, std::ptr::null()) }
}

pub fn compile_shader(opengl:&Gl, shader_id:u32) {
    unsafe { opengl.CompileShader(shader_id) }
}

pub fn get_shader_iv(opengl:&Gl, shader:u32, pname:u32, params:*mut i32) {
    unsafe { opengl.GetShaderiv(shader, pname, params) }
}

pub fn get_program_iv(opengl:&Gl, program:u32, pname:u32, params:*mut i32) {
    unsafe { opengl.GetProgramiv(program, pname, params) }
}

pub fn get_shader_info_log(opengl:&Gl, shader:u32, bufsize:i32, length:*mut i32, infolog:*mut i8) {
    unsafe { opengl.GetShaderInfoLog(shader, bufsize, length, infolog); }
}

pub fn get_program_info_log(opengl:&Gl, program:u32, bufsize:i32, length:*mut i32, infolog:*mut i8) {
    unsafe { opengl.GetProgramInfoLog(program, bufsize, length, infolog); }
}    

pub fn create_program(opengl:&Gl) -> u32 {
    unsafe { opengl.CreateProgram() }
}

pub fn attach_shader(opengl:&Gl, program_id:u32, shader_id:u32) {
    unsafe { opengl.AttachShader(program_id, shader_id) }
}

pub fn link_program(opengl:&Gl, program_id:u32) { unsafe { opengl.LinkProgram(program_id) }
}

pub fn delete_shader(opengl:&Gl, shader_id:u32) { unsafe { opengl.DeleteShader(shader_id) }
}

pub fn detach_shader(opengl:&Gl, program_id:u32, shader_id:u32) {
    unsafe { opengl.DetachShader(program_id, shader_id) }
}

pub fn use_program(opengl:&Gl, program_id:u32) {
    unsafe { opengl.UseProgram(program_id) }
}

pub fn get_uniform_location(opengl:&Gl, program_id:u32, uniform_name:*const i8) -> i32 {
    unsafe { opengl.GetUniformLocation(program_id, uniform_name) }
}

pub fn set_uniform_int(opengl:&Gl, uniform_location:i32, int:*const i32) {
    unsafe { opengl.Uniform1i(uniform_location, *int) }
}

pub fn set_uniform_float(opengl:&Gl, uniform_location:i32, float:*const f32) {
    unsafe { opengl.Uniform1f(uniform_location, *float) }
}

pub fn set_uniform_vec3(opengl:&Gl, uniform_location:i32, vec3_ptr:*const f32) {
    unsafe { opengl.Uniform3fv(uniform_location, 1, vec3_ptr) }
}

pub fn set_uniform_mat4(opengl:&Gl, uniform_location:i32, mat4_ptr:*const f32) {
    unsafe { opengl.UniformMatrix4fv(uniform_location, 1, gl::TRUE, mat4_ptr) }
}

pub fn gen_vertex_arrays(opengl:&Gl) -> u32 {
    let mut vao = 0;
    unsafe { opengl.GenVertexArrays(1, &mut vao) }
    vao
}

pub fn gen_buffers(opengl:&Gl) -> u32 {
    let mut vbo = 0;
    unsafe { opengl.GenBuffers(1, &mut vbo) }
    vbo
}

pub fn bind_vertex_array(opengl:&Gl, vao:u32) {
    unsafe { opengl.BindVertexArray(vao) }
}

pub fn bind_buffer(opengl:&Gl, target:gl::types::GLenum, buffer:u32) {
    unsafe { opengl.BindBuffer(target, buffer) }
}

pub fn buffer_data(
    opengl:&Gl,
    target:gl::types::GLenum,
    size:gl::types::GLsizeiptr,
    data_ptr:*const gl::types::GLvoid,
    draw_type:gl::types::GLenum
) {
    unsafe { opengl.BufferData(target, size, data_ptr, draw_type) }
}


pub fn enable_vertex_attrib_array(opengl:&Gl, layout_location:u32) {
    unsafe { opengl.EnableVertexAttribArray(layout_location) }
}

pub fn vertex_attrib_pointer(
    opengl:&Gl,
    layout_location:u32,
    num_items_in_location:i32,
    type_:gl::types::GLenum,
    normalized:gl::types::GLboolean,
    stride_between_points:i32,
    offset:*const c_void) {
    unsafe {
        opengl.VertexAttribPointer(
            layout_location,
            num_items_in_location,
            type_,
            normalized,
            stride_between_points,
            offset)
    }
    
}

pub fn buffer_sub_data(opengl:&Gl, target:gl::types::GLenum, size:isize, data:*const c_void) {
    unsafe { opengl.BufferSubData(target, 0, size, data)}
}

pub fn point_size(opengl:&Gl, size:f32) {
    unsafe { opengl.PointSize(size) }
}

pub fn draw_arrays(opengl:&Gl, mode:gl::types::GLenum, num_shapes:i32) {
    unsafe { opengl.DrawArrays(mode, 0, num_shapes) }
}

pub fn draw_elements(opengl:&Gl, mode:gl::types::GLenum, num_indices: i32) {
    unsafe { opengl.DrawElements(mode, num_indices, gl::UNSIGNED_INT, 0 as *const c_void); }
}

pub fn viewport(opengl:&Gl, x_low:i32, y_low:i32, x_high:i32, y_high:i32) {
    unsafe { opengl.Viewport(x_low, y_low, x_high, y_high);}
}

pub fn clear_colour(opengl:&Gl, red:f32, green:f32, blue:f32, alpha:f32) {
    unsafe { opengl.ClearColor(red, green, blue, alpha) }
}

pub fn clear(opengl:&Gl, mask:gl::types::GLenum) {
    unsafe { opengl.Clear(mask) }
}

pub fn enable(opengl:&Gl, cap:gl::types::GLenum) {
    unsafe { opengl.Enable(cap) }
}

pub fn blendfunc(opengl:&Gl, sfactor:gl::types::GLenum, dfactor:gl::types::GLenum) {
    unsafe {
        opengl.BlendFunc(sfactor, dfactor)
    }
}




pub fn gen_textures(opengl:&Gl) -> u32 {
    let mut texture = 0;
    unsafe { opengl.GenTextures(1, &mut texture) }
    texture
}

pub fn active_texture(opengl:&Gl, texture: u32) {
    unsafe { opengl.ActiveTexture(texture); }
}

pub fn bind_texture(opengl:&Gl, target: u32, texture: u32) {
    unsafe { opengl.BindTexture(target, texture) }
}

pub fn tex_image_2d(
    opengl:&Gl, target: u32, level: i32,
    internalformat: i32, width: i32, height: i32,
    format: u32, type_: u32, pixels: *const c_void
) {
    unsafe { opengl.TexImage2D(
                        target, level, internalformat,
                        width, height, 0, // border is always 0, according to Learn OpenGL.com
                        format, type_, pixels
                    )
            }
}

pub fn tex_parameter_i(opengl:&Gl, target: u32, pname: u32, param: i32) {
    unsafe { opengl.TexParameteri(target, pname, param) }
}

pub fn tex_parameter_f(opengl:&Gl, target: u32, pname: u32, param: f32) {
    unsafe { opengl.TexParameterf(target, pname, param) }
}

pub fn tex_parameter_fv(opengl:&Gl, target: u32, pname: u32, params: *const f32) {
    unsafe { opengl.TexParameterfv(target, pname, params); }
}

pub fn generate_mipmap(opengl:&Gl, target: u32) {
    unsafe { opengl.GenerateMipmap(target) }
}