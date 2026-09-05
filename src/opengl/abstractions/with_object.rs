use crate::opengl::gl::Gl;
use crate::opengl_helpers::enums::{
    ArrayObject, BufferObject, DataFormat,
    DrawCall, DrawMode, DrawType,
    GlError, Object, TextureTarget, UpdateVertexAttrib,
};
use crate::opengl::intermediate_opengl;

//use numeracy::matrices::Matrix;
use numeracy::matrices::{Matrix, S2, ShapeTrait};

use std::os::raw::c_void;

#[derive(Debug)]
pub struct WithVao<'l> {
    opengl:&'l Gl,
    //data_format:DataFormat,
    vao:u32,
}

impl WithVao<'_> {

    //pub fn get_data_format(&self) -> DataFormat { self.data_format }
    pub fn get_vao(&self) -> u32 { self.vao }

    //pub fn new(opengl:&Gl, format:DataFormat) -> WithVao<'_> {
    pub fn new(opengl:&Gl) -> WithVao<'_> {
        let object_id = intermediate_opengl::generate(opengl, Object::VAO);
        //WithVao::existing(opengl, object_id, format)
        WithVao::existing(opengl, object_id)
    }

    pub fn existing(opengl:&Gl, id:u32) -> WithVao<'_> {
    //pub fn existing(opengl:&Gl, id:u32, format:DataFormat) -> WithVao<'_> {
        intermediate_opengl::bind_vertex_array(opengl, ArrayObject::VertexArrayObject, id);
        //WithVao { opengl, data_format:format, vao:id }
        WithVao { opengl, vao:id }
    }

    //pub fn set_vertex_attribs_per_vertex(&self, dtype_size:i32) {
    pub fn set_vertex_attribs_per_vertex(&self, dtype_size:i32, data_format:DataFormat) {
        data_format.set_vertex_attribs(self.opengl, dtype_size);
        //self.data_format.set_vertex_attribs(self.opengl, dtype_size);
        //match self.data_format {
        //    DataFormat::Position3Colour3Alpha1 => {
        //        intermediate_opengl::set_vertex_attrib_position_3(self.opengl, 0, 7, 0, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_colour_3(  self.opengl, 1, 7, 3, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_alpha_1(   self.opengl, 2, 7, 3+3, dtype_size, UpdateVertexAttrib::PerVertex);
        //    },
        //    DataFormat::Position3Colour3Alpha1Normal3 => {
        //        intermediate_opengl::set_vertex_attrib_position_3(self.opengl, 0, 10, 0, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_colour_3(  self.opengl, 1, 10, 3, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_alpha_1(   self.opengl, 2, 10, 3+3, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_normal_3(  self.opengl, 3, 10, 3+3+1, dtype_size, UpdateVertexAttrib::PerVertex);
        //    },
        //    DataFormat::Position3Colour3Alpha1Normal3Texture2 => {
        //        intermediate_opengl::set_vertex_attrib_position_3(self.opengl, 0, 12, 0, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_colour_3(  self.opengl, 1, 12, 3, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_alpha_1(   self.opengl, 2, 12, 3+3, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_normal_3(  self.opengl, 3, 12, 3+3+1, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_texture_2( self.opengl, 4, 12, 3+3+1+3, dtype_size, UpdateVertexAttrib::PerVertex);
        //    },
        //    DataFormat::Position3Texture2 => {
        //        intermediate_opengl::set_vertex_attrib_position_3(self.opengl, 0, 5, 0, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_texture_2( self.opengl, 1, 5, 3, dtype_size, UpdateVertexAttrib::PerVertex);
        //    },
        //}
    }

    //pub fn get_location_after_vertex_attribs(&self) -> u32 {
    pub fn get_location_after_vertex_attribs(&self, data_format:DataFormat) -> u32 {

        let a = [UpdateVertexAttrib::PerVertex; 3];
        match data_format {
        //match self.data_format {
            DataFormat::Position3Colour4Normal3Texture2Material4TranformationMat4(a) => {panic!(); 16}            DataFormat::Position3Colour3Alpha1 => { 3 },
            DataFormat::Position3Colour3Alpha1Normal3 => { 4 },
            DataFormat::Position3Colour3Alpha1Normal3Texture2 => { 5 },
            DataFormat::Position3Texture2 => { 2 },
            _ => panic!(),
        }
    }

    //pub fn set_vertex_attrib_mat4_per_instance(&self, dtype_size:i32) -> Result<(), GlError> {
    pub fn set_vertex_attrib_mat4_per_instance(&self, dtype_size:i32, data_format:DataFormat) -> Result<(), GlError> {
        if self.vao == 0 { Err(GlError::ObjectNotBound)? }

        //let next_location = self.get_location_after_vertex_attribs();
        let next_location = self.get_location_after_vertex_attribs(data_format);

        intermediate_opengl::set_vertex_attrib_mat4(self.opengl, next_location, dtype_size, UpdateVertexAttrib::PerInstance(1));
        
        Ok(())
    }

    pub fn draw<T:Clone, const ITEMS_PER_VERTEX:usize, const NUM_VERTICES:usize>(&self, mode:DrawMode, _data:&Matrix<T, 2, S2<ITEMS_PER_VERTEX, NUM_VERTICES>>) {
        intermediate_opengl::draw_arrays(self.opengl, mode, NUM_VERTICES as i32);
    }

    pub fn draw_instanced<T:Clone, const ITEMS_PER_VERTEX:usize, const NUM_VERTICES:usize>(&self, mode:DrawMode, _data:&Matrix<T, 2, S2<ITEMS_PER_VERTEX, NUM_VERTICES>>, instance_count:i32) {
        intermediate_opengl::draw_arrays_instanced(self.opengl, mode, NUM_VERTICES as i32, instance_count);
    }
    
}
impl Drop for WithVao<'_> {
    fn drop(&mut self) {
        intermediate_opengl::bind_vertex_array(self.opengl, ArrayObject::VertexArrayObject, 0);
    }
}




















