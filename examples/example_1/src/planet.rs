use atmospheric::objects;
use atmospheric::objects::vao_instancing_object::InstancingObject;
use atmospheric::objects::vao_simple_object::SimpleColourObject;
use numeracy::matrices::{Matrix, S2};
use numeracy::random::pseudo_random::{pseudo_randf64, pseudo_randf32_arr3};
use numeracy::vectors::Vector;
use atmospheric::opengl::gl::Gl;
use atmospheric::enums::{DrawMode, ObjectColour, ObjectMaterials, ObjectTexture};
use numeracy::matrices::{GraphicsMatrices,Grapics,ColumnMajor};
use rand::rngs::ThreadRng;

use crate::{sphere::TripleRingSphere, star::Star};

use rand::RngExt;


#[derive(Debug, Clone)]
pub struct Planets {
    pub drawing_data:InstancingObject<{Self::NUM_VERTICES}>,
    //pub drawing_data:SimpleColourObject<{3*10*60}>,
    pub masses:Vec<f32>,
    pub velocities:Vec<[f32;3]>,
    pub accelerations:Vec<[f32;3]>,
}

impl Planets {
    const NUM_VERTICES:usize = 3*10*60;
    pub fn union(opengl:&Gl, planets:Vec<Self>) -> Self {

        let transformations = planets.iter().map(|p| p.drawing_data.get_transformation_matrices().clone()).collect::<Vec<Vec<Matrix<f32, 2, S2<4, 4>>>>>().concat();
        let colours = planets.iter().map(|p| {
            let mut pi = vec![];
            for t in p.drawing_data.get_transformation_matrices() {
                pi.push(p.drawing_data.get_colour_matrix().clone());
            }
            pi
        }).collect::<Vec<Vec<ObjectColour<{Self::NUM_VERTICES}>>>>().concat();
        
        let mut a = vec![];
        for b in colours {
            if let ObjectColour::PerVertex(mat) = b {
                println!("{:?}", &mat.array[0..10]);
                a.push(mat);
            }
            else if let ObjectColour::PerVertexPerInstance(vec) = b {
                println!("{:?}", &vec.iter().map(|m| &m.array[0..10]));
                a.extend(vec);
            }
            else {
                panic!();
            }
        }
        let c = ObjectColour::PerVertexPerInstance(a.clone());
        let k = &a.iter().map(|m| &m.array[0..10]).collect::<Vec<&[f32]>>().concat();
        println!("c start {:?}", &k[0..10]);
        println!("c end {:?}", &k[k.len()-11..k.len()-1]);
        //panic!();


        let p0: InstancingObject<1800> = planets[0].drawing_data.clone();

        let masses = planets.iter().map(|p| p.masses.clone()).collect::<Vec<Vec<f32>>>().concat();
        let velocities = planets.iter().map(|p| p.velocities.clone()).collect::<Vec<Vec<[f32;3]>>>().concat();
        let accelerations = planets.iter().map(|p| p.accelerations.clone()).collect::<Vec<Vec<[f32;3]>>>().concat();

        let object = InstancingObject::new(
            opengl,
            //p0.get_position_matrix().clone(), p0.get_normal_matrix().clone(), p0.get_colour_matrix().clone(),
            p0.get_position_matrix().clone(), p0.get_normal_matrix().clone(), c,
            ObjectMaterials::None, ObjectTexture::None, transformations,
            DrawMode::GlPoints,
        );


        Self {
            drawing_data : object,
            masses,
            velocities,
            accelerations,
        }
    }
    pub fn new(opengl:&Gl,star:&Star, num_planets:usize, random:&mut ThreadRng) -> Self {
    //pub fn new(opengl:&Gl,star:&Star, xyz_rgb:Matrix<f32, 2, S2<3,3>>, num_planets:usize) -> Self {
        

        let xyz_rgb: [[f32;3];3] = core::array::from_fn(|_| random.random());
        let xyz_rgb = Matrix::from_2darray(xyz_rgb);
        let sphere = TripleRingSphere::new(xyz_rgb);


        let spawn_range = 10.*(star.radius*pseudo_randf64(0.2, 0.6, 100) as f32) as f64;
        let delay = 100;
        let mut transformations = Vec::with_capacity(num_planets);
        for _ in 0..num_planets {
            let tx = star.centre[0] + pseudo_randf64(-spawn_range, spawn_range, delay) as f32;
            let ty = star.centre[1] + pseudo_randf64(-spawn_range, spawn_range, delay) as f32;
            let tz = star.centre[2] + pseudo_randf64(-spawn_range, spawn_range, delay) as f32;
            
            let rx = pseudo_randf64(-45., 45., delay) as f32;
            let ry = pseudo_randf64(-45., 45., delay) as f32;
            let rz = pseudo_randf64(-45., 45., delay) as f32;

            let rotate    = GraphicsMatrices::<ColumnMajor>::rotate(Vector::from_1darray([rx, ry, rz]));
            let translate = GraphicsMatrices::<ColumnMajor>::translate(Vector::from_1darray([tx, ty, tz]));

            transformations.push(translate.matmul(&rotate));
        }

        let object = InstancingObject::new(
            opengl,
            sphere.position_matrix.clone(), sphere.normal_matrix, ObjectColour::PerVertex(sphere.colour_matrix.clone()),
            ObjectMaterials::None, ObjectTexture::None, transformations.clone(),
            DrawMode::GlPoints,
        );

        let mut masses = Vec::with_capacity(num_planets);
        let _ = (0..num_planets).map(|_| masses.push(pseudo_randf64(0.001*star.mass as f64, 0.1*star.mass as f64, 50) as f32));

        let mut velocities = Vec::with_capacity(num_planets);
        let _ = (0..num_planets).map(|_| velocities.push([0.,0.,0.]));
        //let _ = (0..num_planets).map(|_| velocities[i] = pseudo_randf32_arr3(0.001*star.mass, 0.1*star.mass, 50));

        let mut accelerations = Vec::with_capacity(num_planets);
        let _ = (0..num_planets).map(|_| accelerations.push([0.,0.,0.]));
        //let _ = (0..num_planets).map(|i| accelerations[i] = pseudo_randf32_arr3(0.001*star.mass, 0.1*star.mass, 50));


        Self { drawing_data: object, masses, velocities, accelerations }
    }
}