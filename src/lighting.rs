use crate::config::RenderInitialConfig;
use crate::context::Context;
use crate::enums::{ContextError, DataFormat, DrawCall, DrawMode, LightForm, LightSourceForm, Object, UniformType};
use crate::opengl::abstractions2::WithObject;
use numeracy::matrices::Matrix;
use numeracy::vectors::Vector;

#[derive(Clone, Copy)]
pub struct LightCounter {
    directional_lights:u16,
    point_lights:u16,
    spot_lights:u16,
}
impl LightCounter {
    pub const fn new_from(directional:u16, point:u16, spot:u16) -> LightCounter {
        LightCounter { directional_lights: directional, point_lights: point, spot_lights: spot }
    }
    pub fn new_empty() -> LightCounter {
        LightCounter { directional_lights: 0, point_lights: 0, spot_lights: 0 }
    }
    pub fn get_light_count(&self, light:LightSourceForm) -> u16 {
        match light {
            LightSourceForm::Directional => self.directional_lights,
            LightSourceForm::Point => self.point_lights,
            LightSourceForm::Spot => self.spot_lights,
        }
    }
    pub fn increment_light_count(&mut self, light:LightSourceForm) {
        match light {
            LightSourceForm::Directional => self.directional_lights += 1,
            LightSourceForm::Point => self.point_lights += 1,
            LightSourceForm::Spot => self.spot_lights += 1,
        }
    }
}


pub struct LightingGenerator {
    max_lights:LightCounter,

    num_lights_made:LightCounter,
    
}


