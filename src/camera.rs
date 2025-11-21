use numeracy::matrices::Matrix;
use numeracy::enums::MatrixError;


pub struct Camera {
    pub render_distance:u32,
    pub angle_xyz:(f32, f32, f32),
    pub pan_xyz:(f32, f32, f32),
    pub zoom:f32,
    pub pan_sensitivity:f32,
    pub angle_sensitivity:f32,
    pub panning:bool, pub angling:bool,
    pub background_colour:(f32, f32, f32),
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            render_distance:512,
            //angle_xyz:(90.0, -90.0, 0.0), // default orientation to view xy plane
            angle_xyz:(90.0, 0.0, 0.0),
            pan_xyz:(0.0, 0.0, 0.0),
            //pan_xyz:(0.0, 0.0, -80.0),
            zoom:20.0,
            pan_sensitivity:0.001,
            angle_sensitivity:0.01,
            panning:false, angling:false,
            background_colour:(0.5, 0.5, 0.5),
        }
    }

    // reference
    // https://www.scratchapixel.com/lessons/3d-basic-rendering/perspective-and-orthographic-projection-matrix/orthographic-projection-matrix.html
    pub fn get_orthographic_projection(&self, aspect_ratio:f32)
                -> Matrix<f32> {
        let l = -1.0 * aspect_ratio * self.zoom;
        let r = aspect_ratio * self.zoom;
        let b = -1.0 * self.zoom;
        let t = self.zoom;
        let n = -1.0 * self.render_distance as f32;
        //let n = 0.1;
        let f = self.render_distance as f32;
        //let f = 0.0;

        let orthographic_projection = Matrix::from_2darray([
            [2.0/(r-l), 0.0, 0.0, 0.0],
            [0.0, 2.0/(t-b), 0.0, 0.0],
            [0.0, 0.0, -2.0/(f-n), 0.0],
            [-1.0*(r+l)/(r-l), -1.0*(t+b)/(t-b), -1.0*(f+n)/(f-n), 1.0],
        ]);

        orthographic_projection
    }

    pub fn get_camera_transform(&self) -> Result<Matrix<f32>, MatrixError> {
        let camera_rotation = Matrix::rotate_around_p((0.0, 0.0, 0.0), self.angle_xyz)?;
        let camera_pan = Matrix::translate(self.pan_xyz);
        camera_pan.matmul(&camera_rotation)
    }
}