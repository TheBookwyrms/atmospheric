use std::f32::consts::PI;

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


pub struct PointLight {
    pub light_source_pos:(f32, f32, f32),
    
    /// dim to not overpower other lighting
    pub light_ambient_colour:(f32, f32, f32),
    
    /// the colour we want the light to be
    pub light_diffuse_colour:(f32, f32, f32),

    /// typically (1, 1, 1) to allow material specular properties their expression
    pub light_specular_colour:(f32, f32, f32),

    // attenuation terms
    pub attenuation:Attenuation,
}
impl PointLight {
    pub fn new() -> PointLight {
        PointLight {
            //ambient_colour: (24.0/255.0, 128.0/255.0, 0.0),
            //ambient_colour: (209.0/255.0, 6.0/255.0, 141.0/255.0),
            //ambient_colour: (1., 1., 1.),
            //light_source_pos: (-10.0, 10.0, -10.0),
            light_source_pos: (-10.0, 10.0, 10.0),
            light_ambient_colour:(0.2, 0.2, 0.2),
            //light_diffuse_colour:(209.0/255.0, 6.0/255.0, 141.0/255.0),
            light_diffuse_colour:(0.75, 0.95, 0.65),
            light_specular_colour:(1., 1., 1.),
            //light_source_colour: (209.0/255.0, 6.0/255.0, 141.0/255.0),
            //light_source_colour: (209.0/255.0, 66.0/255.0, 141.0/255.0),
            //light_source_colour: (1., 1., 1.),
            attenuation:Attenuation::new_factor(200.0, 0.66),
        }
    }
}



pub struct DirectionalLight {
    pub light_direction:(f32, f32, f32),
    
    /// dim to not overpower other lighting
    pub light_ambient_colour:(f32, f32, f32),
    
    /// the colour we want the light to be
    pub light_diffuse_colour:(f32, f32, f32),

    /// typically (1, 1, 1) to allow material specular properties their expression
    pub light_specular_colour:(f32, f32, f32),
}
impl DirectionalLight {
    pub fn new() -> DirectionalLight {
        DirectionalLight {
            light_direction: (0., 0., 1.0),
            light_ambient_colour:(0.2, 0.2, 0.2),
            light_diffuse_colour:(0.75, 0.95, 0.65),
            light_specular_colour:(1., 1., 1.),
        }
    }
}



pub struct SpotLight {
    pub light_position:(f32, f32, f32),
    pub light_direction:(f32, f32, f32),
    
    /// dim to not overpower other lighting
    pub light_ambient_colour:(f32, f32, f32),
    
    /// the colour we want the light to be
    pub light_diffuse_colour:(f32, f32, f32),

    /// typically (1, 1, 1) to allow material specular properties their expression
    pub light_specular_colour:(f32, f32, f32),

    pub cos_of_inner_cutoff_angle:f32,
    pub cos_of_outer_cutoff_angle:f32,

    // attenuation terms
    pub attenuation:Attenuation,
}
impl SpotLight {
    pub fn new() -> SpotLight {
        SpotLight {
            light_position: (0., 0., 10.0),
            light_direction: (0., 0., -1.0),
            light_ambient_colour:(0.2, 0.2, 0.2),
            light_diffuse_colour:(0.75, 0.95, 0.65),
            light_specular_colour:(1., 1., 1.),
            //cos_of_cutoff_angle:(PI/4.).cos(),
            cos_of_inner_cutoff_angle:(PI/16.).cos(),
            cos_of_outer_cutoff_angle:(PI/8.).cos(),
            attenuation:Attenuation::new_factor(200.0, 0.66),
        }
    }
}