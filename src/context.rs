use std::sync::Arc;
use std::time::{Instant, SystemTime};

use crate::camera::Camera;
use crate::enums::{
    BufferBit, CameraMode, CameraVector, ContextError, DataFormat, DrawType, GlError, UniformType
};
use crate::modules::keybinding_modules::keybindings::{KeybindingModule, Keybindings};
use crate::modules::shader_modules::shaders::Shaders;
use crate::modules::shader_modules::shader_module_enum::ShaderModule;
use crate::objects::lighting::{Light, LightCounter};
use crate::opengl::intermediate_opengl;
//use crate::opengl::abstractions::{Programs, Textures, Uniform, WithObject};
use crate::opengl::abstractions::{Textures, Uniform, WithEbo, WithVao, WithVbo};
use crate::opengl_helpers::opengl_data::OpenGLData;

use numeracy::matrices::{Matrix, S2, ShapeTrait};

use glfw::WindowEvent;
use crate::glfw::window::Window;

//use crate::modules::keybindings::{
//    camera_movement::MouseOnlyMovement,
//    pause::PauseKeybindings,
//    screenshot::ScreenshotKeybindings,
//    window::DefaultWindowKeybindings,
//};


pub struct AssortedContextDetails {
    pub paused:bool,
    pub pause_time:Instant,
    /// pause_minimum is minimum duration of a pause, in milliseconds
    pub pause_minimum:u64,
    pub current_time:Instant,
    pub screenshot_naming_convention:Box<dyn Fn()->String>,
}
impl AssortedContextDetails {
    fn default_naming_convention() -> String {
        format!("screenshot_{:?}", SystemTime::duration_since(&SystemTime::now(), SystemTime::UNIX_EPOCH).unwrap().as_secs())
    }
    pub fn default() -> AssortedContextDetails {
        Self {
            paused: false, pause_time: Instant::now(), pause_minimum:10,
            current_time: Instant::now(),
            screenshot_naming_convention: Box::new(Self::default_naming_convention)
        }
    }
    pub fn set_screenshot_naming_convention(&mut self, function:Box<dyn Fn()->String>) {
        self.screenshot_naming_convention = function
    }
}


pub struct Context<'a> {
    pub window:Window,
    pub camera:Camera,
    pub shaders:Shaders<'a>,
    //pub programs:Programs,
    pub textures:Textures,
    pub keybindings:Keybindings,


    pub assorted_details:AssortedContextDetails,
}
impl<'a> Context<'a> {

    pub fn new(window_name:&'static str, window_height:u32, window_width:u32, camera_mode:CameraMode, max_lights:LightCounter, keybindings:Vec<KeybindingModule>, shaders:Vec<ShaderModule<'a>>) -> Result<Self, ContextError> {
        let window = Window::new_opengl(window_name, window_width, window_height)?;
        let camera = Camera::new(camera_mode);

        let shaders_compiled = Shaders::using(window.get_opengl_handle(), shaders, &max_lights)?;
        let keybindings_struct = Keybindings::using(keybindings);
        let textures = Textures::new_empty();

        Ok(Self {
            window, camera, shaders:shaders_compiled, textures, keybindings:keybindings_struct,
            assorted_details:AssortedContextDetails::default(),
         })
    }

    pub fn new_default(max_lights:LightCounter) -> Result<Self, ContextError> {
        let window = Window::new_opengl("window name!", 1920, 1080)?;
        let camera = Camera::new(CameraMode::Encompassing);

        let textures = Textures::new_empty();

        let keybindings = Keybindings::using(vec![
            KeybindingModule::CameraZoomScroll, KeybindingModule::CameraPanLeftClick, KeybindingModule::CameraRotateRightClick,
            KeybindingModule::PauseSpace, KeybindingModule::ScreenshotCtrlK,
            KeybindingModule::CloseWindowEscape, KeybindingModule::NecessaryWindowStuff,
        ]);

        let shaders = Shaders::using(
            window.get_opengl_handle(),
            vec![
                ShaderModule::InstancingBlinnPhong,
                ShaderModule::PhongOrthographic,
                ShaderModule::PhongTexture,
                ShaderModule::SimpleOrthographic,
                ShaderModule::SimpleTexture,
                ShaderModule::TwoTexture,
                //ShaderModule::SimpleTransform,
            ],
            &max_lights
        )?;

        Ok(Self {
            window, camera, shaders, textures, keybindings,
            assorted_details:AssortedContextDetails::default(),
         })
    }
    
