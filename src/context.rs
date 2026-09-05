use std::time::{Duration, Instant};

use crate::opengl_helpers::camera::Camera;
use crate::opengl_helpers::config::RenderInitialConfig;
use crate::opengl_helpers::enums::{
    BufferBit, CameraMode, DataFormat, DrawType, GlError, Object, UniformType, ContextError, CameraVector
};
use crate::lighting::LightingGenerator;
use crate::opengl::intermediate_opengl;
//use crate::opengl::abstractions::{Programs, Textures, Uniform, WithObject};
use crate::opengl::abstractions::{Programs, ProgramSelect, Textures, Uniform, WithVao, WithVbo, WithEbo, WithVaoVbo, WithVaoEbo};

//use numeracy::matrices::Matrix;
use numeracy::matrices::{Matrix, S2, ShapeTrait};

use glfw;
use glfw::{Action, Key};
use crate::window::Window;


pub struct Context<'a> {
    pub window:Window,
    pub camera:Camera,
    pub programs:Programs,
    pub textures:Textures<'a>,
    pub lighting_generator:LightingGenerator,
    pub paused:bool,
    pub pause_time:Instant,
    pub current_time:Instant,
}
impl<'a> Context<'a> {
    pub fn default(config:RenderInitialConfig) -> Result<Self, ContextError> {
        let window = Window::new_opengl(config.window_name, config.window_width, config.window_height)?;
        let camera = Camera::new(config.camera_mode);
        //let camera = Camera::new(CameraMode::PointOfView);
        //let camera = Camera::new(CameraMode::Encompassing);

        //panic!("add info for number of lights by passing it through config");
        let programs = Programs::compile_all(&window.opengl, &config.max_lights)?;
        let textures = Textures::new_empty();

        let lighting_generator = LightingGenerator::init(&config.max_lights);

        Ok(Self {
            window, camera, programs, textures, lighting_generator,
            paused:false, pause_time:Instant::now(), current_time:Instant::now(),
         })
    }
    pub fn render_over(&self) -> bool { self.window.window.should_close() }
    pub fn poll_events(&mut self) { self.window.poll_events(); }


    pub fn setup_render(&mut self) {
        self.window.default_gl_settings();
        self.window.make_current();
        self.window.set_polling();
    }

    pub fn begin_render_actions(&self) -> Result<(), ContextError> {
        self.window.clear_to_colour(self.window.background_colour, 1.0)?;
        self.window.clear(vec![BufferBit::ColourBufferBit, BufferBit::DepthBufferBit]);
        Ok(())

    }
    
    pub fn end_render_actions(&mut self) -> Result<(), ContextError> {
        
        self.textures.deactivate_all(&self.window.opengl);
        self.programs.disuse_program(&self.window.opengl);

        
        let dt = match Instant::now().duration_since(self.current_time).as_secs_f32() {
            0.0 => 0.0,
            t => t,};
        //println!("dt {}", dt);
        let _fps = 1.0/dt;
        //println!("_fps {}", _fps);
        self.current_time = Instant::now();


        // double buffered window for rendering
        self.window.swap_buffers();

        self.poll_and_perform_polled_events()
    }



    pub fn create_vao_vbo_ebo<U:ShapeTrait<2>, V:ShapeTrait<2>>(&self, vertices:&Matrix<f32, 2, U>, indices:&Matrix<i32, 2, V>, format:DataFormat
    ) -> (u32, u32, u32) {

        let with_vao = WithVao::new(&self.window.opengl);//, format);
        
        let with_vbo = WithVbo::new(&self.window.opengl);//, format);
        with_vbo.buffer_data(vertices, DrawType::DynamicDraw);

        let with_ebo = WithEbo::new(&self.window.opengl, format);
        with_ebo.buffer_data(indices, DrawType::DynamicDraw);

        with_vao.set_vertex_attribs_per_vertex(vertices.dtype_memsize() as i32, format);

        (with_vao.get_vao(), with_vbo.get_vbo(), with_ebo.get_ebo())
    }


    pub fn create_vao_vbo<const N:usize, U:ShapeTrait<N>>(&self, data:&Matrix<f32, N, U>, format:DataFormat) -> (u32, u32) {
        let with_vao = WithVao::new(&self.window.opengl);//, format);
        let with_vbo = WithVbo::new(&self.window.opengl);//, format);

        with_vbo.buffer_data(data, DrawType::DynamicDraw);

        with_vao.set_vertex_attribs_per_vertex(data.dtype_memsize() as i32, format);

        (with_vao.get_vao(), with_vbo.get_vbo())
    }