impl LightingGenerator{
    pub fn init(max_lights:&LightCounter) -> LightingGenerator {


        let dir_max   = max_lights.get_light_count(LightSourceForm::Directional);
        let point_max = max_lights.get_light_count(LightSourceForm::Point);
        let spot_max  = max_lights.get_light_count(LightSourceForm::Spot);


        LightingGenerator {
            max_lights:LightCounter::new_from(dir_max, point_max, spot_max),
            num_lights_made:LightCounter::new_empty(),
        }
    }
    pub fn get_next_light_idx_if_valid(&self, light:LightSourceForm) -> Result<u16, ContextError> {
        if self.num_lights_made.get_light_count(light) < self.max_lights.get_light_count(light) {
            let next_light_idx = self.num_lights_made.get_light_count(light);
            Ok(next_light_idx)
        } else {
            match light {
                LightSourceForm::Directional => Err(ContextError::MaxDirectionalLightsGenerated),
                LightSourceForm::Point => Err(ContextError::MaxPointLightsGenerated),
                LightSourceForm::Spot => Err(ContextError::MaxSpotLightsGenerated),
            }
            //Err(ContextError::MaxDirectionalLightsGenerated)
        }
        //match light {
        //    LightSourceForm::Directional => {
        //        if self.num_lights_made.get_light_count(light) < self.max_lights.get_light_count(light) {
        //            let next_directional_idx = self.num_lights_made.get_light_count(light);
        //            Ok(next_directional_idx)
        //        } else {
        //            Err(ContextError::MaxDirectionalLightsGenerated)
        //        }
        //    },
        //    LightSourceForm::Point => {
        //        if self.num_point_lights_made < self.max_point_lighs {
        //            let next_point_idx = self.num_point_lights_made;
        //            Ok(next_point_idx)
        //        } else {
        //            Err(ContextError::MaxPointLightsGenerated)
        //        }
        //    },
        //    LightSourceForm::Spot => {
        //        if self.num_spot_lights_made < self.max_spot_lighs {
        //            let next_spot_idx = self.num_spot_lights_made;
        //            Ok(next_spot_idx)
        //        } else {
        //            Err(ContextError::MaxSpotLightsGenerated)
        //        }
        //    },
        //}
    }
    pub fn generate_point_light(
        &mut self, render:&Context, position:[f32;3], diffuse_colour:[f32;3]
    ) -> Result<PointLight, ContextError> {

        let next_idx = self.get_next_light_idx_if_valid(LightSourceForm::Point)?;
        self.num_lights_made.increment_light_count(LightSourceForm::Point);
        //self.num_point_lights_made += 1;


        //ambient_colour: (24.0/255.0, 128.0/255.0, 0.0),
        //ambient_colour: (209.0/255.0, 6.0/255.0, 141.0/255.0),
        //ambient_colour: (1., 1., 1.),
        //light_source_pos: (-10.0, 10.0, -10.0),
        //let position     = [-10.0, 10.0, 10.0];
        //light_diffuse_colour:(209.0/255.0, 6.0/255.0, 141.0/255.0),
        //let diffuse_colour  = [0.75, 0.95, 0.65];
        let ambient_colour  = [0.1*diffuse_colour[0], 0.1*diffuse_colour[1], 0.1*diffuse_colour[2]];
        let specular_colour = [1., 1., 1.];
        //light_source_colour: (209.0/255.0, 6.0/255.0, 141.0/255.0),
        //light_source_colour: (209.0/255.0, 66.0/255.0, 141.0/255.0),
        //light_source_colour: (1., 1., 1.),
        let attenuation = Attenuation::new_factor(200.0, 0.66);

        let (vao, vbo) = render.create_vao_vbo(
            &Matrix::from_vector(
                Vector::from_vec([position, diffuse_colour].concat()).extend([1.0])
            ), DataFormat::Position3Colour3Alpha1
        )?;

        Ok(
            PointLight {
                position,
                ambient_colour,
                diffuse_colour,
                specular_colour,
                attenuation,
                vao,
                vbo,
                previous_position:position,
                previous_diffuse_colour:diffuse_colour,
                point_light_index:next_idx
            }
        )
    }
    pub fn generate_directional_light(
        &mut self, direction:[f32;3], diffuse_colour:[f32;3]
    ) -> Result<DirectionalLight, ContextError> {

        let next_idx = self.get_next_light_idx_if_valid(LightSourceForm::Directional)?;
        self.num_lights_made.increment_light_count(LightSourceForm::Directional);
        //self.num_directional_lights_made += 1;

        let ambient_colour  = [0.1*diffuse_colour[0], 0.1*diffuse_colour[1], 0.1*diffuse_colour[2]];
        let specular_colour = [1., 1., 1.];

        Ok(
            DirectionalLight {
                direction,
                ambient_colour,
                diffuse_colour,
                specular_colour,
                directional_light_index: next_idx
            }
        )
    }
    /// takes angles in radians
    pub fn generate_spot_light(
        &mut self, position:[f32;3], direction:[f32;3], diffuse_colour:[f32;3],
        inner_cone_angle:f32, outer_cone_angle:f32,
    ) -> Result<SpotLight, ContextError> {

        let next_idx = self.get_next_light_idx_if_valid(LightSourceForm::Spot)?;
        self.num_lights_made.increment_light_count(LightSourceForm::Spot);
        //self.num_spot_lights_made += 1;

        let ambient_colour  = [0.1*diffuse_colour[0], 0.1*diffuse_colour[1], 0.1*diffuse_colour[2]];
        let specular_colour = [1., 1., 1.];

        let cos_of_inner_cutoff_angle = inner_cone_angle.cos();
        let cos_of_outer_cutoff_angle = outer_cone_angle.cos();

        Ok(
            SpotLight {
                position,
                direction,
                ambient_colour,
                diffuse_colour,
                specular_colour,
                cos_of_inner_cutoff_angle,
                cos_of_outer_cutoff_angle,
                //cos_of_cutoff_angle:(PI/4.).cos(),
                //cos_of_inner_cutoff_angle:(PI/16.).cos(),
                //cos_of_outer_cutoff_angle:(PI/8.).cos(),
                attenuation:Attenuation::new_factor(200.0, 0.66),
                spot_light_index:next_idx,
            }
        )
    }
}