#[derive(Debug)]
pub struct WithVbo<'l> {
    opengl:&'l Gl,
    vbo:u32,
}
impl WithVbo<'_> {

    pub fn get_vbo(&self) -> u32 { self.vbo }

    pub fn new(opengl:&Gl) -> WithVbo<'_> {
        let object_id = intermediate_opengl::generate(opengl, Object::VBO);
        WithVbo::existing(opengl, object_id)
    }

    pub fn existing(opengl:&Gl, id:u32) -> WithVbo<'_> {
        intermediate_opengl::bind_buffer(opengl, BufferObject::VertexBufferObject, id);
        WithVbo { opengl, vbo:id }
    }

    pub fn buffer_data<T:Clone, const N:usize, U:ShapeTrait<N>>(&self, data:&Matrix<T, N, U>, draw_type:DrawType) {
        let data_size = data.memory_size() as isize;
        let data_ptr = data.as_ptr() as *const c_void;
        intermediate_opengl::buffer_data(
            self.opengl,
            BufferObject::VertexBufferObject,
            data_size, data_ptr, draw_type
        )
    }

    pub fn buffer_sub_data<U:ShapeTrait<2>>(&self, data:&Matrix<f32, 2, U>) {
        let data_size = data.memory_size() as isize;
        let data_ptr = data.as_ptr() as *const c_void;

        intermediate_opengl::buffer_sub_data(
            self.opengl,
            BufferObject::VertexBufferObject,
            data_size, data_ptr
        );
    }


    
}
impl Drop for WithVbo<'_> {
    fn drop(&mut self) {
        intermediate_opengl::bind_buffer(self.opengl, BufferObject::VertexBufferObject, 0);
    }
}
























#[derive(Debug)]
pub struct WithEbo<'l> {
    opengl:&'l Gl,
    data_format:DataFormat,
    ebo:u32,
}
impl WithEbo<'_> {
    
    pub fn get_data_format(&self) -> DataFormat { self.data_format }
    pub fn get_ebo(&self) -> u32 { self.ebo }



    pub fn new(opengl:&Gl, format:DataFormat) -> WithEbo<'_> {
        let object_id = intermediate_opengl::generate(opengl, Object::EBO);
        WithEbo::existing(opengl, object_id, format)
    }

    pub fn existing(opengl:&Gl, id:u32, format:DataFormat) -> WithEbo<'_> {
        intermediate_opengl::bind_buffer(opengl, BufferObject::ElementBufferObject, id);
        WithEbo { opengl, data_format:format, ebo:id }
    }

    pub fn buffer_data<T:Clone, const N:usize, U:ShapeTrait<N>>(&self, data:&Matrix<T, N, U>, draw_type:DrawType) {
        let data_size = data.memory_size() as isize;
        let data_ptr = data.as_ptr() as *const c_void;
        intermediate_opengl::buffer_data(
            self.opengl,
            BufferObject::ElementBufferObject,
            data_size, data_ptr, draw_type
        )
    }

    pub fn buffer_sub_data<U:ShapeTrait<2>>(&self, data:&Matrix<f32, 2, U>, object:Object) {
        let data_size = data.memory_size() as isize;
        let data_ptr = data.as_ptr() as *const c_void;
        intermediate_opengl::buffer_sub_data(
            self.opengl,
            BufferObject::ElementBufferObject,
            data_size, data_ptr
        )
    }    
}
impl Drop for WithEbo<'_> {
    fn drop(&mut self) {
        intermediate_opengl::bind_buffer(self.opengl, BufferObject::ElementBufferObject, 0);
    }
}
























