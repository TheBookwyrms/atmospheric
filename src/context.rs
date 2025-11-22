use std::time::{Duration, Instant};

use crate::enums::{
    BufferBit, DataFormat, DrawType, GlError, Object, ProgramSelect, UniformType
};
use crate::opengl;
use crate::opengl::abstractions::{Programs, Textures, WithObject};
use numeracy::matrices::Matrix;

use glfw;
use glfw::{Action, Key};
use crate::enums::ContextError;
use crate::{camera::Camera};
use crate::lighting::Lighting;
use crate::window::Window;


pub struct Context {
    pub window:Window,
    pub camera:Camera,
    pub lighting:Lighting,
    pub programs:Programs,
    pub textures:Textures,
    pub paused:bool,
    pub pause_time:Instant,
    pub current_time:Instant,
}
impl Context {
    pub fn default() -> Result<Self, ContextError> {
        let window = Window::new_opengl()?;
        let camera = Camera::new();
        let lighting = Lighting::new();

        let programs = Programs::compile(&window.opengl)?;

        Ok(Self {
            window, camera, lighting, programs:programs, textures:Textures::new_empty(),
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
        self.window.clear_to_colour(self.camera.background_colour, 1.0)?;
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
        //println!("_fps {}", fps);
        self.current_time = Instant::now();


        // double buffered window for rendering
        self.window.swap_buffers();

        self.poll_and_perform_polled_events()
    }



    pub fn create_vao_vbo_ebo(&self, vertices:&Matrix<f32>, indices:&Matrix<i32>, format:DataFormat
    ) -> Result<(u32, u32, u32), ContextError> {

        let with_vao = WithObject::new(&self.window.opengl, Object::VAO, format);
        
        let with_vbo = WithObject::new(&self.window.opengl, Object::VBO, format);
        with_vbo.buffer_data(vertices, DrawType::DynamicDraw, Object::VBO)?;

        let with_ebo = WithObject::new(&self.window.opengl, Object::EBO, format);
        with_ebo.buffer_data(indices, DrawType::DynamicDraw, Object::EBO)?;

        with_vao.set_vertex_attribs(vertices.dtype_memsize() as i32)?;

        Ok((with_vao.vao, with_vbo.vbo, with_ebo.ebo))
    }


    pub fn create_vao_vbo(&self, data:&Matrix<f32>, format:DataFormat) -> Result<(u32, u32), ContextError> {
        let with_vao = WithObject::new(&self.window.opengl, Object::VAO, format);
        let with_vbo = WithObject::new(&self.window.opengl, Object::VBO, format);

        with_vbo.buffer_data(data, DrawType::DynamicDraw, Object::VBO)?;

        with_vao.set_vertex_attribs(data.dtype_memsize() as i32)?;

        Ok((with_vao.vao, with_vbo.vbo))
    }


    //pub fn draw<T:Clone>(&self, call:DrawCall, mode:DrawMode, vao:u32, data:&Matrix<T>, format:DataFormat) -> Result<(), RenderError> {
    //    Ok(self.programs.draw(&self.window.opengl, call, mode, vao, data, format)?)
    //}


    pub fn use_program(&mut self, program_type:ProgramSelect) -> Result<(), ContextError> {

        self.programs.use_program(&self.window.opengl, program_type)?;

        match program_type {
            ProgramSelect::SelectSimpleOrthographic => {
                self.set_orthographic_camera_uniforms()?;
            },
            ProgramSelect::SelectBlinnPhongOrthographic => {
                self.set_orthographic_camera_uniforms()?;
                self.set_blinn_phong_uniforms()?;
            },
            ProgramSelect::SelectSimpleTexture => {
                self.set_orthographic_camera_uniforms()?;
            }
        }
        Ok(())
    }

    fn set_orthographic_camera_uniforms(&self) -> Result<(), ContextError> {
        // opengl, id, uniform_name, uniform_type, value
        self.programs.set_uniform(&self.window.opengl, "world_transform", UniformType::Mat4, Matrix::opengl_to_right_handed())?;
        self.programs.set_uniform(&self.window.opengl, "orthographic_projection", UniformType::Mat4,
            self.camera.get_orthographic_projection(self.window.aspect_ratio))?;
        let camera_transform = match self.camera.get_camera_transform() {
            Ok(mat) => Ok(mat),
            Err(error) => Err(GlError::MatrixError(error)),
        }?;
        self.programs.set_uniform(&self.window.opengl, "camera_transformation", UniformType::Mat4,
            camera_transform)?;
        Ok(())
    }



    fn set_blinn_phong_uniforms(&self) -> Result<(), ContextError> {
        self.programs.set_uniform(&self.window.opengl,"ambient_strength", UniformType::Float,
            Matrix::from_scalar(self.lighting.ambient_strength))?;
        self.programs.set_uniform(&self.window.opengl,"ambient_colour", UniformType::Vec3, 
            Matrix::from_1darray(self.lighting.ambient_colour.into()))?;
        Ok(())

        //self.programs.set_uniform(&self.window.opengl,"diffuse_strength", UniformType::Float,
        //    Matrix::from_float(self.lighting.diffuse_strength));
        //self.programs.set_uniform(&self.window.opengl,"diffuse_base", UniformType::Float,
        //    Matrix::from_float(self.lighting.diffuse_base));
        //self.programs.set_uniform(&self.window.opengl,"light_source_pos", UniformType::Vec3,
        //    Matrix::from_1darray(self.lighting.light_source_pos.into()));
        //self.programs.set_uniform(&self.window.opengl,"light_source_colour", UniformType::Vec3,
        //    Matrix::from_1darray(self.lighting.light_source_colour.into()));
        //self.programs.set_uniform(&self.window.opengl,"specular_strength", UniformType::Float,
        //    Matrix::from_float(self.lighting.specular_strength));
        //self.programs.set_uniform(&self.window.opengl,"specular_power", UniformType::Float,
        //    Matrix::from_float(self.lighting.specular_power as f32));
        //let view_vec = self.lighting.view_vec;
        //let view_vec3 = (view_vec.0, view_vec.1, view_vec.2);
        //self.programs.set_uniform(&self.window.opengl,"camera_viewpos", UniformType::Vec3,
        //    Matrix::from_1darray(view_vec3.into()));
        //self.programs.set_uniform(&self.window.opengl,"light_y_transform", UniformType::Mat4,
        //    self.lighting.light_y_transform.clone());
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
                        self.camera.pan_xyz.0 += dx * self.camera.pan_sensitivity * self.camera.zoom;
                            // add dx
                        self.camera.pan_xyz.1 -= dy * self.camera.pan_sensitivity * self.camera.zoom;
                            // subtract dy
                    }
                    if self.camera.angling {
                        self.camera.angle_xyz.0 += dy * self.camera.angle_sensitivity * self.camera.zoom;
                            // y and x are swapped
                        self.camera.angle_xyz.1 += dx * self.camera.angle_sensitivity * self.camera.zoom;
                            // y and x are swapped
                    }

                    self.window.last_cursor_pos = [xpos as f32, ypos as f32];

                    Ok(())
                },

                glfw::WindowEvent::Size(width, height) => {
                    match (width==0) || (height==0) {
                        true => Err(ContextError::GLFWResizeBoundsError((width, height))),
                        false => {
                            self.window.aspect_ratio = width as f32/height as f32;
                            Ok(opengl::intermediate_opengl::viewport(&self.window.opengl, width, height))
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