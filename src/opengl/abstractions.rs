use crate::gl::Gl;
use crate::enums::{
    ArrayObject, BufferObject, DataFormat, DrawCall, DrawMode, DrawType, GlError, InternalFormat, Object, ProgramSelect, ShaderType, TextureFilter, TextureMagFilter, TextureMinFilter, TextureTarget, TextureWrap, TextureWrapping, UniformType, UnpreparedTexture
};
use crate::{gl, intermediate_opengl, raw_opengl};

use numeracy::matrices::Matrix;

use std::os::raw::c_void;

include!(concat!(env!("OUT_DIR"), "\\shaders_glsl.rs"));




#[derive(Debug)]
pub struct WithObject<'l> {
    opengl:&'l Gl,
    pub texture_type:Option<TextureTarget>,
    pub data_format:Option<DataFormat>,
    pub vao:u32,
    pub vbo:u32,
    pub ebo:u32,
    pub tex:u32,
}
impl WithObject<'_> {
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
                WithObject { opengl, texture_type:None, data_format:Some(format),
                             vao:0, vbo:id, ebo:0, tex:0 }
            },
            Object::VAO => {
                intermediate_opengl::bind_vertex_array(opengl, ArrayObject::VertexArrayObject, id);
                WithObject { opengl, texture_type:None, data_format:Some(format),
                             vao:id, vbo:0, ebo:0, tex:0 }
            },
            Object::EBO => {
                intermediate_opengl::bind_buffer(opengl, BufferObject::ElementBufferObject, id);
                WithObject { opengl, texture_type:None, data_format:Some(format),
                             vao:0, vbo:0, ebo:id, tex:0 }
            },
            Object::Texture2D => {
                intermediate_opengl::bind_texture(opengl, TextureTarget::Texture2D, id);
                WithObject { opengl, texture_type:None, data_format:Some(format),
                             vao:0, vbo:0, ebo:0, tex:id }
            }
        }
    }

    pub fn buffer_data<T:Clone>(&self, data:&Matrix<T>, draw_type:DrawType, object:Object) -> Result<(), GlError> {
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

    pub fn buffer_sub_data(&self, data:&Matrix<f32>, object:Object) -> Result<(), GlError> {
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
        match self.data_format.ok_or(GlError::InvalidDataFormat)? {
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
            DataFormat::Position3Texture2 => {
                intermediate_opengl::set_vertex_attrib_position_3(self.opengl, 0, 5, 0, dtype_size);
                intermediate_opengl::set_vertex_attrib_colour_3(  self.opengl, 1, 5, 3, dtype_size);
            },
        }
        Ok(())
    }

    pub fn draw<T:Clone>(&self, call:DrawCall, mode:DrawMode, data:&Matrix<T>) -> Result<(), GlError> {
        if data.ndims() != 2 { Err(GlError::InvalidDataDims(data.ndims()))? }

        let format = self.data_format.ok_or(GlError::InvalidDataFormat)?;

        match call {
            DrawCall::Vertices => {
                if self.vbo != 0 && self.vao == 0 && self.ebo == 0 { Err(GlError::InvalidObjectType)? }
                //if self.object_type != Object::VBO { Err(GlError::InvalidObjectType)? }
                let is_ok_format = match format {
                    DataFormat::Position3Colour3Alpha1 => true,
                    DataFormat::Position3Colour3Alpha1Normal3 => true,
                    DataFormat::Position3Texture2 => false,
                };
                if !is_ok_format { Err(GlError::InvalidDataFormat)? }

                let dtype_memsize = match data.dtype_memsize().try_into() {
                    Ok(dtype_size) => Ok(dtype_size),
                    Err(error) => Err(GlError::TryFromIntError(error)),
                }?;

                self.set_vertex_attribs(dtype_memsize)
            },
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
    
}
impl Drop for WithObject<'_> {
    fn drop(&mut self) {
        intermediate_opengl::bind_vertex_array(self.opengl, ArrayObject::VertexArrayObject, 0);
        intermediate_opengl::bind_buffer(self.opengl, BufferObject::VertexBufferObject, 0);
        intermediate_opengl::bind_buffer(self.opengl, BufferObject::VertexBufferObject, 0);
        if self.texture_type.is_some() {
            intermediate_opengl::bind_texture(self.opengl, self.texture_type.unwrap(), 0);
        }
    }
}



#[derive(Clone, Copy)]
pub struct Programs {
    pub simple_orthographic_shader:u32,
    pub blinn_phone_orthographic_shader:u32,
    pub simple_texture_shader:u32,
    pub current_program:Option<u32>,
    pub current_program_type:Option<ProgramSelect>,
}
impl Programs {

    pub fn compile_program_from_text(opengl:&Gl, vertex_text:&str, fragment_text:&str) -> Result<u32, GlError> {
        let vertex_id = intermediate_opengl::create_shader_variant(opengl, vertex_text, ShaderType::VertexShader)?;
        let fragment_id = intermediate_opengl::create_shader_variant(opengl, fragment_text, ShaderType::FragmentShader)?;

        let program_id = intermediate_opengl::create_shader_program(opengl, vertex_id, fragment_id)?;

        intermediate_opengl::remove_shader_variant(opengl, program_id, vertex_id);
        intermediate_opengl::remove_shader_variant(opengl, program_id, fragment_id);

        Ok(program_id)
    }

    pub fn compile_program_from_select(opengl:&Gl, program_type:ProgramSelect) -> Result<u32, GlError> {
        match program_type {
            ProgramSelect::SelectBlinnPhongOrthographic => {
                let vertex_text   = BLINN_PHONG_ORTHOGRAPHIC_VERTEX;
                let fragment_text = BLINN_PHONG_ORTHOGRAPHIC_FRAGMENT;
                let shader_id = Programs::compile_program_from_text(
                    opengl, vertex_text, fragment_text
                )?;
                Ok(shader_id)
            },
            ProgramSelect::SelectSimpleOrthographic => {
                let vertex_text   = SIMPLE_ORTHOGRAPHIC_VERTEX;
                let fragment_text = SIMPLE_ORTHOGRAPHIC_FRAGMENT;
                let shader_id = Programs::compile_program_from_text(
                    opengl, vertex_text, fragment_text
                )?;
                Ok(shader_id)
            },
            ProgramSelect::SelectSimpleTexture => {
                let vertex_text   = SIMPLE_TEXTURE_VERTEX;
                let fragment_text = SIMPLE_TEXTURE_FRAGMENT;
                let shader_id = Programs::compile_program_from_text(
                    opengl, vertex_text, fragment_text
                )?;
                Ok(shader_id)
            },
        }
    }

    pub fn compile(opengl:&Gl) -> Result<Programs, GlError> {
        let simple_orthographic_shader = Programs::compile_program_from_select(opengl, ProgramSelect::SelectSimpleOrthographic)?;
        let blinn_phone_orthographic_shader = Programs::compile_program_from_select(opengl, ProgramSelect::SelectBlinnPhongOrthographic)?;
        let simple_texture_shader = Programs::compile_program_from_select(opengl, ProgramSelect::SelectSimpleTexture)?;

        Ok(Programs { simple_orthographic_shader, blinn_phone_orthographic_shader,
                      simple_texture_shader,
                      current_program:None, current_program_type:None })
    }

    pub fn use_program(&mut self, opengl:&Gl, program:ProgramSelect) -> Result<(), GlError> {
        match program {
            ProgramSelect::SelectSimpleOrthographic => {
                intermediate_opengl::use_program(opengl, self.simple_orthographic_shader)?;
                self.current_program = Some(self.simple_orthographic_shader);
                self.current_program_type = Some(ProgramSelect::SelectSimpleOrthographic);
                Ok(())
            },
            ProgramSelect::SelectBlinnPhongOrthographic => {
                intermediate_opengl::use_program(opengl, self.blinn_phone_orthographic_shader)?;
                self.current_program = Some(self.blinn_phone_orthographic_shader);
                self.current_program_type = Some(ProgramSelect::SelectBlinnPhongOrthographic);
                Ok(())
            },
            ProgramSelect::SelectSimpleTexture => {
                intermediate_opengl::use_program(opengl, self.simple_texture_shader)?;
                self.current_program = Some(self.simple_texture_shader);
                self.current_program_type = Some(ProgramSelect::SelectSimpleTexture);
                Ok(())
            },
        }
    }

    pub fn disuse_program(&mut self, opengl:&Gl) {
        intermediate_opengl::disuse_program(opengl);
        self.current_program = None;
        self.current_program_type = None;
    }

    pub fn set_uniform(&self, opengl:&Gl, uniform_name:&str, uniform_type:UniformType, value:Matrix<f32>
    ) -> Result<(), GlError> {
        match self.current_program {
            Some(id) => intermediate_opengl::set_uniform(opengl, id, uniform_name, uniform_type, value.as_ptr()),
            None => Err(GlError::InvalidProgramID),
        }
    }

    pub fn draw<T:Clone>(
        &self, objects:WithObject, call:DrawCall,
        mode:DrawMode, data:&Matrix<T>,
    ) -> Result<(), GlError> {

        let format = objects.data_format.ok_or(GlError::InvalidDataFormat)?;

        match self.current_program_type {
            None => Err(GlError::InvalidProgramType),
            Some(program) => {
                match program {
                    ProgramSelect::SelectSimpleOrthographic => {
                        if format == DataFormat::Position3Colour3Alpha1 { Ok(()) } else { Err(GlError::InvalidDataFormat) }
                    },
                    ProgramSelect::SelectBlinnPhongOrthographic => {
                        if format == DataFormat::Position3Colour3Alpha1Normal3 { Ok(()) } else { Err(GlError::InvalidDataFormat) }
                    },
                    ProgramSelect::SelectSimpleTexture => {
                        if format == DataFormat::Position3Texture2 { Ok(()) } else { Err(GlError::InvalidDataFormat) }
                    },
                }
            },
        }?;

        //if format == DataFormat::Position3Texture2 {
        //    raw_opengl::bind_texture(opengl, gl::TEXTURE_2D, texture);
        //}

        Ok(objects.draw(call, mode, data)?)
        //let with_vao = WithObject::existing(opengl, Object::VAO, vao, format);
        //Ok(with_vao.draw(call, mode, data)?)

    }
}


pub struct Textures {
    texture_0  : Option<PreparedTexture>,
    texture_1  : Option<PreparedTexture>,
    texture_2  : Option<PreparedTexture>,
    texture_3  : Option<PreparedTexture>,
    texture_4  : Option<PreparedTexture>,
    texture_5  : Option<PreparedTexture>,
    texture_6  : Option<PreparedTexture>,
    texture_7  : Option<PreparedTexture>,
    texture_8  : Option<PreparedTexture>,
    texture_9  : Option<PreparedTexture>,
    texture_10 : Option<PreparedTexture>,
    texture_11 : Option<PreparedTexture>,
    texture_12 : Option<PreparedTexture>,
    texture_13 : Option<PreparedTexture>,
    texture_14 : Option<PreparedTexture>,
    texture_15 : Option<PreparedTexture>,
    texture_16 : Option<PreparedTexture>,
    texture_17 : Option<PreparedTexture>,
    texture_18 : Option<PreparedTexture>,
    texture_19 : Option<PreparedTexture>,
    texture_20 : Option<PreparedTexture>,
    texture_21 : Option<PreparedTexture>,
    texture_22 : Option<PreparedTexture>,
    texture_23 : Option<PreparedTexture>,
    texture_24 : Option<PreparedTexture>,
    texture_25 : Option<PreparedTexture>,
    texture_26 : Option<PreparedTexture>,
    texture_27 : Option<PreparedTexture>,
    texture_28 : Option<PreparedTexture>,
    texture_29 : Option<PreparedTexture>,
    texture_30 : Option<PreparedTexture>,
    texture_31 : Option<PreparedTexture>,
}

pub struct PreparedTexture {
    pub texture:u32,
    texture_type:TextureTarget,
    width:i32,
    height:i32,
    pixels:Vec<u8>,
}


#[derive(Clone, Debug)]
pub struct TextureSetup<'a> {
    opengl:&'a Gl,
    texture:u32,
    texture_type:TextureTarget,
    width:i32,
    height:i32,
    pixels:Vec<u8>,
    image_format:InternalFormat,
    wrapping_set:bool,
    filters_set:bool,
    texture_image_created:bool,
    mipmap_created:bool,
}
impl<'a> TextureSetup<'a> {
    pub fn get(opengl:&Gl, texture_type:TextureTarget, width:i32, height:i32, pixels:Vec<u8>, format:InternalFormat) -> TextureSetup {
        let texture_id = intermediate_opengl::generate(opengl, texture_type.into());


        TextureSetup { opengl:opengl, texture: texture_id, texture_type,
                        width, height, pixels, image_format:format,
                        wrapping_set:false, filters_set:false,
                        texture_image_created:false, mipmap_created:false }
    }