#[derive(Debug)]
pub struct WithVaoEbo<'l> {
    opengl:&'l Gl,
    data_format:DataFormat,
    vao:u32,
    ebo:u32,
}
impl WithVaoEbo<'_> {
    
    pub fn get_data_format(&self) -> DataFormat { self.data_format }
    pub fn get_vao(&self) -> u32 { self.vao }
    pub fn get_ebo(&self) -> u32 { self.ebo }


    pub fn new(opengl:&Gl, format:DataFormat) -> WithVaoEbo<'_> {
        let vao_id = intermediate_opengl::generate(opengl, Object::VAO);
        let ebo_id = intermediate_opengl::generate(opengl, Object::EBO);
        WithVaoEbo::existing(opengl, vao_id, ebo_id, format)
    }

    pub fn existing(opengl:&Gl, vao:u32, ebo:u32, format:DataFormat) -> WithVaoEbo<'_> {
        intermediate_opengl::bind_vertex_array(opengl, ArrayObject::VertexArrayObject, vao);
        intermediate_opengl::bind_buffer(opengl, BufferObject::ElementBufferObject, ebo);
        WithVaoEbo { opengl, data_format:format, vao, ebo }
    }

    pub fn buffer_data<T:Clone, const N:usize, U:ShapeTrait<N>>(&self, data:&Matrix<T, N, U>, draw_type:DrawType, object:Object) {
        let data_size = data.memory_size() as isize;
        let data_ptr = data.as_ptr() as *const c_void;
        intermediate_opengl::buffer_data(
            self.opengl,
            BufferObject::ElementBufferObject,
            data_size, data_ptr, draw_type
        );
    }

    pub fn buffer_sub_data<U:ShapeTrait<2>>(&self, data:&Matrix<f32, 2, U>, object:Object) {
        let data_size = data.memory_size() as isize;
        let data_ptr = data.as_ptr() as *const c_void;

        intermediate_opengl::buffer_sub_data(
            self.opengl,
            BufferObject::ElementBufferObject,
            data_size, data_ptr
        )
    }

    pub fn set_vertex_attribs(&self, dtype_size:i32) -> Result<(), GlError> {
        self.data_format.set_vertex_attribs(self.opengl, dtype_size);
        //match self.data_format {
        //    DataFormat::Position3Colour3Alpha1 => {
        //        intermediate_opengl::set_vertex_attrib_position_3(self.opengl, 0, 7, 0, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_colour_3(  self.opengl, 1, 7, 3, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_alpha_1(   self.opengl, 2, 7, 3+3, dtype_size, UpdateVertexAttrib::PerVertex);
        //    },
        //    DataFormat::Position3Colour3Alpha1Normal3 => {
        //        intermediate_opengl::set_vertex_attrib_position_3(self.opengl, 0, 10, 0, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_colour_3(  self.opengl, 1, 10, 3, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_alpha_1(   self.opengl, 2, 10, 3+3, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_normal_3(  self.opengl, 3, 10, 3+3+1, dtype_size, UpdateVertexAttrib::PerVertex);
        //    },
        //    DataFormat::Position3Colour3Alpha1Normal3Texture2 => {
        //        intermediate_opengl::set_vertex_attrib_position_3(self.opengl, 0, 12, 0, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_colour_3(  self.opengl, 1, 12, 3, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_alpha_1(   self.opengl, 2, 12, 3+3, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_normal_3(  self.opengl, 3, 12, 3+3+1, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_texture_2( self.opengl, 4, 12, 3+3+1+3, dtype_size, UpdateVertexAttrib::PerVertex);
        //    },
        //    DataFormat::Position3Texture2 => {
        //        intermediate_opengl::set_vertex_attrib_position_3(self.opengl, 0, 5, 0, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_texture_2( self.opengl, 1, 5, 3, dtype_size, UpdateVertexAttrib::PerVertex);
        //    },
        //}
        Ok(())
    }

    pub fn get_location_after_vertex_attribs(&self) -> u32 {
        let a = [UpdateVertexAttrib::PerVertex; 3];
        match self.data_format {
            DataFormat::Position3Colour4Normal3Texture2Material4TranformationMat4(a) => {panic!(); 16}
            DataFormat::Position3Colour3Alpha1 => { 3 },
            DataFormat::Position3Colour3Alpha1Normal3 => { 4 },
            DataFormat::Position3Colour3Alpha1Normal3Texture2 => { 5 },
            DataFormat::Position3Texture2 => { 2 },
            _ => panic!()
        }
    }

    pub fn set_vertex_attrib_mat4_per_instance(&self, dtype_size:i32) {

        let next_location = self.get_location_after_vertex_attribs();

        intermediate_opengl::set_vertex_attrib_mat4(self.opengl, next_location, dtype_size, UpdateVertexAttrib::PerInstance(1));
        
    }

    pub fn draw<T:Clone, U:ShapeTrait<2>>(&self, mode:DrawMode, data:&Matrix<T, 2, U>) {

        let count = data.shape.as_array().iter().map(|s| *s as i32).product();

        intermediate_opengl::draw_elements(&self.opengl, mode, count);
    }

    pub fn draw_instanced<T:Clone, U:ShapeTrait<2>>(&self, mode:DrawMode, data:&Matrix<T, 2, U>, instance_count:i32) {

        let count = data.shape.as_array().iter().map(|s| *s as i32).product();

        intermediate_opengl::draw_elements_instanced(&self.opengl, mode, count, instance_count);
    }
    
}
impl Drop for WithVaoEbo<'_> {
    fn drop(&mut self) {
        intermediate_opengl::bind_vertex_array(self.opengl, ArrayObject::VertexArrayObject, 0);
        intermediate_opengl::bind_buffer(self.opengl, BufferObject::ElementBufferObject, 0);
    }
}
























