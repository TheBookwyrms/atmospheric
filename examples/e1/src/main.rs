//#![allow(warnings)]

// uncomment for release
// #![windows_subsystem = "windows"]


use atmospheric::image_processing::Image;
use atmospheric::enums::{
    CameraMode, ContextError, DataFormat, DrawMode, ImageFormat, LightForm, ObjectColour, ObjectMaterials, ObjectTexture
};
use atmospheric::modules::shader_modules::shader_module_enum::ShaderModule;
use atmospheric::objects::ntree::PointThing;
use atmospheric::objects::octree::Octree;
use atmospheric::objects::vao_simple_object::SimpleColourObject;
use atmospheric::objects::{
    lighting::{LightCounter, LightingGenerator},
    vao_instancing_object::InstancingObject,
    cube::Cube,
};
use atmospheric::context::Context;

use atmospheric::opengl::abstractions::WithVao;
use numeracy::matrices::{ColumnMajor, GraphicsMatrices, Matrix, S2, Grapics};
use numeracy::vectors::Vector;
use numeracy::random::pseudo_random;

use std::f32::consts::PI;


use atmospheric::modules::keybinding_modules::keybindings::KeybindingModule;


fn main() -> Result<(), ContextError> {

    let keybinding_modules = vec![
        KeybindingModule::CameraZoomScroll, KeybindingModule::CameraPanLeftClick, KeybindingModule::CameraRotateRightClick,
        KeybindingModule::PauseSpace, KeybindingModule::ScreenshotCtrlK,
        KeybindingModule::CloseWindowEscape, KeybindingModule::NecessaryWindowStuff,
        KeybindingModule::ChangeCameraModeE,
        KeybindingModule::CameraPanWASD, KeybindingModule::CameraRotateUpDownLeftRight,
    ];

    let shader_modules = vec![
        ShaderModule::PhongOrthographic,
        ShaderModule::SimpleOrthographic,
        ShaderModule::InstancingBlinnPhong,
        ShaderModule::SimpleTransform,
    ];

    const NUM_SUNS:usize = 2;

    let max_lights = LightCounter::max_values(1, 10, 10);
    let mut lighting_generator = LightingGenerator::init(&max_lights);

    //let mut render = Context::new_default(max_lights)?;
    let mut render = Context::new("window name", 900, 900, CameraMode::Encompassing, max_lights, keybinding_modules, shader_modules)?;
    render.setup_render();


    
    let mut point_light2 = lighting_generator.generate_point_light(&render, [0., 0., 0.], [0.75, 0.95, 0.65])?;
    let mut point_light3 = lighting_generator.generate_point_light(&render, [0., 0., 0.], [0.75, 0.95, 0.65])?;
    let mut dir_light = lighting_generator.generate_directional_light([0., 0., 1.], [1.;3])?;
    let mut spot_light = lighting_generator.generate_spot_light([0., 0., 10.], [0., 0., -1.], [0.75, 0.95, 0.65], PI/6., PI/3.)?;

    


    let cube = Cube::new((0., 0., 0.), 6., 1.);



    const NUM_INSTANCES:usize = 1000;
    let spawn_range = 50.;

    let mut colour_matrices = Matrix::array_of::<NUM_INSTANCES>(Matrix::null(S2::<4, 1>));
    let mut transformation_matrices = Matrix::array_of::<NUM_INSTANCES>(Matrix::null(S2::<4, 4>));
    let delay = 100;
    for i in 0..NUM_INSTANCES {
        let tx = pseudo_random::pseudo_randf64(-spawn_range, spawn_range, delay) as f32;
        let ty = pseudo_random::pseudo_randf64(-spawn_range, spawn_range, delay) as f32;
        let tz = pseudo_random::pseudo_randf64(-spawn_range, spawn_range, delay) as f32;
        
        let rx = pseudo_random::pseudo_randf64(-45., 45., delay) as f32;
        let ry = pseudo_random::pseudo_randf64(-45., 45., delay) as f32;
        let rz = pseudo_random::pseudo_randf64(-45., 45., delay) as f32;
        
        let cx = pseudo_random::pseudo_randf64(0., 1., delay) as f32;
        let cy = pseudo_random::pseudo_randf64(0., 1., delay) as f32;
        let cz = pseudo_random::pseudo_randf64(0., 1., delay) as f32;
        let ca = 1.;



        let rotate    = GraphicsMatrices::<ColumnMajor>::rotate(Vector::from_1darray([rx, ry, rz]));
        let translate = GraphicsMatrices::<ColumnMajor>::translate(Vector::from_1darray([tx, ty, tz]));

        // transpose this as opengl uses column-major format
        // while i use row-major format for these matrices
        transformation_matrices[i] = translate.matmul(&rotate);
        colour_matrices[i] = Matrix::from_2darray([[cx, cy, cz, ca]]);


    }



    let container_diffuse_map    = Image::decode_from_path("assets/container_diffuse_map.png",  ImageFormat::PNG,   true);
    let container_specular_map   = Image::decode_from_path("assets/container_specular_map.png", ImageFormat::PNG,   true);


    let (cube_pos, cube_col, cube_norm, cube_tex) = cube.get_pcnt_matrices();

    let mut real_instancing_object = InstancingObject::new(
        &render.window.get_opengl_handle(),
        cube_pos.clone(),
        cube_norm,
        //ObjectColour::None,
        //ObjectColour::Constant(Matrix::from_2darray([[1., 1., 1., 1.]])),
        ObjectColour::ConstantPerInstance(colour_matrices),
        //ObjectColour::PerVertex(ccnt_col),
        //ObjectTextureCoords::PerVertex(ccnt_tex),
        ObjectMaterials::None,
        //ObjectMaterials::Constant(Material::Emerald),
        //ObjectMaterials::Constant(Material::Silver),
        //ObjectTexture::None,
        ObjectTexture::PerVertex(container_diffuse_map, container_specular_map, cube_tex),
        transformation_matrices
    );


    let simple_cube = SimpleColourObject::new(
        &render.window.get_opengl_handle(),
        cube_pos,
        ObjectColour::PerVertex(cube_col),
        Matrix::identity()
    );

    
    
    const NUM_POINTS : usize = 299;
    const SIDE_LEN : f64 = 4.;

    let mut points = vec![];
    for _ in 0..NUM_POINTS {
        let x = pseudo_random::pseudo_randf64(-SIDE_LEN, SIDE_LEN, 100) as f32;
        let y = pseudo_random::pseudo_randf64(-SIDE_LEN, SIDE_LEN, 100) as f32;
        let z = pseudo_random::pseudo_randf64(-SIDE_LEN, SIDE_LEN, 100) as f32;
        //let z = 0.0;
        points.push(PointThing::new(x, y, z));
    }

    let q1 = Octree::new_on_origin(2.*SIDE_LEN as f32);
    //let q1 = QuadTree::new_on_origin(2.*SIDE_LEN as f32);
    let q2 = q1.insert(points, 5).unwrap();
    

    let quadtree_final = q2;    
    let quadtree_lines = quadtree_final.get_all_lines::<{NUM_POINTS*2*12}>();
    let quadtree_points =  quadtree_final.get_all_points::<NUM_POINTS>();
    
    let (qtl_vao, qtl_vbo) = render.create_vao_vbo(&quadtree_lines, DataFormat::Position3Colour3Alpha1);//?;
    let (qtp_vao, qtp_vbo) = render.create_vao_vbo(&quadtree_points, DataFormat::Position3Colour3Alpha1);//?;


    while !render.render_over() {

        render.begin_render_actions()?;




        // quadtree and octree testing
        render.use_program(ShaderModule::SimpleOrthographic, vec![])?;
        let with_qtl = WithVao::existing(&render.window.get_opengl_handle(), qtl_vao);
        render.shaders.draw(with_qtl, DrawMode::GlLines, &quadtree_lines, DataFormat::Position3Colour3Alpha1)?;
        let with_qtp = WithVao::existing(&render.window.get_opengl_handle(), qtp_vao);
        render.shaders.draw(with_qtp, DrawMode::GlPoints, &quadtree_points, DataFormat::Position3Colour3Alpha1)?;




        let time = render.window.get_time_since_glfw_init() as f32;
        

        let tr = 0.5;
        // simple shader for light source
        render.use_program(ShaderModule::SimpleOrthographic, vec![])?;

        // // light source's diffuse colour changes over time
        point_light2.set_light(LightForm::Diffuse, [
            f32::sin(2.*0.75*time), f32::sin(2.*0.25*time), f32::sin(2.*0.65*time),
        ]);
        //point_light2.set_position([10.*f32::sin(time), 10.*f32::cos(time), 10.]);
        point_light2.translate([tr*f32::sin(time), tr*f32::cos(time), 0.]);

        point_light3.set_light(LightForm::Diffuse, [
            f32::sin(2.*0.65*time), f32::sin(2.*0.75*time), f32::sin(2.*0.25*time),
        ]);
        //point_light3.set_position([-10.*f32::cos(time), -10.*f32::sin(time), 10.]);
        point_light3.translate([-tr*f32::cos(time), -tr*f32::sin(time), 0.]);

        point_light2.draw(&render)?;
        point_light3.draw(&render)?;

        spot_light.rotate([0., -0.75, 0.])?;

        
        let a = vec![point_light2.into(), point_light3.into()];
        render.use_program(ShaderModule::InstancingBlinnPhong, a.clone())?;

        render.shaders.set_orthographic_camera_uniforms(&render.window, &render.camera)?;
        render.shaders.set_blinn_phong_uniforms(&render.window, &render.camera)?;
        for light in a {
            light.set_lighting_uniforms(&render.window, &render.shaders)?
        }
        //render.use_program(ShaderModule::InstancingBlinnPhong, vec![point_light2.into(), point_light3.into()])?;
        //let a = real_instancing_object.iter_mut();
        //a.map(|a| a.matmul(&Matrix::translate(Vector::from_1darray([f32::sin(time), 0., 0.]))));
        //let _ = real_instancing_object.iter_mut().map(|a| a.matmul(&Matrix::translate(Vector::from_1darray([f32::sin(time), 0., 0.]))));


        //for i in 0..NUM_INSTANCES {
        //    real_instancing_object[i] = real_instancing_object[i].matmul(&Matrix::translate(Vector::from_1darray([0.5*f32::sin(time), 0.5*f32::cos(time), 0.5*f32::tan(time)])).transpose());
        //}
        let k = GraphicsMatrices::<ColumnMajor>::translate(Vector::from_1darray([0.5*f32::sin(time), 0.5*f32::cos(time), 0.0]));
        //real_instancing_object.iter_mut().for_each(|a| *a = a.matmul(&k));
        for m in real_instancing_object.iter_mut() {
            *m = m.matmul(&k);
        }

        real_instancing_object.draw(&render.window.get_opengl_handle(), &mut render.textures, &render.shaders)?;



        render.use_program(ShaderModule::SimpleTransform, vec![])?;
        simple_cube.draw(&render.window.get_opengl_handle());



        render.end_render_actions()?;
    }

    Ok(())
    
}