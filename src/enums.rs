use glfw::{InitError, WindowEvent};
use numeracy::enums::MatrixError;
use numeracy::matrices::MatrixError as MatrixErrorNew;
use std::str::Utf8Error;
use std::num::TryFromIntError;
use std::ffi::NulError;

use crate::opengl::gl::Gl;
use crate::opengl::intermediate_opengl;



#[derive(Debug)] // Copy
pub enum GlError {
    CStringError(NulError),
    InvalidShaderType(ShaderType),
    InvalidBufferType(BufferObject),
    InvalidDrawType(DrawType),
    InvalidLayoutLocation(u32),
    InvalidDrawMode(DrawMode),
    CompilationSuccessFailed(String),
    InvalidDataDims(usize),
    InvalidColour(f32, f32, f32, f32),
    FileError(std::io::Error),
    TextError(Utf8Error),
    MatrixError(MatrixError),
    TryFromIntError(TryFromIntError),
    DataLengthError(usize),
    InvalidObjectType,
    NotImplementedYet,
    InvalidProgramID,
    InvalidProgramType,
    InvalidDataFormat,
    ObjectNotBound,
    ObjectAlreadyBound,
    TextureUnprepared(UnpreparedTexture),
    AlreadyActivated(OpenglTexture),
    AlreadyDeactivated(OpenglTexture),
    NoProgramBound,
    InvalidIndex(usize),
    InvalidCustomProgramSelect,
    InvalidDrawInstancing,
}

#[derive(Clone, Copy, Debug)]
pub enum UnpreparedTexture {
    Wrapping,
    Filters,
    TextureImage,
    Mipmap,
}

#[derive(Clone, Copy, Debug)]
pub enum UniformType {
    Int,
    Float,
    Vec3,
    Mat4,
}

