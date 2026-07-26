use crate::enums::UniformType;

pub struct Uniform<'a> {
    pub name:&'a str,
    pub uniform_type:UniformType,
}