    pub fn render_over(&self) -> bool { self.window.should_close() }
    pub fn poll_events(&mut self) { self.window.poll_events(); }


    pub fn setup_render(&mut self) {
        self.window.default_gl_settings();
        self.window.make_current();
        self.window.set_polling();
    }

    pub fn begin_render_actions(&self) -> Result<(), ContextError> {
        self.window.clear_to_colour(self.window.get_background_colour(), 1.0)?;
        self.window.clear(vec![BufferBit::ColourBufferBit, BufferBit::DepthBufferBit]);
        Ok(())

    }
    
    pub fn end_render_actions(&mut self) -> Result<(), ContextError> {
        
        self.textures.deactivate_all(&self.window.get_opengl_handle());
        self.shaders.disuse_program(&self.window.get_opengl_handle());
        //self.programs.disuse_program(&self.window.get_opengl_handle());

        
        let dt = Instant::now().duration_since(self.assorted_details.current_time).as_secs_f32();
        //println!("dt {}", dt);
        let _fps = 1.0/dt;
        //println!("_fps {}", _fps);
        self.assorted_details.current_time = Instant::now();


        // double buffered window for rendering
        self.window.swap_buffers();

        self.poll_and_perform_polled_events()
    }



    pub fn create_vao_vbo_ebo<U:ShapeTrait<2>, V:ShapeTrait<2>>(
        &self, vertices:&Matrix<f32, 2, U>, indices:&Matrix<i32, 2, V>, format:DataFormat
    ) -> (u32, u32, u32) where Matrix<f32, 2, U>: OpenGLData, Matrix<i32, 2, V>: OpenGLData{

        let with_vao = WithVao::new(&self.window.get_opengl_handle());//, format);
        
        let with_vbo = WithVbo::new(&self.window.get_opengl_handle());//, format);
        with_vbo.buffer_data(vertices, DrawType::DynamicDraw);

        let with_ebo = WithEbo::new(&self.window.get_opengl_handle(), format);
        with_ebo.buffer_data(indices, DrawType::DynamicDraw);

        with_vao.set_vertex_attribs_per_vertex(vertices.dtype_memsize() as i32, format);

        (with_vao.get_vao(), with_vbo.get_vbo(), with_ebo.get_ebo())
    }


    pub fn create_vao_vbo<U:ShapeTrait<2>>(
        &self, data:&Matrix<f32, 2, U>, format:DataFormat
    ) -> (u32, u32) where Matrix<f32, 2, U>: OpenGLData {
        let with_vao = WithVao::new(&self.window.get_opengl_handle());//, format);
        let with_vbo = WithVbo::new(&self.window.get_opengl_handle());//, format);

        with_vbo.buffer_data(data, DrawType::DynamicDraw);

        with_vao.set_vertex_attribs_per_vertex(data.dtype_memsize() as i32, format);

        (with_vao.get_vao(), with_vbo.get_vbo())
    }




    /// lights is used for setting lighting uniforms \
    /// such as for blinn-phong lighting, where we need light source uniforms \
    /// if there are no lights needed, pass empty vec \
    pub fn use_program(&mut self, shader:ShaderModule<'a>, lights:Vec<Light>) -> Result<(), ContextError> {

        self.shaders.use_program(&self.window.get_opengl_handle(), shader)?;
        //self.programs.use_program(&self.window.get_opengl_handle(), program_type)?;

        self.shaders.set_uniforms_for_program(shader, &self.window, &self.camera, lights)
    }





    fn poll_and_perform_polled_events(&mut self) -> Result<(), ContextError> {
        self.poll_events();
        //KeybindingModule::CameraPanWASD.keybinding_callback(&WindowEvent::Close, self)?;
        let events = self.window.flush_messages();
        let keybindings = self.keybindings.clone();
        keybindings.invoke(events, self)?;

        self.window.set_last_cursor_pos(self.window.get_cursor_pos());

        Ok(())
    }
}