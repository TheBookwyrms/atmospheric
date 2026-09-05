use crate::opengl_helpers::image_processing::Image;
use crate::opengl::gl::Gl;
use crate::opengl_helpers::enums::{
    GlError, InternalFormat, OpenglTexture,
    TextureMagFilter, TextureMinFilter, TextureTarget,
    TextureWrap, TextureWrapping, UniformType,
    UnpreparedTexture,
};
use crate::opengl::intermediate_opengl;

use crate::opengl::abstractions::Programs;

//use numeracy::matrices::Matrix;
use numeracy::matrices::Matrix;


fn opengl_texture_to_index(tex:OpenglTexture) -> usize {
    match tex {
        OpenglTexture::Texture0  => 0,
        OpenglTexture::Texture1  => 1,
        OpenglTexture::Texture2  => 2,
        OpenglTexture::Texture3  => 3,
        OpenglTexture::Texture4  => 4,
        OpenglTexture::Texture5  => 5,
        OpenglTexture::Texture6  => 6,
        OpenglTexture::Texture7  => 7,
        OpenglTexture::Texture8  => 8,
        OpenglTexture::Texture9  => 9,
        OpenglTexture::Texture10 => 10,
        OpenglTexture::Texture11 => 11,
        OpenglTexture::Texture12 => 12,
        OpenglTexture::Texture13 => 13,
        OpenglTexture::Texture14 => 14,
        OpenglTexture::Texture15 => 15,
        OpenglTexture::Texture16 => 16,
        OpenglTexture::Texture17 => 17,
        OpenglTexture::Texture18 => 18,
        OpenglTexture::Texture19 => 19,
        OpenglTexture::Texture20 => 20,
        OpenglTexture::Texture21 => 21,
        OpenglTexture::Texture22 => 22,
        OpenglTexture::Texture23 => 23,
        OpenglTexture::Texture24 => 24,
        OpenglTexture::Texture25 => 25,
        OpenglTexture::Texture26 => 26,
        OpenglTexture::Texture27 => 27,
        OpenglTexture::Texture28 => 28,
        OpenglTexture::Texture29 => 29,
        OpenglTexture::Texture30 => 30,
        OpenglTexture::Texture31 => 31,
    }
}

fn index_to_opengl_texture(index:usize) -> Result<OpenglTexture, GlError> {
    match index {
        0  => Ok(OpenglTexture::Texture0),
        1  => Ok(OpenglTexture::Texture1),
        2  => Ok(OpenglTexture::Texture2),
        3  => Ok(OpenglTexture::Texture3),
        4  => Ok(OpenglTexture::Texture4),
        5  => Ok(OpenglTexture::Texture5),
        6  => Ok(OpenglTexture::Texture6),
        7  => Ok(OpenglTexture::Texture7),
        8  => Ok(OpenglTexture::Texture8),
        9  => Ok(OpenglTexture::Texture9),
        10 => Ok(OpenglTexture::Texture10),
        11 => Ok(OpenglTexture::Texture11),
        12 => Ok(OpenglTexture::Texture12),
        13 => Ok(OpenglTexture::Texture13),
        14 => Ok(OpenglTexture::Texture14),
        15 => Ok(OpenglTexture::Texture15),
        16 => Ok(OpenglTexture::Texture16),
        17 => Ok(OpenglTexture::Texture17),
        18 => Ok(OpenglTexture::Texture18),
        19 => Ok(OpenglTexture::Texture19),
        20 => Ok(OpenglTexture::Texture20),
        21 => Ok(OpenglTexture::Texture21),
        22 => Ok(OpenglTexture::Texture22),
        23 => Ok(OpenglTexture::Texture23),
        24 => Ok(OpenglTexture::Texture24),
        25 => Ok(OpenglTexture::Texture25),
        26 => Ok(OpenglTexture::Texture26),
        27 => Ok(OpenglTexture::Texture27),
        28 => Ok(OpenglTexture::Texture28),
        29 => Ok(OpenglTexture::Texture29),
        30 => Ok(OpenglTexture::Texture30),
        31 => Ok(OpenglTexture::Texture31),
        n => Err(GlError::InvalidIndex(n))
    }
}





pub struct Textures<'a> {
    textures  : [Option<&'a PreparedTexture>; 32]
}
impl<'a> Textures<'a> {
    pub fn new_empty() -> Self {
        Textures {
            textures:[
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
            ]
        }
    }