#[derive(Debug)]
pub struct WithVaoVbo<'l> {
    opengl:&'l Gl,
    data_format:DataFormat,
    vao:u32,
    vbo:u32,
}
impl WithVaoVbo<'_> {

    pub fn get_data_format(&self) -> DataFormat { self.data_format }
    pub fn get_vao(&self) -> u32 { self.vao }
    pub fn get_vbo(&self) -> u32 { self.vbo }

    pub fn new(opengl:&Gl, format:DataFormat) -> WithVaoVbo<'_> {
        let vao_id = intermediate_opengl::generate(opengl, Object::VAO);
        let vbo_id = intermediate_opengl::generate(opengl, Object::VBO);
        WithVaoVbo::existing(opengl, vao_id, vbo_id, format)
    }

    pub fn existing(opengl:&Gl, vao:u32, vbo:u32, format:DataFormat) -> WithVaoVbo<'_> {
        intermediate_opengl::bind_buffer(opengl, BufferObject::VertexBufferObject, vao);
        intermediate_opengl::bind_vertex_array(opengl, ArrayObject::VertexArrayObject, vbo);
        WithVaoVbo { opengl, data_format:format, vao, vbo }
    }

    pub fn buffer_data<T:Clone, const N:usize, U:ShapeTrait<N>>(&self, data:&Matrix<T, N, U>, draw_type:DrawType, object:Object) {
        let data_size = data.memory_size() as isize;
        let data_ptr = data.as_ptr() as *const c_void;
        intermediate_opengl::buffer_data(
            self.opengl,
            BufferObject::VertexBufferObject,
            data_size, data_ptr, draw_type
        );
    }

    pub fn buffer_sub_data<U:ShapeTrait<2>>(&self, data:&Matrix<f32, 2, U>, object:Object) {
        let data_size = data.memory_size() as isize;
        let data_ptr = data.as_ptr() as *const c_void;

        intermediate_opengl::buffer_sub_data(
            self.opengl,
            BufferObject::VertexBufferObject,
            data_size, data_ptr
        );
    }

    pub fn set_vertex_attribs(&self, dtype_size:i32) {
        self.data_format.set_vertex_attribs(self.opengl, dtype_size);
        //match self.data_format {
        //    DataFormat::Position3Colour3Alpha1 => {
        //        intermediate_opengl::set_vertex_attrib_position_3(self.opengl, 0, 7, 0, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_colour_3(  self.opengl, 1, 7, 3, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_alpha_1(   self.opengl, 2, 7, 3+3, dtype_size, UpdateVertexAttrib::PerVertex);
        //    },
        //    DataFormat::Position3Colour3Alpha1Normal3 => {
        //        intermediate_opengl::set_vertex_attrib_position_3(self.opengl, 0, 10, 0, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_colour_3(  self.opengl, 1, 10, 3, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_alpha_1(   self.opengl, 2, 10, 3+3, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_normal_3(  self.opengl, 3, 10, 3+3+1, dtype_size, UpdateVertexAttrib::PerVertex);
        //    },
        //    DataFormat::Position3Colour3Alpha1Normal3Texture2 => {
        //        intermediate_opengl::set_vertex_attrib_position_3(self.opengl, 0, 12, 0, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_colour_3(  self.opengl, 1, 12, 3, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_alpha_1(   self.opengl, 2, 12, 3+3, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_normal_3(  self.opengl, 3, 12, 3+3+1, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_texture_2( self.opengl, 4, 12, 3+3+1+3, dtype_size, UpdateVertexAttrib::PerVertex);
        //    },
        //    DataFormat::Position3Texture2 => {
        //        intermediate_opengl::set_vertex_attrib_position_3(self.opengl, 0, 5, 0, dtype_size, UpdateVertexAttrib::PerVertex);
        //        intermediate_opengl::set_vertex_attrib_texture_2( self.opengl, 1, 5, 3, dtype_size, UpdateVertexAttrib::PerVertex);
        //    },
        //}
    }

    pub fn get_location_after_vertex_attribs(&self) -> u32 {

        let a = [UpdateVertexAttrib::PerVertex; 3];
        match self.data_format {
            DataFormat::Position3Colour4Normal3Texture2Material4TranformationMat4(a) => {panic!(); 16}            DataFormat::Position3Colour3Alpha1 => { 3 },
            DataFormat::Position3Colour3Alpha1Normal3 => { 4 },
            DataFormat::Position3Colour3Alpha1Normal3Texture2 => { 5 },
            DataFormat::Position3Texture2 => { 2 },
            _ => panic!()
        }
    }

    pub fn set_vertex_attrib_mat4_per_instance(&self, dtype_size:i32) {

        let next_location = self.get_location_after_vertex_attribs();

        intermediate_opengl::set_vertex_attrib_mat4(self.opengl, next_location, dtype_size, UpdateVertexAttrib::PerInstance(1));
        
    }

    pub fn draw<T:Clone, const ITEMS_PER_VERTEX:usize, const NUM_VERTICES:usize>(&self, mode:DrawMode, data:&Matrix<T, 2, S2<ITEMS_PER_VERTEX, NUM_VERTICES>>) {
        intermediate_opengl::draw_arrays(self.opengl, mode, NUM_VERTICES as i32);
    }

    pub fn draw_instanced<T:Clone, const ITEMS_PER_VERTEX:usize, const NUM_VERTICES:usize>(&self, mode:DrawMode, data:&Matrix<T, 2, S2<ITEMS_PER_VERTEX, NUM_VERTICES>>, instance_count:i32) {
        intermediate_opengl::draw_arrays_instanced(self.opengl, mode, NUM_VERTICES as i32, instance_count);
    }
    
}
impl Drop for WithVaoVbo<'_> {
    fn drop(&mut self) {
        intermediate_opengl::bind_vertex_array(self.opengl, ArrayObject::VertexArrayObject, 0);
        intermediate_opengl::bind_buffer(self.opengl, BufferObject::VertexBufferObject, 0);
    }
}
























