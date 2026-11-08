//#![allow(warnings)]

// uncomment for release
// #![windows_subsystem = "windows"]


use atmospheric::image_processing::Image;
use atmospheric::enums::{
    CameraMode, ContextError, DataFormat, DrawMode, ImageFormat, LightForm, ObjectColour, ObjectMaterials, ObjectTexture
};
use atmospheric::modules::shader_modules::shader_module_enum::ShaderModule;
use atmospheric::objects::lighting::{Light, PointLight};
use atmospheric::objects::vao_simple_object::SimpleColourObject;
use atmospheric::objects::{
    lighting::{LightCounter, LightingGenerator},
    vao_instancing_object::InstancingObject,
    cube::Cube,
};
use atmospheric::context::Context;

use numeracy::matrices::{ColumnMajor, GraphicsMatrices, Matrix, S2, Grapics};
use numeracy::vectors::Vector;
use numeracy::random::pseudo_random::{self, pseudo_randf64};

use atmospheric::modules::keybinding_modules::keybindings::KeybindingModule;

use crate::planet::Planets;
use crate::sphere::TripleRingSphere;
use crate::star::Star;

mod sphere;
mod planet;
mod star;


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

    const NUM_STARS:usize = 2;
    const NUM_PLANETS_PER_STAR:u8 = 2;

    let max_lights = LightCounter::max_values(1, NUM_STARS as u16, 1);
    let mut lighting_generator = LightingGenerator::init(&max_lights);

    //let mut render = Context::new_default(max_lights)?;
    let mut render = Context::new("example_1", 900, 900, CameraMode::Encompassing, max_lights, keybinding_modules, shader_modules)?;
    render.setup_render();


    let mut stars = vec![];
    for _ in 0..NUM_STARS {
        let m = 100.;
        let centre = [pseudo_randf64(-m, m, 100) as f32, pseudo_randf64(-m, m, 100) as f32, pseudo_randf64(-m, m, 100) as f32];
        let radius = pseudo_randf64(5., 15., 100) as f32;
        let mass = pseudo_randf64(50., 200., 100) as f32;
        let xyz_rgb = Matrix::from_2darray([
            [0.99,0.44,0.],
            [0.89,0.55,0.],
            [0.85,0.72,0.],
        ]);
        stars.push(Star::new(centre, radius, xyz_rgb, mass, &mut lighting_generator, &render)?);
    }
    
    let mut planets_vec: Vec<Planets<NUM_STARS>> = vec![];
    for star in &stars {
        for _ in 0..NUM_PLANETS_PER_STAR {
            planets_vec.push(
                Planets::new(
                    &render.window.get_opengl_handle(),
                    &star,
                    Matrix::from_2darray([
                        [pseudo_randf64(0., 1., 100) as f32, pseudo_randf64(0., 1., 100) as f32, pseudo_randf64(0., 1., 100) as f32],
                        [pseudo_randf64(0., 1., 100) as f32, pseudo_randf64(0., 1., 100) as f32, pseudo_randf64(0., 1., 100) as f32],
                        [pseudo_randf64(0., 1., 100) as f32, pseudo_randf64(0., 1., 100) as f32, pseudo_randf64(0., 1., 100) as f32]
                    ])
                )
            );
        }
    }




    while !render.render_over() {

        render.begin_render_actions()?;



        // simple shader for light source
        render.use_program(ShaderModule::SimpleOrthographic, vec![])?;

        for star in &stars {
            star.drawing_data.draw(&render.window.get_opengl_handle());
        }


        
        let lights = stars.iter().map(|s| s.light.into()).collect::<Vec<Light>>();
        render.use_program(ShaderModule::InstancingBlinnPhong, lights.clone())?;

        render.shaders.set_orthographic_camera_uniforms(&render.window, &render.camera)?;
        render.shaders.set_blinn_phong_uniforms(&render.window, &render.camera)?;
        for light in lights {
            light.set_lighting_uniforms(&render.window, &render.shaders)?
        }


        for planets in &planets_vec {
            planets.drawing_data.draw(&render.window.get_opengl_handle(), &mut render.textures, &render.shaders)?;
        }




        render.end_render_actions()?;
    }

    Ok(())
    
}