    //pub fn create_vao_vbo_ebo<U:ShapeTrait<2>, V:ShapeTrait<2>>(&self, vertices:&Matrix<f32, 2, U>, indices:&Matrix<i32, 2, V>, format:DataFormat
    //) -> Result<(u32, u32, u32), ContextError> {
//
    //    let with_vao = WithObject::new(&self.window.opengl, Object::VAO, format);
    //    
    //    let with_vbo = WithObject::new(&self.window.opengl, Object::VBO, format);
    //    with_vbo.buffer_data(vertices, DrawType::DynamicDraw, Object::VBO)?;
//
    //    let with_ebo = WithObject::new(&self.window.opengl, Object::EBO, format);
    //    with_ebo.buffer_data(indices, DrawType::DynamicDraw, Object::EBO)?;
//
    //    with_vao.set_vertex_attribs(vertices.dtype_memsize() as i32)?;
//
    //    Ok((with_vao.get_vao(), with_vbo.get_vbo(), with_ebo.get_ebo()))
    //}
//
//
    //pub fn create_vao_vbo<const N:usize, U:ShapeTrait<N>>(&self, data:&Matrix<f32, N, U>, format:DataFormat) -> Result<(u32, u32), ContextError> {
    //    let with_vao = WithObject::new(&self.window.opengl, Object::VAO, format);
    //    let with_vbo = WithObject::new(&self.window.opengl, Object::VBO, format);
//
    //    with_vbo.buffer_data(data, DrawType::DynamicDraw, Object::VBO)?;
//
    //    with_vao.set_vertex_attribs(data.dtype_memsize() as i32)?;
//
    //    Ok((with_vao.get_vao(), with_vbo.get_vbo()))
    //}


    pub fn set_custom_uniform<T:Clone, const N:usize, U:ShapeTrait<N>>(&self, program_id:u32, uniform:Uniform, value:Matrix<T, N, U>) -> Result<(), GlError> {
        intermediate_opengl::set_uniform(&self.window.opengl, program_id, uniform.name, uniform.uniform_type, value.as_ptr())
    }

    pub fn compile_custom_program(&mut self, vertex_text:&str, fragment_text:&str) -> Result<u32, GlError> {
        Programs::compile_program_from_text(&self.window.opengl, vertex_text, fragment_text)
    }

    pub fn use_custom_program(&mut self, shader_id:u32) -> Result<(), GlError> {
        self.programs.use_program(&self.window.opengl, ProgramSelect::Custom(shader_id))
    }


    pub fn use_program(&mut self, program_type:ProgramSelect) -> Result<(), ContextError> {

        self.programs.use_program(&self.window.opengl, program_type)?;

        match program_type {
            ProgramSelect::SelectSimpleOrthographic => {
                self.set_orthographic_camera_uniforms()?;
            },
            ProgramSelect::SelectPhongOrthographic => {
                self.set_orthographic_camera_uniforms()?;
                self.set_blinn_phong_uniforms()?;
            },
            ProgramSelect::SelectPhongTexture => {
                self.set_orthographic_camera_uniforms()?;
                self.set_blinn_phong_uniforms()?;
            },
            ProgramSelect::SelectInstancingBlinnPhong => {
                self.set_orthographic_camera_uniforms()?;
                self.set_blinn_phong_uniforms()?;
            },
            //ProgramSelect::SelectInstancingPhongTexture => {
            //    self.set_orthographic_camera_uniforms()?;
            //    self.set_blinn_phong_uniforms()?;
            //},
            //ProgramSelect::SelectInstancingFull => {
            //    self.set_orthographic_camera_uniforms()?;
            //    self.set_blinn_phong_uniforms()?;
            //},
            ProgramSelect::SelectSimpleTexture | ProgramSelect::SelectTwoTexture => {
                self.set_orthographic_camera_uniforms()?;
            },
            ProgramSelect::Custom(_) => Err(GlError::InvalidCustomProgramSelect)?
        }
        Ok(())
    }

    pub fn set_world_transform_uniform(&self, transform:Matrix<f32, 2, S2<4, 4>>) -> Result<(), ContextError> {
        
        let model_transform = Matrix::opengl_to_right_handed().matmul(&transform);

        self.programs.set_uniform(&self.window.opengl, "world_transform",
            UniformType::Mat4, model_transform)?;
        
        Ok(())
    }

    pub fn set_orthographic_camera_uniforms(&self) -> Result<(), ContextError> {
        // opengl, id, uniform_name, uniform_type, value

        // model
        self.set_world_transform_uniform(Matrix::identity())?;

        // view
        self.programs.set_uniform(&self.window.opengl, "camera_transformation", UniformType::Mat4,
            //self.camera.get_camera_transform()?)?;
            self.camera.get_camera_view_matrix()?)?;

        // projection
        self.programs.set_uniform(&self.window.opengl, "orthographic_projection", UniformType::Mat4,
            self.camera.get_orthographic_projection(self.window.aspect_ratio))?;

        Ok(())
    }