#[derive(Clone, Copy)]
/// attenuation factor = a/(a-d^2)
/// where d = light position - fragment position
/// and a = p^2 * f/(1-f)
/// where p is 1/4 the ideal range distance
/// and f is the fraction of light at the distance p
pub struct Attenuation {
    pub ideal_range:f32,
    pub fraction_at_quarter_range:f32,
    attenuation_factor:f32,
}
impl Attenuation {
    pub fn new_factor(range:f32, fraction:f32) -> Attenuation {
        Attenuation {
            ideal_range: range,
            fraction_at_quarter_range:fraction,
            attenuation_factor: (range/4.).powf(2.)*fraction/(1.-fraction)
        }
    }
    pub fn get_attenuation_factor(&self) -> f32 { self.attenuation_factor }
}


#[derive(Clone, Copy)]
pub struct PointLight {
    position:[f32; 3],
    
    pub ambient_colour:[f32; 3],
    pub diffuse_colour:[f32; 3],
    pub specular_colour:[f32; 3],
    pub attenuation:Attenuation,

    vao:u32,
    vbo:u32,
    previous_position:[f32;3],
    previous_diffuse_colour:[f32;3],
    point_light_index:u16,

}
impl PointLight {
    pub fn set_lighting_uniforms(&self, context:&Context) -> Result<(), ContextError> {
        context.programs.set_uniform(
            &context.window.opengl,
            &format!("point_lights[{}].position", self.point_light_index),
            UniformType::Vec3,
            Matrix::from_1darray(self.position)
        )?;
        context.programs.set_uniform(
            &context.window.opengl,
            &format!("point_lights[{}].ambient_colour", self.point_light_index),
            UniformType::Vec3,
            Matrix::from_1darray(self.ambient_colour)
        )?;
        context.programs.set_uniform(
            &context.window.opengl,
            &format!("point_lights[{}].diffuse_colour", self.point_light_index),
            UniformType::Vec3,
            Matrix::from_1darray(self.diffuse_colour)
        )?;
        context.programs.set_uniform(
            &context.window.opengl,
            &format!("point_lights[{}].specular_colour", self.point_light_index),
            UniformType::Vec3,
            Matrix::from_1darray(self.specular_colour)
        )?;
        context.programs.set_uniform(
            &context.window.opengl,
            &format!("point_lights[{}].attenuation_factor", self.point_light_index),
            UniformType::Float,
            Matrix::from_scalar(self.attenuation.get_attenuation_factor())
        )?;
        Ok(())
    }
    pub fn get_vertex_data(&self) -> Matrix<f32> {
        let mut arr = [self.position, self.diffuse_colour].concat();
        arr.push(1.);
        Matrix::from_vec(arr).new_axis()
    }
    pub fn translate(&mut self, translation:[f32; 3]) {
        self.position[0] += translation[0];
        self.position[1] += translation[1];
        self.position[2] += translation[2];
    }
    pub fn get_position(&self) -> [f32;3] { self.position }
    pub fn set_light(&mut self, light:LightForm, colour:[f32;3]) {
        match light {
            LightForm::Ambient  => self.ambient_colour  = colour,
            LightForm::Diffuse  => self.diffuse_colour  = colour,
            LightForm::Specular => self.specular_colour = colour,
        }
    }
    pub fn draw(&self, render:&Context) -> Result<(), ContextError> {

        let with_light_source = WithObject::existing(
            &render.window.opengl, Object::VAO, self.vao, DataFormat::Position3Colour3Alpha1
        ).add(Object::VBO, self.vbo)?;

        let data = self.get_vertex_data();

        if (self.position != self.previous_position) || (self.diffuse_colour != self.previous_diffuse_colour) {
            with_light_source.buffer_sub_data(&data, Object::VBO)?;
        }
        
        render.programs.draw(with_light_source, DrawCall::Arrays, DrawMode::GlPoints, &data)?;
        Ok(())
    }
}



#[derive(Clone, Copy)]
pub struct DirectionalLight {
    pub direction:[f32; 3],
    
    pub ambient_colour:[f32; 3],
    pub diffuse_colour:[f32; 3],
    pub specular_colour:[f32; 3],

