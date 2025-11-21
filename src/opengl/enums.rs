use std::iter;
use std::str::Utf8Error;
use std::num::TryFromIntError;
use std::ffi::NulError;

use numeracy::enums::MatrixError;
use crate::abstractions::TextureSetup;


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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ProgramSelect {
    SelectBlinnPhongOrthographic,
    SelectSimpleOrthographic,
    SelectSimpleTexture,
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
    Vertices,
    Arrays,
    Elements,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DataFormat {
    //Position3,
    //Position3Colour3,
    Position3Texture2,
    Position3Colour3Alpha1,
    Position3Colour3Alpha1Normal3,
    //Position3Colour3Texture2,
    //Position3Colour3Alpha1Texture2,
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
pub enum TextureFilter {
    MinFilter(TextureMinFilter),
    MagFilter(TextureMagFilter),
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

#[derive(Clone, PartialEq, Debug)]
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