    pub fn set_blinn_phong_uniforms(&self) -> Result<(), ContextError> {



            
        self.programs.set_uniform(&self.window.opengl,"camera_viewpos", UniformType::Vec3,
            Matrix::from_vector(
                self.camera.camera_info_matrix.get_camera(CameraVector::Position)
            ))?;


        Ok(())

    }




    fn poll_and_perform_polled_events(&mut self) -> Result<(), ContextError> {
        self.poll_events();
        for (_, event) in glfw::flush_messages(&self.window.events) {
            match event {

                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    {let _ = &self.window.window.set_should_close(true); Ok(())}
                },
                glfw::WindowEvent::Key(Key::Space, _, Action::Press, _) => {
                    match self.paused {
                        false => {self.paused=true; self.pause_time=Instant::now()},
                        true => if Instant::now().duration_since(self.pause_time) > Duration::from_millis(10) {self.paused=false},
                    };
                    Ok(())
                },

                glfw::WindowEvent::Close => {
                    {let _ = &self.window.window.set_should_close(true); Ok(())}
                },
                
                glfw::WindowEvent::MouseButton(button, action, _mods) => {
                    {
                    match action {
                        Action::Press => {
                            match button {
                                glfw::MouseButton::Button1 => {self.camera.panning = true}, // left button
                                glfw::MouseButton::Button2 => {self.camera.angling = true}, // right button
                                _ => {},
                            }
                        },
                        Action::Release => {
                            match button {
                                glfw::MouseButton::Button1 => {self.camera.panning = false}, // left button
                                glfw::MouseButton::Button2 => {self.camera.angling = false}, // right button
                                _ => {},
                            }
                        },
                        Action::Repeat => {},
                    };
                    Ok(())
                    }
                },

                glfw::WindowEvent::Scroll(_xoffset, yoffset) => {
                    {self.camera.zoom -= ((0.24*yoffset) as f32) * self.camera.zoom*0.25; Ok(())}
                },

                glfw::WindowEvent::CursorPos(xpos, ypos) => {
                    let dx = xpos as f32 - self.window.last_cursor_pos[0];
                    let dy = ypos as f32 - self.window.last_cursor_pos[1];

                    if self.camera.panning {
                        let sensitivity = self.camera.pan_sensitivity * self.camera.zoom;
                        self.camera.translation_by_internal_axes(0.0, -dx*sensitivity, dy*sensitivity)?;
                        //self.camera.pan_xyz += Vector::from_1darray([dx, -1.0*dy, 0.0])
                        //                                .multiply_by_constant(sensitivity);
                    }
                    if self.camera.angling {
                        let sensitivity = self.camera.angle_sensitivity * self.camera.zoom;
                        self.camera.translate_relative_to_the_target(0.0, -dx*sensitivity, dy*sensitivity)?;
                        //self.camera.angle_xyz += Vector::from_1darray([dy, dx, 0.0])
                        //                                .multiply_by_constant(sensitivity);
                    }

                    self.window.last_cursor_pos = [xpos as f32, ypos as f32];

                    Ok(())
                },

                glfw::WindowEvent::Size(width, height) => {
                    match (width==0) || (height==0) {
                        true => {
                            //glfw::Window::iconify(&mut self);
                            self.window.window.iconify();
                            Ok(())
                        },
                        //true => Err(ContextError::GLFWResizeBoundsError((width, height))),
                        false => {
                            self.window.aspect_ratio = width as f32/height as f32;
                            Ok(intermediate_opengl::viewport(&self.window.opengl, width, height))
                        },
                    }
                },

                glfw::WindowEvent::Key(_, _, _, _) => {Ok(())},
                glfw::WindowEvent::Char(_) => {Ok(())},
                glfw::WindowEvent::CharModifiers(_, _) => {Ok(())},
                glfw::WindowEvent::Focus(_) => {Ok(())},
                glfw::WindowEvent::Pos(_, _) => {Ok(())},
                glfw::WindowEvent::FramebufferSize(_, _) => {Ok(())},
                glfw::WindowEvent::Iconify(_) => {Ok(())},
                glfw::WindowEvent::Maximize(_) => {Ok(())},
                glfw::WindowEvent::Refresh => {Ok(())},
                glfw::WindowEvent::CursorEnter(_) => {Ok(())},
                _ => Err(ContextError::NewGLFWEventDetected(event)),
            }?;
        }
        Ok(())
    }
}