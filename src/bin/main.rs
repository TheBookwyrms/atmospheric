//#![allow(warnings)]

// uncomment for release
// #![windows_subsystem = "windows"]


use atmospheric::image_processing::Image;
use atmospheric::enums::{
    CameraMode, ContextError, ImageFormat, LightForm,
    ObjectColour, ObjectMaterials, ObjectTexture
};
use atmospheric::objects::lighting::Light;
use atmospheric::objects::{
    lighting::{LightCounter, LightingGenerator},
    vao_instancing_object::InstancingObject,
    cube::Cube,
};
use atmospheric::materials::Material;
use atmospheric::opengl::abstractions::ProgramSelect;
use atmospheric::context::Context;

use numeracy::matrices::{Matrix, S2};
use numeracy::vectors::Vector;
use numeracy::random::pseudo_random;

use std::f32::consts::PI;
use std::sync::Arc;


use atmospheric::modules::keybindings::{KeybindingModule, Keybindings};


fn main() -> Result<(), ContextError> {

    let keybinding_modules = Keybindings::using(vec![
        KeybindingModule::CameraZoomScroll, KeybindingModule::CameraPanLeftClick, KeybindingModule::CameraRotateRightClick,
        KeybindingModule::PauseSpace, KeybindingModule::ScreenshotCtrlK,
        KeybindingModule::CloseWindowEscape, KeybindingModule::NecessaryWindowStuff,
        KeybindingModule::ChangeCameraModeE,
        KeybindingModule::CameraPanWASD, KeybindingModule::CameraRotateUpDownLeftRight,
    ]);

    let max_lights = LightCounter::max_values(1, 10, 10);
    let mut lighting_generator = LightingGenerator::init(&max_lights);

    //let mut render = Context::new_default(max_lights)?;
    let mut render = Context::new("window name", 900, 900, CameraMode::Encompassing, max_lights, keybinding_modules)?;
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



        let rotate = Matrix::rotate(Vector::from_1darray([rx, ry, rz]));
        let translate = Matrix::translate(Vector::from_1darray([tx, ty, tz]));

        // transpose this as opengl uses column-major format
        // while i use row-major format for these matrices
        transformation_matrices[i] = (translate.matmul(&rotate)).transpose();
        colour_matrices[i] = Matrix::from_2darray([[cx, cy, cz, ca]]);


    }



    let container_diffuse_map    = Image::decode_from_path("assets/container_diffuse_map.png",  ImageFormat::PNG,   true);
    let container_specular_map   = Image::decode_from_path("assets/container_specular_map.png", ImageFormat::PNG,   true);




    let real_instancing_object = InstancingObject::new(
        &render.window.get_opengl_handle(),
        cube.get_position_matrix(),
        cube.get_normal_matrix(),
        //ObjectColour::None,
        //ObjectColour::Constant(Matrix::from_2darray([[1., 1., 1., 1.]])),
        ObjectColour::ConstantPerInstance(colour_matrices),
        //ObjectColour::PerVertex(ccnt_col),
        //ObjectTextureCoords::PerVertex(ccnt_tex),
        ObjectMaterials::None,
        //ObjectMaterials::Constant(Material::Emerald),
        //ObjectMaterials::Constant(Material::Silver),
        ObjectTexture::None,
        //ObjectTexture::PerVertex(container_diffuse_map, container_specular_map, cube.get_texture_matrix()),
        transformation_matrices
    );

    
    
    while !render.render_over() {

        render.begin_render_actions()?;

        let time = render.window.get_time_since_glfw_init() as f32;
        

        let tr = 0.5;
        // simple shader for light source
        render.use_program(ProgramSelect::SelectSimpleOrthographic, vec![])?;

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

        

        render.use_program(ProgramSelect::SelectInstancingBlinnPhong, vec![point_light2.into(), point_light3.into()])?;
        
        real_instancing_object.draw(&render.window.get_opengl_handle(), &mut render.textures, &render.programs)?;





        render.end_render_actions()?;
    }

    Ok(())
}