//#[derive(Debug)]
//pub struct WithTexture<'l> {
//    opengl:&'l Gl,
//    data_format:DataFormat,
//    vao:u32,
//}
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//#[derive(Debug)]
//pub struct WithObject<'l> {
//    opengl:&'l Gl,
//    texture_type:Option<TextureTarget>,
//    data_format:DataFormat,
//    vao:u32,
//    vbo:u32,
//    ebo:u32,
//    tex:u32,
//}
//impl WithObject<'_> {
//    pub fn get_data_format(&self) -> DataFormat { self.data_format }
//    pub fn get_vao(&self) -> u32 { self.vao }
//    pub fn get_vbo(&self) -> u32 { self.vbo }
//    pub fn get_ebo(&self) -> u32 { self.ebo }
//    pub fn get_tex(&self) -> u32 { self.tex }
//
//    pub fn add(mut self, object:Object, id:u32) -> Result<Self, GlError> {
//        match object {
//            Object::VBO => {
//                if self.vbo != 0 { Err(GlError::ObjectAlreadyBound)? }
//                intermediate_opengl::bind_buffer(self.opengl, BufferObject::VertexBufferObject, id);
//                self.vbo = id;
//            },
//            Object::VAO => {
//                if self.vao != 0 { Err(GlError::ObjectAlreadyBound)? }
//                intermediate_opengl::bind_vertex_array(self.opengl, ArrayObject::VertexArrayObject, id);
//                self.vao = id;
//            },
//            Object::EBO => {
//                if self.ebo != 0 { Err(GlError::ObjectAlreadyBound)? }
//                intermediate_opengl::bind_buffer(self.opengl, BufferObject::ElementBufferObject, id);
//                self.ebo = id;
//            },
//            Object::Texture2D => {
//                if self.tex != 0 { Err(GlError::ObjectAlreadyBound)? }
//                intermediate_opengl::bind_texture(self.opengl, TextureTarget::Texture2D, id);
//                self.texture_type = Some(TextureTarget::Texture2D);
//                self.tex = id;
//            },
//        }
//        Ok(self)
//    }
//
//    //pub fn unbind_all(opengl:&Gl) {
//    //    intermediate_opengl::bind_vertex_array(opengl, ArrayObject::VertexArrayObject, 0);
//    //    intermediate_opengl::bind_buffer(opengl, BufferObject::VertexBufferObject, 0);
//    //    intermediate_opengl::bind_buffer(opengl, BufferObject::VertexBufferObject, 0);
//    //    intermediate_opengl::bind_texture(opengl, TextureTarget::Texture2D, 0);
//    //}
//
//    pub fn new(opengl:&Gl, object:Object, format:DataFormat) -> WithObject<'_> {
//        let object_id = intermediate_opengl::generate(opengl, object);
//        WithObject::existing(opengl, object, object_id, format)
//    }
//
//    pub fn existing(opengl:&Gl, object:Object, id:u32, format:DataFormat) -> WithObject<'_> {
//        match object {
//            Object::VBO => {
//                intermediate_opengl::bind_buffer(opengl, BufferObject::VertexBufferObject, id);
//                WithObject { opengl, texture_type:None, data_format:format,
//                             vao:0, vbo:id, ebo:0, tex:0 }
//            },
//            Object::VAO => {
//                intermediate_opengl::bind_vertex_array(opengl, ArrayObject::VertexArrayObject, id);
//                WithObject { opengl, texture_type:None, data_format:format,
//                             vao:id, vbo:0, ebo:0, tex:0 }
//            },
//            Object::EBO => {
//                intermediate_opengl::bind_buffer(opengl, BufferObject::ElementBufferObject, id);
//                WithObject { opengl, texture_type:None, data_format:format,
//                             vao:0, vbo:0, ebo:id, tex:0 }
//            },
//            Object::Texture2D => {
//                intermediate_opengl::bind_texture(opengl, TextureTarget::Texture2D, id);
//                WithObject { opengl, texture_type:None, data_format:format,
//                             vao:0, vbo:0, ebo:0, tex:id }
//            }
//        }
//    }
//
//    pub fn buffer_data<T:Clone, const N:usize, U:ShapeTrait<N>>(&self, data:&Matrix<T, N, U>, draw_type:DrawType, object:Object) -> Result<(), GlError> {
//        let data_size = data.memory_size() as isize;
//        let data_ptr = data.as_ptr() as *const c_void;
//        match object {
//            Object::VBO => {
//                if self.vbo == 0 { Err(GlError::ObjectNotBound)? }
//                intermediate_opengl::buffer_data(
//                    self.opengl,
//                    BufferObject::VertexBufferObject,
//                    data_size, data_ptr, draw_type
//                );
//                Ok(())
//                },
//            Object::EBO => {
//                if self.ebo == 0 { Err(GlError::ObjectNotBound)? }
//                intermediate_opengl::buffer_data(
//                    self.opengl,
//                    BufferObject::ElementBufferObject,
//                    data_size, data_ptr, draw_type
//                );
//                Ok(())
//                },
//            Object::VAO => Err(GlError::InvalidObjectType),
//            Object::Texture2D => Err(GlError::InvalidObjectType),
//        }
//    }
//
//    pub fn buffer_sub_data<U:ShapeTrait<2>>(&self, data:&Matrix<f32, 2, U>, object:Object) -> Result<(), GlError> {
//        let data_size = data.memory_size() as isize;
//        let data_ptr = data.as_ptr() as *const c_void;
//
//        match object {
//            Object::VBO => {
//                if self.vbo == 0 { Err(GlError::ObjectNotBound)? }
//                intermediate_opengl::buffer_sub_data(
//                    self.opengl,
//                    BufferObject::VertexBufferObject,
//                    data_size, data_ptr
//                );
//                Ok(())
//                },
//            Object::EBO => {
//                if self.ebo == 0 { Err(GlError::ObjectNotBound)? }
//                intermediate_opengl::buffer_sub_data(
//                    self.opengl,
//                    BufferObject::ElementBufferObject,
//                    data_size, data_ptr
//                );
//                Ok(())
//                },
//            Object::VAO => Err(GlError::InvalidObjectType),
//            Object::Texture2D => Err(GlError::InvalidObjectType),
//        }
//    }
//
//    pub fn set_vertex_attribs(&self, dtype_size:i32) -> Result<(), GlError> {
//        if self.vao == 0 { Err(GlError::ObjectNotBound)? }
//        match self.data_format {
//            DataFormat::Position3Colour3Alpha1 => {
//                intermediate_opengl::set_vertex_attrib_position_3(self.opengl, 0, 7, 0, dtype_size, UpdateVertexAttrib::PerVertex);
//                intermediate_opengl::set_vertex_attrib_colour_3(  self.opengl, 1, 7, 3, dtype_size, UpdateVertexAttrib::PerVertex);
//                intermediate_opengl::set_vertex_attrib_alpha_1(   self.opengl, 2, 7, 3+3, dtype_size, UpdateVertexAttrib::PerVertex);
//            },
//            DataFormat::Position3Colour3Alpha1Normal3 => {
//                intermediate_opengl::set_vertex_attrib_position_3(self.opengl, 0, 10, 0, dtype_size, UpdateVertexAttrib::PerVertex);
//                intermediate_opengl::set_vertex_attrib_colour_3(  self.opengl, 1, 10, 3, dtype_size, UpdateVertexAttrib::PerVertex);
//                intermediate_opengl::set_vertex_attrib_alpha_1(   self.opengl, 2, 10, 3+3, dtype_size, UpdateVertexAttrib::PerVertex);
//                intermediate_opengl::set_vertex_attrib_normal_3(  self.opengl, 3, 10, 3+3+1, dtype_size, UpdateVertexAttrib::PerVertex);
//            },
//            DataFormat::Position3Colour3Alpha1Normal3Texture2 => {
//                intermediate_opengl::set_vertex_attrib_position_3(self.opengl, 0, 12, 0, dtype_size, UpdateVertexAttrib::PerVertex);
//                intermediate_opengl::set_vertex_attrib_colour_3(  self.opengl, 1, 12, 3, dtype_size, UpdateVertexAttrib::PerVertex);
//                intermediate_opengl::set_vertex_attrib_alpha_1(   self.opengl, 2, 12, 3+3, dtype_size, UpdateVertexAttrib::PerVertex);
//                intermediate_opengl::set_vertex_attrib_normal_3(  self.opengl, 3, 12, 3+3+1, dtype_size, UpdateVertexAttrib::PerVertex);
//                intermediate_opengl::set_vertex_attrib_texture_2( self.opengl, 4, 12, 3+3+1+3, dtype_size, UpdateVertexAttrib::PerVertex);
//            },
//            DataFormat::Position3Texture2 => {
//                intermediate_opengl::set_vertex_attrib_position_3(self.opengl, 0, 5, 0, dtype_size, UpdateVertexAttrib::PerVertex);
//                intermediate_opengl::set_vertex_attrib_texture_2( self.opengl, 1, 5, 3, dtype_size, UpdateVertexAttrib::PerVertex);
//            },
//        }
//        Ok(())
//    }
//
//    pub fn get_location_after_vertex_attribs(&self) -> u32 {
//        match self.data_format {
//            DataFormat::Position3Colour3Alpha1 => { 3 },
//            DataFormat::Position3Colour3Alpha1Normal3 => { 4 },
//            DataFormat::Position3Colour3Alpha1Normal3Texture2 => { 5 },
//            DataFormat::Position3Texture2 => { 2 },
//        }
//    }
//
//    pub fn set_vertex_attrib_mat4_per_instance(&self, dtype_size:i32) -> Result<(), GlError> {
//        if self.vao == 0 { Err(GlError::ObjectNotBound)? }
//
//        let next_location = self.get_location_after_vertex_attribs();
//
//        intermediate_opengl::set_vertex_attrib_mat4(self.opengl, next_location, dtype_size, UpdateVertexAttrib::PerInstance(1));
//        
//        Ok(())
//    }
//
//    pub fn draw<T:Clone, U:ShapeTrait<2>>(&self, call:DrawCall, mode:DrawMode, data:&Matrix<T, 2, U>) -> Result<(), GlError> {
//        match call {
//            //DrawCall::Vertices => {
//            //    if self.vbo != 0 && self.vao == 0 && self.ebo == 0 { Err(GlError::InvalidObjectType)? }
//            //    //if self.object_type != Object::VBO { Err(GlError::InvalidObjectType)? }
//            //    let is_ok_format = match self.data_format {
//            //        DataFormat::Position3Colour3Alpha1 => true,
//            //        DataFormat::Position3Colour3Alpha1Normal3 => true,
//            //        DataFormat::Position3Texture2 => false,
//            //        DataFormat::Position3Colour3Alpha1Normal3Texture2 => false, // untested, false to be safe
//            //        //DataFormat::Position3Colour3Alpha1Normal3Texture2 => true, // untested, true to be safe
//            //    };
//            //    if !is_ok_format { Err(GlError::InvalidDataFormat)? }
//            //
//            //    let dtype_memsize = match data.dtype_memsize().try_into() {
//            //        Ok(dtype_size) => Ok(dtype_size),
//            //        Err(error) => Err(GlError::TryFromIntError(error)),
//            //    }?;
//            //
//            //    self.set_vertex_attribs(dtype_memsize)
//            //},
//            DrawCall::Arrays => {
//                if self.vao == 0 || self.ebo != 0 { Err(GlError::InvalidObjectType)? }
//                //if self.object_type != Object::VAO { Err(GlError::InvalidObjectType)? }
//
//                let count : i32 = match data.shape.as_array()[1].try_into() {
//                    Ok(i) => Ok(i),
//                    Err(error) => Err(GlError::TryFromIntError(error)),
//                }?;
//
//                intermediate_opengl::draw_arrays(self.opengl, mode, count);
//                Ok(())
//            },
//            DrawCall::Elements => {
//                if self.vao == 0 || self.ebo == 0 { Err(GlError::InvalidObjectType)? }
//                //if self.object_type != Object::VAO { Err(GlError::InvalidObjectType)? }
//
//                let count = data.shape.as_array().iter().map(|s| *s as i32).product();
//
//                intermediate_opengl::draw_elements(&self.opengl, mode, count);
//                Ok(())
//            },
//        }
//    }
//
//    pub fn draw_instanced<T:Clone, U:ShapeTrait<2>>(&self, call:DrawCall, mode:DrawMode, data:&Matrix<T, 2, U>, instance_count:i32) -> Result<(), GlError> {
//        match call {
//            DrawCall::Arrays => {
//                if self.vao == 0 || self.ebo != 0 { Err(GlError::InvalidObjectType)? }
//                //if self.object_type != Object::VAO { Err(GlError::InvalidObjectType)? }
//
//                let count : i32 = match data.shape.as_array()[1].try_into() {
//                //let count : i32 = match data.shape[1].try_into() {
//                    Ok(i) => Ok(i),
//                    Err(error) => Err(GlError::TryFromIntError(error)),
//                }?;
//
//                intermediate_opengl::draw_arrays_instanced(self.opengl, mode, count, instance_count);
//                Ok(())
//            },
//            DrawCall::Elements => {
//                if self.vao == 0 || self.ebo == 0 { Err(GlError::InvalidObjectType)? }
//                //if self.object_type != Object::VAO { Err(GlError::InvalidObjectType)? }
//
//                let count = data.shape.as_array().iter().map(|s| *s as i32).product();
//
//                intermediate_opengl::draw_elements_instanced(&self.opengl, mode, count, instance_count);
//                Ok(())
//            },
//        }
//    }
//    
//}
//impl Drop for WithObject<'_> {
//    fn drop(&mut self) {
//        if self.get_vao() != 0 {
//            intermediate_opengl::bind_vertex_array(self.opengl, ArrayObject::VertexArrayObject, 0);
//        }
//        if self.get_vbo() != 0 {
//            intermediate_opengl::bind_buffer(self.opengl, BufferObject::VertexBufferObject, 0);
//        }
//        if self.get_ebo() != 0 {
//            intermediate_opengl::bind_buffer(self.opengl, BufferObject::ElementBufferObject, 0);
//        }
//        if let Some(texture_type) = self.texture_type {
//            intermediate_opengl::bind_texture(self.opengl, texture_type, 0);
//        }
//        //if self.texture_type.is_some() {
//        //    intermediate_opengl::bind_texture(self.opengl, self.texture_type.unwrap(), 0);
//        //}
//    }
//}
//
//