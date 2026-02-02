use numeracy::matrices::Matrix;
use numeracy::vectors::Vector;
use numeracy::enums::MatrixError;


pub struct Camera {
    pub render_distance:u32,
    pub angle_xyz:Vector<f32>,
    pub pan_xyz:Vector<f32>,
    pub zoom:f32,
    pub pan_sensitivity:f32,
    pub angle_sensitivity:f32,
    pub panning:bool, pub angling:bool,
    pub background_colour:(f32, f32, f32),
    pub camera_position:Vector<f32>,
    pub camera_target:Vector<f32>,
    pub camera_up:Vector<f32>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            render_distance:512,
            //angle_xyz:Vector::from_1darray([90.0, 0.0, 0.0]),
            angle_xyz:Vector::from_1darray([0.0, 0.0, 0.0]),
            pan_xyz:Vector::from_1darray([0.0, 0.0, 0.0]),
            //pan_xyz:(0.0, 0.0, -80.0),
            zoom:20.0,
            //zoom:5.0,
            pan_sensitivity:0.001,
            angle_sensitivity:0.01,
            panning:false, angling:false,
            background_colour:(0.5, 0.5, 0.5),
            camera_position:Vector::from_1darray([0.0, 0.0, -10.0]),
            camera_target:Vector::from_1darray([0.0, 0.0, 10.0]),
            camera_up:Vector::from_1darray([0.0, 1.0, 0.0])
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
        let n = 0.001;
        let f = self.render_distance as f32;
        //let f = 5.1;

        // // from https://www.songho.ca/opengl/gl_projectionmatrix.html#ortho
        // let orthographic_projection = Matrix::from_2darray([
        //     [1.0/r, 0.0, 0.0, 0.0],
        //     [0.0, 1.0/t, 0.0, 0.0],
        //     [0.0, 0.0, -2.0/(f-n),-1.0*(f+n)/(f-n)],
        //     [0.0, 0.0, 0.0, 1.0],
        // ]);

        // // from https://www.songho.ca/opengl/gl_projectionmatrix.html#ortho
        // // transposed - because so is the old one i was using
        let orthographic_projection = Matrix::from_2darray([
            [1.0/r,   0.0,              0.0, 0.0],
            [  0.0, 1.0/t,              0.0, 0.0],
            [  0.0,   0.0,       -2.0/(f-n), 0.0],
            [  0.0,   0.0, -1.0*(f+n)/(f-n), 1.0],
        ]);

        // // what i had from long ago, origin unknown
        // let orthographic_projection = Matrix::from_2darray([
        //     [2.0/(r-l), 0.0, 0.0, 0.0],
        //     [0.0, 2.0/(t-b), 0.0, 0.0],
        //     [0.0, 0.0, -2.0/(f-n), 0.0],
        //     [-1.0*(r+l)/(r-l), -1.0*(t+b)/(t-b), -1.0*(f+n)/(f-n), 1.0],
        // ]);


        
        //let perspective_projection = Matrix::from_2darray([
        //    [n/r, 0.0, 0.0, 0.0],
        //    [0.0, n/t, 0.0, 0.0],
        //    [0.0, 0.0, -1.0*(f+n)/(f-n),-1.0],
        //    [0.0, 0.0, -2.0*f*n/(f-n), 0.0],
        //]);

        orthographic_projection
        //perspective_projection
    }

    pub fn get_camera_transform(&self) -> Result<Matrix<f32>, MatrixError> {
                                                                // CHANGE p TO self.camera_position 
        let camera_rotation = Matrix::rotate_around_p(Vector::null(3), self.angle_xyz.clone())?;
        let camera_pan = Matrix::translate(self.pan_xyz.clone());
        camera_pan.matmul(&camera_rotation)
    }

    pub fn get_look_at_matrix(
        &self, camera_pos:Vector<f32>, target_pos:Vector<f32>, up_in_world_space:Vector<f32>
    ) -> Result<Matrix<f32>, MatrixError> {
        let negative_camera_pos = camera_pos.multiply_by_constant(-1.0);
        let negative_camera_direction = (negative_camera_pos.multiply_by_constant(-1.0) - target_pos)?.normalise()?;
        //let negative_camera_direction = (camera_pos - target_pos)?.normalise()?;
        let camera_right = up_in_world_space.cross_product(&negative_camera_direction)?.normalise()?;
        let camera_up = negative_camera_direction.cross_product(&camera_right)?.normalise()?;

        let r = camera_right;
        let u = camera_up;
        let d = negative_camera_direction;

        let look_at_matrix_left = Matrix::from_2darray([
            [r[0], r[1], r[2], 0.],
            [u[0], u[1], u[2], 0.],
            [d[0], d[1], d[2], 0.],
            [0., 0., 0., 1.],
        ]);
        let look_at_matrix_right = Matrix::translate(negative_camera_pos);
        //let look_at_matrix_right = Matrix::translate(camera_pos.multiply_by_constant(-1.0));

        let look_at_matrix = look_at_matrix_left.matmul(&look_at_matrix_right)?;
        Ok(look_at_matrix)
    }

    pub fn get_camera_view_matrix(&self, glfw_time:f64) -> Result<Matrix<f32>, MatrixError> {
        let radius = 10.0;
        let camx = (glfw_time as f32).sin() * radius;
        let camz = (glfw_time as f32).cos() * radius;

        // rotating
        let target_pos = Vector::from_1darray([camx, 0.0, camz]);
        let camera_pos = Vector::from_1darray([0.0, 0.0, 0.0]);

        // static
        let camera_pos = Vector::from_1darray([0.0, 0.0, -10.0]);
        let target_pos = Vector::from_1darray([0.0, 0.0, 10.0]);

        // actual
        let camera_pos = self.camera_position.clone();
        let target_pos = self.camera_target.clone();


        let up_in_world_space = Vector::from_1darray([0.0, 1.0, 0.0]);
        

        let view_matrix = self.get_look_at_matrix(camera_pos, target_pos, up_in_world_space)?;
        Ok(view_matrix)


    }
}