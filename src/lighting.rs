use numeracy::matrices::Matrix;


pub struct Lighting {
    pub ambient_strength:f32,
    pub ambient_colour:(f32, f32, f32),
    pub diffuse_strength:f32,
    pub diffuse_base:f32,
    pub light_source_pos:(f32, f32, f32),
    pub light_source_colour:(f32, f32, f32),
    pub specular_strength:f32,
    pub view_vec:(f32, f32, f32, f32), // what the hell is this ???
    //pub camera_viewpos:(f32, f32, f32),
    pub specular_power:u32,
    pub light_y_transform:Matrix<f32>, // mat4
}
impl Lighting {
    pub fn new() -> Lighting {
        Lighting {
            //ambient_strength: 0.25,
            //ambient_strength: 0.1,
            ambient_strength: 0.9,
            ambient_colour: (24.0/255.0, 128.0/255.0, 0.0),
            diffuse_strength: 0.5,
            diffuse_base: 0.2,
            light_source_pos: (0.0, -10.0,0.0), // in world space // what does this mean???
            light_source_colour: (209.0/255.0, 6.0/255.0, 141.0/255.0),
            specular_strength: 0.8,
            view_vec: (0.0, 0.0, 32.0, 1.0),
            //camera_viewpos: (),
            specular_power: 2,
            light_y_transform: Matrix::from_2darray([
                [-1.0,  0.0, 0.0, 0.0],
                [ 0.0, -1.0, 0.0, 0.0],
                [ 0.0,  0.0, 1.0, 0.0],
                [ 0.0,  0.0, 0.0, 1.0],
            ])
        }
    }
}