    pub fn activate(&mut self, opengl:&Gl, tex:OpenglTexture, prepared:&'a PreparedTexture, programs:&Programs) -> Result<(), GlError> {
        
        let tex_index = opengl_texture_to_index(tex);
        let current_program = match programs.current_program {
            None => Err(GlError::NoProgramBound),
            Some(n) => Ok(n),
        }?;
        let uniform = format!("texture{}", tex_index);

        match &self.textures[tex_index] {
            None => {
                intermediate_opengl::active_texture(opengl, tex);
                intermediate_opengl::bind_texture(opengl, prepared.texture_type, prepared.texture);
                intermediate_opengl::set_uniform(
                    opengl, current_program, &uniform,
                    UniformType::Int, Matrix::from_scalar(tex_index).as_ptr()
                )?;
                self.textures[tex_index] = Some(prepared);
                Ok(())
            },
            Some(_) => Err(GlError::AlreadyActivated(tex)),
        }
    }

    pub fn deactivate(&mut self, opengl:&Gl, tex:OpenglTexture) -> Result<(), GlError> {
        
        let tex_index = opengl_texture_to_index(tex);

        match &self.textures[tex_index] {
            Some(texture) => {
                intermediate_opengl::active_texture(opengl, tex);
                intermediate_opengl::bind_texture(opengl, texture.texture_type, 0);
                self.textures[tex_index] = None;
                Ok(())
            },
            None => Err(GlError::AlreadyDeactivated(tex)),
        }
    }

    pub fn deactivate_all(&mut self, opengl:&Gl) {
        for tex_index in 0..32 {
            if let Some(texture) = &self.textures[tex_index] {
                intermediate_opengl::active_texture(opengl, index_to_opengl_texture(tex_index).unwrap());
                intermediate_opengl::bind_texture(opengl, texture.texture_type, 0);
                self.textures[tex_index] = None;
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct PreparedTexture {
    texture:u32,
    texture_type:TextureTarget,
    _width:i32,
    _height:i32,
    _pixels:Vec<u8>,
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
    pub fn get_prepared(
        opengl:&'a Gl, texture_type:TextureTarget, image:Image,
        s_wrapping:TextureWrapping, t_wrapping:TextureWrapping,
        min_filter:TextureMinFilter, mag_filter:TextureMagFilter,
        mipmap_level:i32
    ) -> PreparedTexture {
        let texture_id = intermediate_opengl::generate(opengl, texture_type.into());

        let prepared_texture = TextureSetup {
            opengl:opengl, texture: texture_id, texture_type,
            width:image.width, height:image.height, pixels:image.data, image_format:image.format.into(),
            wrapping_set:false, filters_set:false,
            texture_image_created:false, mipmap_created:false
        }.set_st_wrapping(s_wrapping, t_wrapping)
         .set_filters(min_filter, mag_filter)
         .set_texture_image_and_mipmap(mipmap_level)
         .get_if_prepared().unwrap();

        prepared_texture
    }
    pub fn get_prepared_default(opengl:&'a Gl, image:Image) -> PreparedTexture {
        let texture_id = intermediate_opengl::generate(opengl, TextureTarget::Texture2D.into());

        let prepared_texture = TextureSetup {
            opengl:opengl, texture: texture_id, texture_type:TextureTarget::Texture2D,
            width:image.width, height:image.height, pixels:image.data, image_format:image.format.into(),
            wrapping_set:false, filters_set:false,
            texture_image_created:false, mipmap_created:false
        }.set_st_wrapping(TextureWrapping::Repeat, TextureWrapping::Repeat)
         .set_filters(TextureMinFilter::LinearMipmapNearest, TextureMagFilter::Linear)
         .set_texture_image_and_mipmap(0)
         .get_if_prepared().unwrap();

        prepared_texture
    }

    pub fn get_unprepared(opengl:&'a Gl, texture_type:TextureTarget, image:Image) -> TextureSetup<'a> {
        let texture_id = intermediate_opengl::generate(opengl, texture_type.into());


        TextureSetup {
            opengl:opengl, texture: texture_id, texture_type,
            width:image.width, height:image.height, pixels:image.data, image_format:image.format.into(),
            wrapping_set:false, filters_set:false,
            texture_image_created:false, mipmap_created:false
        }
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

    pub fn get_if_prepared(self) -> Result<PreparedTexture, GlError> {
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
                    _width: self.width, _height: self.height, _pixels: self.pixels
                }
            )
        }
    }
}