#[derive(Clone, Copy, Debug)]
pub enum ShaderType {
    VertexShader,
    FragmentShader,
    ShaderProgram,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BufferObject {
    VertexBufferObject,
    ElementBufferObject,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ArrayObject {
    VertexArrayObject,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DrawType {
    StaticDraw,
    StreamDraw,
    DynamicDraw,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DrawMode {
    GlTriangles,
    GlPoints,
    GlLines,
    GlTriangleStrip,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BlendFunc {
    SRCAlphaOneMinusSRCAlpha,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BufferBit {
    ColourBufferBit,
    DepthBufferBit,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GlEnable {
    DepthTest,
    Multisample,
    Blend,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Object {
    VAO,
    VBO,
    EBO,
    Texture2D,
}

impl From<TextureTarget> for Object {
    fn from(value: TextureTarget) -> Self {
        match value {
            TextureTarget::Texture2D => Object::Texture2D
        }
    }
}


#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DrawCall {
    //Vertices, // deprecated because why would you ever use it
    Arrays,
    Elements,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AttributeLoc {
    At(u32)
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DataFormat {
    Position3(AttributeLoc, UpdateVertexAttrib),
    Colour4(AttributeLoc, UpdateVertexAttrib),
    Normal3(AttributeLoc, UpdateVertexAttrib),
    Texture2(AttributeLoc, UpdateVertexAttrib),
    Material3331(AttributeLoc, UpdateVertexAttrib),
    TranformationMatrix4x4(AttributeLoc, UpdateVertexAttrib),
    //Position3Colour3,
    Position3Texture2,
    Position3Colour3Alpha1,
    Position3Colour3Alpha1Normal3,
    Position3Colour3Alpha1Normal3Texture2,
    Position3Colour4Normal3Texture2Material4TranformationMat4([UpdateVertexAttrib; 3]),
    //Position3Colour3Texture2,
    //Position3Colour3Alpha1Texture2,
}
impl DataFormat {
    pub fn set_vertex_attribs(self, opengl:&Gl, dtype_size:i32) {
        match self {
            Self::Position3(loc, update) => {
                let AttributeLoc::At(location) = loc;
                intermediate_opengl::set_vertex_attrib_vec3(opengl, location, 3, 0, dtype_size, update);
            },
            Self::Colour4(loc, update) => {
                let AttributeLoc::At(location) = loc;
                intermediate_opengl::set_vertex_attrib_vec4(opengl, location, 4, 0, dtype_size, update);
            },
            Self::Normal3(loc, update) => {
                let AttributeLoc::At(location) = loc;
                intermediate_opengl::set_vertex_attrib_vec3(opengl, location, 3, 0, dtype_size, update);
            },
            Self::Texture2(loc, update) => {
                let AttributeLoc::At(location) = loc;
                intermediate_opengl::set_vertex_attrib_vec2(opengl, location, 2, 0, dtype_size, update);
            },
            Self::Material3331(loc, update) => {
                let AttributeLoc::At(location) = loc;
                intermediate_opengl::set_vertex_attrib_vec3(opengl, location,       10, 0, dtype_size, update);
                intermediate_opengl::set_vertex_attrib_vec3(opengl, location+1,     10, 0+3, dtype_size, update);
                intermediate_opengl::set_vertex_attrib_vec3(opengl, location+1+1,   10, 0+3+3, dtype_size, update);
                intermediate_opengl::set_vertex_attrib_vec1(opengl, location+1+1+1, 10, 0+3+3+3, dtype_size, update);
            },
            Self::TranformationMatrix4x4(loc, update) => {
                let AttributeLoc::At(location) = loc;
                intermediate_opengl::set_vertex_attrib_mat4(opengl, location, dtype_size, update);
            },
            Self::Position3Colour3Alpha1 => {
                intermediate_opengl::set_vertex_attrib_vec3(opengl, 0, 7, 0, dtype_size, UpdateVertexAttrib::PerVertex);
                intermediate_opengl::set_vertex_attrib_vec3(opengl, 1, 7, 3, dtype_size, UpdateVertexAttrib::PerVertex);
                intermediate_opengl::set_vertex_attrib_vec1(opengl, 2, 7, 3+3, dtype_size, UpdateVertexAttrib::PerVertex);
            },
            Self::Position3Colour3Alpha1Normal3 => {
                intermediate_opengl::set_vertex_attrib_vec3(opengl, 0, 10, 0, dtype_size, UpdateVertexAttrib::PerVertex);
                intermediate_opengl::set_vertex_attrib_vec3(opengl, 1, 10, 3, dtype_size, UpdateVertexAttrib::PerVertex);
                intermediate_opengl::set_vertex_attrib_vec1(opengl, 2, 10, 3+3, dtype_size, UpdateVertexAttrib::PerVertex);
                intermediate_opengl::set_vertex_attrib_vec3(opengl, 3, 10, 3+3+1, dtype_size, UpdateVertexAttrib::PerVertex);
            },
            Self::Position3Colour3Alpha1Normal3Texture2 => {
                intermediate_opengl::set_vertex_attrib_vec3(opengl, 0, 12, 0, dtype_size, UpdateVertexAttrib::PerVertex);
                intermediate_opengl::set_vertex_attrib_vec3(opengl, 1, 12, 3, dtype_size, UpdateVertexAttrib::PerVertex);
                intermediate_opengl::set_vertex_attrib_vec1(opengl, 2, 12, 3+3, dtype_size, UpdateVertexAttrib::PerVertex);
                intermediate_opengl::set_vertex_attrib_vec3(opengl, 3, 12, 3+3+1, dtype_size, UpdateVertexAttrib::PerVertex);
                intermediate_opengl::set_vertex_attrib_vec2(opengl, 4, 12, 3+3+1+3, dtype_size, UpdateVertexAttrib::PerVertex);
            },
            Self::Position3Texture2 => {
                intermediate_opengl::set_vertex_attrib_vec3(opengl, 0, 5, 0, dtype_size, UpdateVertexAttrib::PerVertex);
                intermediate_opengl::set_vertex_attrib_vec2(opengl, 1, 5, 3, dtype_size, UpdateVertexAttrib::PerVertex);
            },
            Self::Position3Colour4Normal3Texture2Material4TranformationMat4(
                [colour_update, texture_update, material_update]
            ) => {
                intermediate_opengl::set_vertex_attrib_vec3(opengl, 0, 16, 0, dtype_size, UpdateVertexAttrib::PerVertex);
                intermediate_opengl::set_vertex_attrib_vec4(opengl, 1, 16, 0+3, dtype_size, colour_update);
                intermediate_opengl::set_vertex_attrib_vec3(opengl, 2, 16, 0+3+4, dtype_size, UpdateVertexAttrib::PerVertex);
                intermediate_opengl::set_vertex_attrib_vec2(opengl, 3, 16, 0+3+4+3, dtype_size, texture_update);
                intermediate_opengl::set_vertex_attrib_vec4(opengl, 4, 16, 0+3+4+3+2, dtype_size, material_update);
                intermediate_opengl::set_vertex_attrib_mat4(opengl, 5, dtype_size, UpdateVertexAttrib::PerInstance(1));
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TextureTarget {
    Texture2D,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TextureWrap {
    /// x
    S,

    /// y
    T,
    
    /// z
    R,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TextureWrapping {
    Repeat,
    MirroredRepeat,
    ClampToEdge,
    ClampToBorder(f32, f32, f32, f32),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TextureMinFilter {
    NearestMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapNearest,
    LinearMipmapLinear,
    Linear,
    Nearest,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TextureMagFilter {
    Linear,
    Nearest,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum InternalFormat {
    RGB,
    RGBA,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum OpenglTexture {
    Texture0,
    Texture1,
    Texture2,
    Texture3,
    Texture4,
    Texture5,
    Texture6,
    Texture7,
    Texture8,
    Texture9,
    Texture10,
    Texture11,
    Texture12,
    Texture13,
    Texture14,
    Texture15,
    Texture16,
    Texture17,
    Texture18,
    Texture19,
    Texture20,
    Texture21,
    Texture22,
    Texture23,
    Texture24,
    Texture25,
    Texture26,
    Texture27,
    Texture28,
    Texture29,
    Texture30,
    Texture31,
}
#[derive(Debug)]
pub enum ContextError {
    NewGLFWEventDetected(WindowEvent),
    GLFWinitError(InitError),
    GLFWNoWindowCreated,
    GLFWResizeBoundsError((i32, i32)),
    GLError(GlError),
    TryFromIntError(TryFromIntError),
    DataLengthError(usize),
    MatrixError(MatrixError),
    MatrixErrorDim1(MatrixErrorNew<1>),
    MatrixErrorDim2(MatrixErrorNew<2>),
    MaxDirectionalLightsGenerated,
    MaxPointLightsGenerated,
    MaxSpotLightsGenerated,
}

impl From<GlError> for ContextError {
    fn from(value: GlError) -> Self {
        Self::GLError(value)
    }
}

impl From<MatrixError> for ContextError {
    fn from(value: MatrixError) -> Self {
        Self::MatrixError(value)
    }
}

impl From<MatrixErrorNew<1>> for ContextError {
    fn from(value: MatrixErrorNew<1>) -> Self {
        Self::MatrixErrorDim1(value)
    }
}

impl From<MatrixErrorNew<2>> for ContextError {
    fn from(value: MatrixErrorNew<2>) -> Self {
        Self::MatrixErrorDim2(value)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ImageFormat {
    JPEG,
    PNG,
    PPMP3
}

impl Into<InternalFormat> for ImageFormat {
    fn into(self) -> InternalFormat {
        match self {
            ImageFormat::JPEG => InternalFormat::RGB,
            ImageFormat::PNG => InternalFormat::RGBA,
            ImageFormat::PPMP3 => InternalFormat::RGB,
        }
    }
}



pub enum PPMType {
    P3,
}

impl Into<ImageFormat> for PPMType {
    fn into(self) -> ImageFormat {
        match self {
            PPMType::P3 => ImageFormat::PPMP3,
        }
    }
}

#[derive(Clone, Copy)]
pub enum CameraMode {
    Encompassing,
    PointOfView,
}

pub enum CameraAxis {
    Forward,
    Right,
    Up,
}

pub enum CameraVector {
    Position,
    Target,
    Right,
    Up
}

#[derive(Clone, Copy)]
pub enum LightForm {
    Ambient,
    Diffuse,
    Specular,
}

#[derive(Clone, Copy)]
pub enum LightSourceForm {
    Directional,
    Point,
    Spot,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum UpdateVertexAttrib {
    PerVertex,
    PerInstance(u32)
}