    directional_light_index:u16,
}
impl DirectionalLight {
    pub fn set_lighting_uniforms(&self, context:&Context) -> Result<(), ContextError> {
        context.programs.set_uniform(
            &context.window.opengl,
            &format!("directional_lights[{}].direction", self.directional_light_index),
            UniformType::Vec3,
            Matrix::from_1darray(self.direction)
        )?;
        context.programs.set_uniform(
            &context.window.opengl,
            &format!("directional_lights[{}].ambient_colour", self.directional_light_index),
            UniformType::Vec3,
            Matrix::from_1darray(self.ambient_colour)
        )?;
        context.programs.set_uniform(
            &context.window.opengl,
            &format!("directional_lights[{}].diffuse_colour", self.directional_light_index),
            UniformType::Vec3,
            Matrix::from_1darray(self.diffuse_colour)
        )?;
        context.programs.set_uniform(
            &context.window.opengl,
            &format!("directional_lights[{}].specular_colour", self.directional_light_index),
            UniformType::Vec3,
            Matrix::from_1darray(self.specular_colour)
        )?;
        Ok(())
    }
}



#[derive(Clone, Copy)]
pub struct SpotLight {
    pub position:[f32; 3],
    pub direction:[f32; 3],
    
    /// dim to not overpower other lighting
    pub ambient_colour:[f32; 3],
    
    /// the colour we want the light to be
    pub diffuse_colour:[f32; 3],

    /// typically (1, 1, 1) to allow material specular properties their expression
    pub specular_colour:[f32; 3],

    pub cos_of_inner_cutoff_angle:f32,
    pub cos_of_outer_cutoff_angle:f32,

    // attenuation terms
    pub attenuation:Attenuation,

    spot_light_index:u16,
}
impl SpotLight {
    pub fn set_lighting_uniforms(&self, context:&Context) -> Result<(), ContextError> {
        context.programs.set_uniform(
            &context.window.opengl,
            &format!("spot_lights[{}].position", self.spot_light_index),
            UniformType::Vec3,
            Matrix::from_1darray(self.position)
        )?;
        context.programs.set_uniform(
            &context.window.opengl,
            &format!("spot_lights[{}].direction", self.spot_light_index),
            UniformType::Vec3,
            Matrix::from_1darray(self.direction)
        )?;
        context.programs.set_uniform(
            &context.window.opengl,
            &format!("spot_lights[{}].ambient_colour", self.spot_light_index),
            UniformType::Vec3,
            Matrix::from_1darray(self.ambient_colour)
        )?;
        context.programs.set_uniform(
            &context.window.opengl,
            &format!("spot_lights[{}].diffuse_colour", self.spot_light_index),
            UniformType::Vec3,
            Matrix::from_1darray(self.diffuse_colour)
        )?;
        context.programs.set_uniform(
            &context.window.opengl,
            &format!("spot_lights[{}].specular_colour", self.spot_light_index),
            UniformType::Vec3,
            Matrix::from_1darray(self.specular_colour)
        )?;
        context.programs.set_uniform(
            &context.window.opengl,
            &format!("spot_lights[{}].inner_cutoff_angle", self.spot_light_index),
            UniformType::Float,
            Matrix::from_scalar(self.cos_of_inner_cutoff_angle)
        )?;
        context.programs.set_uniform(
            &context.window.opengl,
            &format!("spot_lights[{}].outer_cutoff_angle", self.spot_light_index),
            UniformType::Float,
            Matrix::from_scalar(self.cos_of_outer_cutoff_angle)
        )?;
        context.programs.set_uniform(
            &context.window.opengl,
            &format!("spot_lights[{}].attenuation_factor", self.spot_light_index),
            UniformType::Float,
            Matrix::from_scalar(self.attenuation.get_attenuation_factor())
        )?;
        Ok(())
    }
    pub fn translate(&mut self, translation:[f32; 3]) {
        self.position[0] += translation[0];
        self.position[1] += translation[1];
        self.position[2] += translation[2];
    }
    pub fn rotate(&mut self, rotation:[f32; 3]) -> Result<(), ContextError> {
        let direction = Vector::from_1darray(self.direction).extend([0.0]);
        let rotation = Matrix::rotate(Vector::from_1darray(rotation))?;
        rotation.matmul(&Matrix::from_vector(direction))?;
        Ok(())
    }
}