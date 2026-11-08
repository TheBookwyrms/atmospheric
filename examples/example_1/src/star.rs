use atmospheric::context::Context;
use atmospheric::objects::lighting::{LightingGenerator, PointLight};
use atmospheric::objects::vao_simple_object::SimpleColourObject;
use atmospheric::opengl::gl::Gl;
use atmospheric::enums::{ContextError, DrawMode, ObjectColour};
use numeracy::matrices::{ColumnMajor, GraphicsMatrices, Grapics, Matrix, S2};
use numeracy::vectors::Vector;

use crate::sphere::TripleRingSphere;

pub struct Star {
    pub drawing_data:SimpleColourObject<{3*10*60}>,
    pub mass:f32,
    pub velocity:[f32;3],
    pub acceleration:[f32;3],
    pub light:PointLight,
    pub centre:[f32;3],
    pub radius:f32,
}

impl Star {
    pub fn new(centre:[f32;3], radius:f32, xyz_rgb:Matrix<f32, 2, S2<3,3>>, mass:f32, lighting_generator:&mut LightingGenerator, render:&Context) -> Result<Self, ContextError> {

        let xc = xyz_rgb.get_col(0).unwrap().to_vector();
        let yc = xyz_rgb.get_col(1).unwrap().to_vector();
        let zc = xyz_rgb.get_col(2).unwrap().to_vector();
        let colour_avg = [
            (xc[0]+yc[0]+zc[0])/3., (xc[1]+yc[1]+zc[1])/3., (xc[2]+yc[2]+zc[2])/3.
        ];

        let sphere = TripleRingSphere::new(radius, xyz_rgb);

        let transformation = GraphicsMatrices::<ColumnMajor>::translate(Vector::from_1darray(centre));

        let object = SimpleColourObject::new(
            render.window.get_opengl_handle(),
            sphere.position_matrix, ObjectColour::PerVertex(sphere.colour_matrix), transformation,
            DrawMode::GlPoints,
        );
        let light = lighting_generator.generate_point_light(render, centre, colour_avg)?;

        Ok(Self { drawing_data: object, mass, velocity: [0.;3], acceleration: [0.;3], light, centre, radius })
    }
}