    pub fn set_st_wrapping(mut self, s_wrapping:TextureWrapping, t_wrapping:TextureWrapping) -> Self {
        intermediate_opengl::bind_texture(self.opengl, self.texture_type, self.texture);
        intermediate_opengl::texture_wrap(self.opengl, self.texture_type, TextureWrap::S, s_wrapping);
        intermediate_opengl::texture_wrap(self.opengl, self.texture_type, TextureWrap::T, t_wrapping);
        intermediate_opengl::bind_texture(self.opengl, self.texture_type, 0);
        self.wrapping_set = true;
        self
    }
    pub fn set_filters(mut self, min_filter:TextureMinFilter, mag_filter:TextureMagFilter) -> Self {
        intermediate_opengl::bind_texture(self.opengl, self.texture_type, self.texture);

        intermediate_opengl::texture_min_filter(self.opengl, self.texture_type, min_filter);
        intermediate_opengl::texture_mag_filter(self.opengl, self.texture_type, mag_filter);

        intermediate_opengl::bind_texture(self.opengl, self.texture_type, 0);
        self.filters_set = true;
        self
    }
    pub fn set_texture_image_and_mipmap(mut self, mipmap_level:i32) -> Self {
        intermediate_opengl::bind_texture(self.opengl, self.texture_type, self.texture);

        intermediate_opengl::texture_image(
            self.opengl, self.texture_type, mipmap_level,
            self.image_format, self.width, self.height, self.pixels.as_ptr() as *const u8
        );
        intermediate_opengl::generate_mipmap(self.opengl, self.texture_type);

        intermediate_opengl::bind_texture(self.opengl, self.texture_type, 0);
        self.texture_image_created = true;
        self.mipmap_created = true;
        self
    }
    //pub fn create_mipmap(mut self) -> Self {
    //    intermediate_opengl::bind_texture(self.opengl, self.texture_type, self.texture);
//
    //    intermediate_opengl::generate_mipmap(self.opengl, self.texture_type);
//
    //    intermediate_opengl::bind_texture(self.opengl, self.texture_type, 0);
    //    self.mipmap_created = true;
    //    self
    //}

    pub fn get_prepared_texture(self) -> Result<PreparedTexture, GlError> {
        if !self.wrapping_set {
            Err(GlError::TextureUnprepared(UnpreparedTexture::Wrapping))
        } else if !self.filters_set {
            Err(GlError::TextureUnprepared(UnpreparedTexture::Filters))
        } else if !self.texture_image_created {
            Err(GlError::TextureUnprepared(UnpreparedTexture::TextureImage))
        } else if !self.mipmap_created {
            Err(GlError::TextureUnprepared(UnpreparedTexture::Mipmap))
        } else {
            Ok(
                PreparedTexture {
                    texture: self.texture, texture_type: self.texture_type,
                    width: self.width, height: self.height, pixels: self.pixels
                }
            )
        }
    }
}