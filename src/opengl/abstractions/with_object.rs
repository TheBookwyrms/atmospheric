use crate::opengl::gl::Gl;
use crate::enums::{
    ArrayObject, BufferObject, DataFormat,
    DrawCall, DrawMode, DrawType,
    GlError, Object, TextureTarget, UpdateVertexAttrib,
};
use crate::opengl::intermediate_opengl;

//use numeracy::matrices::Matrix;
use numeracy::matrices2::Matrix;

use std::os::raw::c_void;


#[derive(Debug)]
pub struct WithObject<'l> {
    opengl:&'l Gl,
    texture_type:Option<TextureTarget>,
    data_format:DataFormat,
    vao:u32,
    vbo:u32,
    ebo:u32,
    tex:u32,
}
impl WithObject<'_> {
    pub fn get_data_format(&self) -> DataFormat { self.data_format }
    pub fn get_vao(&self) -> u32 { self.vao }
    pub fn get_vbo(&self) -> u32 { self.vbo }
    pub fn get_ebo(&self) -> u32 { self.ebo }
    pub fn get_tex(&self) -> u32 { self.tex }

    pub fn add(mut self, object:Object, id:u32) -> Result<Self, GlError> {
        match object {
            Object::VBO => {
                if self.vbo != 0 { Err(GlError::ObjectAlreadyBound)? }
                intermediate_opengl::bind_buffer(self.opengl, BufferObject::VertexBufferObject, id);
                self.vbo = id;
            },
            Object::VAO => {
                if self.vao != 0 { Err(GlError::ObjectAlreadyBound)? }
                intermediate_opengl::bind_vertex_array(self.opengl, ArrayObject::VertexArrayObject, id);
                self.vao = id;
            },
            Object::EBO => {
                if self.ebo != 0 { Err(GlError::ObjectAlreadyBound)? }
                intermediate_opengl::bind_buffer(self.opengl, BufferObject::ElementBufferObject, id);
                self.ebo = id;
            },
            Object::Texture2D => {
                if self.tex != 0 { Err(GlError::ObjectAlreadyBound)? }
                intermediate_opengl::bind_texture(self.opengl, TextureTarget::Texture2D, id);
                self.texture_type = Some(TextureTarget::Texture2D);
                self.tex = id;
            },
        }
        Ok(self)
    }

    //pub fn unbind_all(opengl:&Gl) {
    //    intermediate_opengl::bind_vertex_array(opengl, ArrayObject::VertexArrayObject, 0);
    //    intermediate_opengl::bind_buffer(opengl, BufferObject::VertexBufferObject, 0);
    //    intermediate_opengl::bind_buffer(opengl, BufferObject::VertexBufferObject, 0);
    //    intermediate_opengl::bind_texture(opengl, TextureTarget::Texture2D, 0);
    //}

    pub fn new(opengl:&Gl, object:Object, format:DataFormat) -> WithObject<'_> {
        let object_id = intermediate_opengl::generate(opengl, object);
        WithObject::existing(opengl, object, object_id, format)
    }

    pub fn existing(opengl:&Gl, object:Object, id:u32, format:DataFormat) -> WithObject<'_> {
        match object {
            Object::VBO => {
                intermediate_opengl::bind_buffer(opengl, BufferObject::VertexBufferObject, id);
                WithObject { opengl, texture_type:None, data_format:format,
                             vao:0, vbo:id, ebo:0, tex:0 }
            },
            Object::VAO => {
                intermediate_opengl::bind_vertex_array(opengl, ArrayObject::VertexArrayObject, id);
                WithObject { opengl, texture_type:None, data_format:format,
                             vao:id, vbo:0, ebo:0, tex:0 }
            },
            Object::EBO => {
                intermediate_opengl::bind_buffer(opengl, BufferObject::ElementBufferObject, id);
                WithObject { opengl, texture_type:None, data_format:format,
                             vao:0, vbo:0, ebo:id, tex:0 }
            },
            Object::Texture2D => {
                intermediate_opengl::bind_texture(opengl, TextureTarget::Texture2D, id);
                WithObject { opengl, texture_type:None, data_format:format,
                             vao:0, vbo:0, ebo:0, tex:id }
            }
        }
    }

    pub fn buffer_data<T:Clone, const N:usize>(&self, data:&Matrix<T, N>, draw_type:DrawType, object:Object) -> Result<(), GlError> {
        let data_size = data.memory_size() as isize;
        let data_ptr = data.as_ptr() as *const c_void;
        match object {
            Object::VBO => {
                if self.vbo == 0 { Err(GlError::ObjectNotBound)? }
                intermediate_opengl::buffer_data(
                    self.opengl,
                    BufferObject::VertexBufferObject,
                    data_size, data_ptr, draw_type
                );
                Ok(())
                },
            Object::EBO => {
                if self.ebo == 0 { Err(GlError::ObjectNotBound)? }
                intermediate_opengl::buffer_data(
                    self.opengl,
                    BufferObject::ElementBufferObject,
                    data_size, data_ptr, draw_type
                );
                Ok(())
                },
            Object::VAO => Err(GlError::InvalidObjectType),
            Object::Texture2D => Err(GlError::InvalidObjectType),
        }
    }

    pub fn buffer_sub_data(&self, data:&Matrix<f32, 2>, object:Object) -> Result<(), GlError> {
        let data_size = data.memory_size() as isize;
        let data_ptr = data.as_ptr() as *const c_void;

        match object {
            Object::VBO => {
                if self.vbo == 0 { Err(GlError::ObjectNotBound)? }
                intermediate_opengl::buffer_sub_data(
                    self.opengl,
                    BufferObject::VertexBufferObject,
                    data_size, data_ptr
                );
                Ok(())
                },
            Object::EBO => {
                if self.ebo == 0 { Err(GlError::ObjectNotBound)? }
                intermediate_opengl::buffer_sub_data(
                    self.opengl,
                    BufferObject::ElementBufferObject,
                    data_size, data_ptr
                );
                Ok(())
                },
            Object::VAO => Err(GlError::InvalidObjectType),
            Object::Texture2D => Err(GlError::InvalidObjectType),
        }
    }

    pub fn set_vertex_attribs(&self, dtype_size:i32) -> Result<(), GlError> {
        if self.vao == 0 { Err(GlError::ObjectNotBound)? }
        match self.data_format {
            DataFormat::Position3Colour3Alpha1 => {
                intermediate_opengl::set_vertex_attrib_position_3(self.opengl, 0, 7, 0, dtype_size);
                intermediate_opengl::set_vertex_attrib_colour_3(  self.opengl, 1, 7, 3, dtype_size);
                intermediate_opengl::set_vertex_attrib_alpha_1(   self.opengl, 2, 7, 3+3, dtype_size);
            },
            DataFormat::Position3Colour3Alpha1Normal3 => {
                intermediate_opengl::set_vertex_attrib_position_3(self.opengl, 0, 10, 0, dtype_size);
                intermediate_opengl::set_vertex_attrib_colour_3(  self.opengl, 1, 10, 3, dtype_size);
                intermediate_opengl::set_vertex_attrib_alpha_1(   self.opengl, 2, 10, 3+3, dtype_size);
                intermediate_opengl::set_vertex_attrib_normal_3(  self.opengl, 3, 10, 3+3+1, dtype_size);
            },
            DataFormat::Position3Colour3Alpha1Normal3Texture2 => {
                intermediate_opengl::set_vertex_attrib_position_3(self.opengl, 0, 12, 0, dtype_size);
                intermediate_opengl::set_vertex_attrib_colour_3(  self.opengl, 1, 12, 3, dtype_size);
                intermediate_opengl::set_vertex_attrib_alpha_1(   self.opengl, 2, 12, 3+3, dtype_size);
                intermediate_opengl::set_vertex_attrib_normal_3(  self.opengl, 3, 12, 3+3+1, dtype_size);
                intermediate_opengl::set_vertex_attrib_texture_2( self.opengl, 4, 12, 3+3+1+3, dtype_size);
            },
            DataFormat::Position3Texture2 => {
                intermediate_opengl::set_vertex_attrib_position_3(self.opengl, 0, 5, 0, dtype_size);
                intermediate_opengl::set_vertex_attrib_texture_2( self.opengl, 1, 5, 3, dtype_size);
            },
        }
        Ok(())
    }

    pub fn get_location_after_vertex_attribs(&self) -> u32 {
        match self.data_format {
            DataFormat::Position3Colour3Alpha1 => { 3 },
            DataFormat::Position3Colour3Alpha1Normal3 => { 4 },
            DataFormat::Position3Colour3Alpha1Normal3Texture2 => { 5 },
            DataFormat::Position3Texture2 => { 2 },
        }
    }

    pub fn set_vertex_attrib_mat4_per_instance(&self, dtype_size:i32) -> Result<(), GlError> {
        if self.vao == 0 { Err(GlError::ObjectNotBound)? }

        let next_location = self.get_location_after_vertex_attribs();

        intermediate_opengl::set_vertex_attrib_mat4(self.opengl, next_location, dtype_size, UpdateVertexAttrib::PerInstance(1));
        
        Ok(())
    }

    pub fn draw<T:Clone, const N:usize>(&self, call:DrawCall, mode:DrawMode, data:&Matrix<T, N>) -> Result<(), GlError> {
        if data.ndims() != 2 { Err(GlError::InvalidDataDims(data.ndims()))? }

        match call {
            //DrawCall::Vertices => {
            //    if self.vbo != 0 && self.vao == 0 && self.ebo == 0 { Err(GlError::InvalidObjectType)? }
            //    //if self.object_type != Object::VBO { Err(GlError::InvalidObjectType)? }
            //    let is_ok_format = match self.data_format {
            //        DataFormat::Position3Colour3Alpha1 => true,
            //        DataFormat::Position3Colour3Alpha1Normal3 => true,
            //        DataFormat::Position3Texture2 => false,
            //        DataFormat::Position3Colour3Alpha1Normal3Texture2 => false, // untested, false to be safe
            //        //DataFormat::Position3Colour3Alpha1Normal3Texture2 => true, // untested, true to be safe
            //    };
            //    if !is_ok_format { Err(GlError::InvalidDataFormat)? }
            //
            //    let dtype_memsize = match data.dtype_memsize().try_into() {
            //        Ok(dtype_size) => Ok(dtype_size),
            //        Err(error) => Err(GlError::TryFromIntError(error)),
            //    }?;
            //
            //    self.set_vertex_attribs(dtype_memsize)
            //},
            DrawCall::Arrays => {
                if self.vao == 0 || self.ebo != 0 { Err(GlError::InvalidObjectType)? }
                //if self.object_type != Object::VAO { Err(GlError::InvalidObjectType)? }

                let count : i32 = match data.shape[1].try_into() {
                    Ok(i) => Ok(i),
                    Err(error) => Err(GlError::TryFromIntError(error)),
                }?;

                intermediate_opengl::draw_arrays(self.opengl, mode, count);
                Ok(())
            },
            DrawCall::Elements => {
                if self.vao == 0 || self.ebo == 0 { Err(GlError::InvalidObjectType)? }
                //if self.object_type != Object::VAO { Err(GlError::InvalidObjectType)? }

                let count = data.shape.iter().map(|s| *s as i32).product();

                intermediate_opengl::draw_elements(&self.opengl, mode, count);
                Ok(())
            },
        }
    }

    pub fn draw_instanced<T:Clone, const N:usize>(&self, call:DrawCall, mode:DrawMode, data:&Matrix<T, N>, instance_count:i32) -> Result<(), GlError> {
        if data.ndims() != 2 { Err(GlError::InvalidDataDims(data.ndims()))? }

        match call {
            DrawCall::Arrays => {
                if self.vao == 0 || self.ebo != 0 { Err(GlError::InvalidObjectType)? }
                //if self.object_type != Object::VAO { Err(GlError::InvalidObjectType)? }

                let count : i32 = match data.shape[1].try_into() {
                //let count : i32 = match data.shape[1].try_into() {
                    Ok(i) => Ok(i),
                    Err(error) => Err(GlError::TryFromIntError(error)),
                }?;

                intermediate_opengl::draw_arrays_instanced(self.opengl, mode, count, instance_count);
                Ok(())
            },
            DrawCall::Elements => {
                if self.vao == 0 || self.ebo == 0 { Err(GlError::InvalidObjectType)? }
                //if self.object_type != Object::VAO { Err(GlError::InvalidObjectType)? }

                let count = data.shape.iter().map(|s| *s as i32).product();

                intermediate_opengl::draw_elements_instanced(&self.opengl, mode, count, instance_count);
                Ok(())
            },
        }
    }
    
}
impl Drop for WithObject<'_> {
    fn drop(&mut self) {
        if self.get_vao() != 0 {
            intermediate_opengl::bind_vertex_array(self.opengl, ArrayObject::VertexArrayObject, 0);
        }
        if self.get_vbo() != 0 {
            intermediate_opengl::bind_buffer(self.opengl, BufferObject::VertexBufferObject, 0);
        }
        if self.get_ebo() != 0 {
            intermediate_opengl::bind_buffer(self.opengl, BufferObject::ElementBufferObject, 0);
        }
        if let Some(texture_type) = self.texture_type {
            intermediate_opengl::bind_texture(self.opengl, texture_type, 0);
        }
        //if self.texture_type.is_some() {
        //    intermediate_opengl::bind_texture(self.opengl, self.texture_type.unwrap(), 0);
        //}
    }
}

