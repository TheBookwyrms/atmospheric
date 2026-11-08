use atmospheric::objects::vao_instancing_object::InstancingObject;
use numeracy::matrices::{Matrix, S2};
use numeracy::random::pseudo_random::pseudo_randf64;
use numeracy::vectors::Vector;
use atmospheric::opengl::gl::Gl;
use atmospheric::enums::{DrawMode, ObjectColour, ObjectMaterials, ObjectTexture};
use numeracy::matrices::{GraphicsMatrices,Grapics,ColumnMajor};

use crate::{sphere::TripleRingSphere, star::Star};

pub struct Planets<const NUM_PLANETS:usize> {
    pub drawing_data:InstancingObject<NUM_PLANETS, {3*10*60}>,
    pub masses:[f32;NUM_PLANETS],
    pub velocities:[[f32;3];NUM_PLANETS],
    pub accelerations:[[f32;3];NUM_PLANETS],
}

impl<const NUM_PLANETS:usize> Planets<NUM_PLANETS> {
    pub fn new(opengl:&Gl,star:&Star, xyz_rgb:Matrix<f32, 2, S2<3,3>>) -> Self {
        

        let sphere = TripleRingSphere::new(star.radius*pseudo_randf64(0.2, 0.6, 100) as f32, xyz_rgb);



        let spawn_range = 10.*sphere.radius as f64;
        let delay = 100;
        let mut transformations = Matrix::array_of::<NUM_PLANETS>(Matrix::identity());
        for i in 0..NUM_PLANETS {
            let tx = star.centre[0] + pseudo_randf64(-spawn_range, spawn_range, delay) as f32;
            let ty = star.centre[1] + pseudo_randf64(-spawn_range, spawn_range, delay) as f32;
            let tz = star.centre[2] + pseudo_randf64(-spawn_range, spawn_range, delay) as f32;
            
            let rx = pseudo_randf64(-45., 45., delay) as f32;
            let ry = pseudo_randf64(-45., 45., delay) as f32;
            let rz = pseudo_randf64(-45., 45., delay) as f32;

            let rotate    = GraphicsMatrices::<ColumnMajor>::rotate(Vector::from_1darray([rx, ry, rz]));
            let translate = GraphicsMatrices::<ColumnMajor>::translate(Vector::from_1darray([tx, ty, tz]));

            transformations[i] = translate.matmul(&rotate);
        }

        let object = InstancingObject::new(
            opengl,
            sphere.position_matrix, sphere.normal_matrix, ObjectColour::PerVertex(sphere.colour_matrix),
            ObjectMaterials::None, ObjectTexture::None, transformations,
            DrawMode::GlPoints,
        );

        let mut masses = [0.;NUM_PLANETS];
        let _ = (0..NUM_PLANETS).map(|i| masses[i] = pseudo_randf64(0.001*star.mass as f64, 0.1*star.mass as f64, 50) as f32);

        let mut velocities = [[0.;3];NUM_PLANETS];
        //(0..NUM_PLANETS).map(|i| masses[i] = pseudo_randf64(0.001*star.mass as f64, 0.1*star.mass as f64, 50) as f32);

        let mut accelerations = [[0.;3];NUM_PLANETS];
        //(0..NUM_PLANETS).map(|i| masses[i] = pseudo_randf64(0.001*star.mass as f64, 0.1*star.mass as f64, 50) as f32);

        Self { drawing_data: object, masses, velocities, accelerations }
    }
}