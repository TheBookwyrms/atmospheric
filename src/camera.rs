use numeracy::matrices::Matrix;
use numeracy::vectors::Vector;
use numeracy::enums::MatrixError;

use crate::camera;
use crate::enums::CameraMode;


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
    pub camera_right:Vector<f32>,
    pub camera_mode:CameraMode,
}

impl Camera {
    pub fn new(camera_mode:CameraMode) -> Camera {
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
            //camera_target:Vector::from_1darray([0.0, 0.0, 0.0]),
            camera_up:Vector::from_1darray([0.0, 1.0, 0.0]),
            camera_right:Vector::from_1darray([1.0, 0.0, 0.0]),
            camera_mode:camera_mode,
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
        let n = match self.camera_mode {
            CameraMode::Encompassing => -1.0 * self.render_distance as f32,
            CameraMode::PointOfView => 0.001,
        };
        let f = self.render_distance as f32;


        // from https://www.songho.ca/opengl/gl_projectionmatrix.html#ortho
        // transposed due to linear algebra notation
        let orthographic_projection = Matrix::from_2darray([
            [       2.0/(r-l),              0.0,              0.0, 0.0],
            [             0.0,        2.0/(t-b),              0.0, 0.0],
            [             0.0,              0.0,       -2.0/(f-n), 0.0],
            [-1.0*(r+l)/(r-l), -1.0*(t+b)/(t-b), -1.0*(f+n)/(f-n), 1.0],
        ]);

        orthographic_projection
    }

    pub fn get_look_at_matrix(
        &self, camera_pos:Vector<f32>, target_pos:Vector<f32>, up_in_world_space:Vector<f32>
    ) -> Result<Matrix<f32>, MatrixError> {
        let negative_camera_pos = camera_pos.multiply_by_constant(-1.0);
        let negative_camera_direction = (negative_camera_pos.multiply_by_constant(-1.0) - target_pos)?.normalise()?;
        //let camera_up = self.camera_up.clone().normalise()?;
        let camera_up = self.camera_up.clone();
        let camera_right = self.camera_right.clone();
        //let camera_right = camera_up.clone().cross_product(&negative_camera_direction)?.normalise()?;
        //let camera_right = camera_up.clone().cross_product(&negative_camera_direction)?;
        //let camera_right = up_in_world_space.cross_product(&negative_camera_direction)?.normalise()?;
        //let camera_up = negative_camera_direction.cross_product(&camera_right)?.normalise()?;

        let r = camera_right;
        let u = camera_up;
        let d = negative_camera_direction;
        //println!("r = {}", r);
        //println!("u = {}", u);
        //println!("d = {}", d);
        //println!("");

        let look_at_matrix_left = Matrix::from_2darray([
            [r[0], r[1], r[2], 0.],
            [u[0], u[1], u[2], 0.],
            [d[0], d[1], d[2], 0.],
            [0., 0., 0., 1.],
        ]);
        let look_at_matrix_right = Matrix::translate(negative_camera_pos);

        let look_at_matrix = look_at_matrix_left.matmul(&look_at_matrix_right)?;
        Ok(look_at_matrix)
    }

    pub fn get_camera_view_matrix(&self) -> Result<Matrix<f32>, MatrixError> {

        let camera_pos = self.camera_position.clone();
        let target_pos = self.camera_target.clone();

        let up_in_world_space = Vector::from_1darray([0.0, 1.0, 0.0]);

        let view_matrix = self.get_look_at_matrix(camera_pos, target_pos, up_in_world_space)?;
        Ok(view_matrix)
    }

    pub fn translate_independant_of_external_information(&mut self, translation:Vector<f32>) {
        self.camera_position += translation.clone();
        self.camera_target += translation;
    }
    pub fn rotation_independant_of_external_information(&mut self, rotation:Vector<f32>) -> Result<(), MatrixError> {
        let rotate_about_camera = Matrix::rotate_around_p(self.camera_position.clone(), rotation)?;
        self.camera_position = rotate_about_camera.matmul(&self.camera_position.clone().to_matrix())?.to_vector()?;
        self.camera_target = rotate_about_camera.matmul(&self.camera_target.clone().to_matrix())?.to_vector()?;
        self.camera_up = rotate_about_camera.matmul(&self.camera_up.clone().to_matrix())?.to_vector()?;
        self.camera_right = rotate_about_camera.matmul(&self.camera_right.clone().to_matrix())?.to_vector()?;
        Ok(())
    }
    pub fn rotation_relative_to_the_target(&mut self, rotation:Vector<f32>) -> Result<(), MatrixError> {
        let rotate_about_target = Matrix::rotate_around_p(self.camera_target.clone(), rotation)?;
        self.camera_position = rotate_about_target.matmul(&self.camera_position.clone().to_matrix())?.to_vector()?;
        self.camera_target = rotate_about_target.matmul(&self.camera_target.clone().to_matrix())?.to_vector()?;
        self.camera_up = rotate_about_target.matmul(&self.camera_up.clone().to_matrix())?.to_vector()?;
        self.camera_right = rotate_about_target.matmul(&self.camera_right.clone().to_matrix())?.to_vector()?;
        Ok(())
    }
    pub fn rotation_relative_to_the_origin(&mut self, rotation:Vector<f32>) -> Result<(), MatrixError> {
        let rotate_about_origin = Matrix::rotate(rotation)?;
        self.camera_position = rotate_about_origin.matmul(&self.camera_position.clone().to_matrix())?.to_vector()?;
        self.camera_target = rotate_about_origin.matmul(&self.camera_target.clone().to_matrix())?.to_vector()?;
        self.camera_up = rotate_about_origin.matmul(&self.camera_up.clone().to_matrix())?.to_vector()?;
        self.camera_right = rotate_about_origin.matmul(&self.camera_right.clone().to_matrix())?.to_vector()?;
        Ok(())
    }
    pub fn translate_relative_to_the_target(&mut self, forward:f32, right:f32, up:f32) -> Result<(), MatrixError> {

        

        // FORWARDS AXIS
        // adds a distance along the forwards axis to the current vector
        // if this forces the camera to pass the target
        // then the camera is flipped (looking back to the target)
        // and the right is flipped as well, to ensure right handed axes

        let current_direction_vector = (self.camera_position.clone()-self.camera_target.clone())?;
        let current_direction_unit = current_direction_vector.normalise()?;

        self.camera_position += current_direction_unit.multiply_by_constant(forward);
        let new_direction = (self.camera_position.clone()-self.camera_target.clone())?.normalise()?;

        if new_direction == current_direction_unit.multiply_by_constant(-1.0) {
            self.camera_right = self.camera_right.multiply_by_constant(-1.0);
        }


        


        // RIGHT AXIS
        // take camera pos p(x, y, z)
        // along the axes forward and right
        // up is normal to the plane
        // thus remains constant
        //
        // define points:
        //     T(target pos),     P(current_camera_pos)
        //     Q(new_camera_pos), R(current_camera_pos + Δright)
        // 
        // we want the vector <TQ>
        // if you take the unit vector <TR>/||<TR>
        // and multiply it by the radius of the circle
        // you get <TQ>
        // as in <TQ> = r * <TR>/||<TR>
        // <TR> is just <T(P+R)> - <TP>



        // get the direction from the camera to the target
        // get the radius of the circle
        let current_direction_vector = (self.camera_position.clone()-self.camera_target.clone())?;
        let camera_target_radius = current_direction_vector.magnitude()?;

        // get the delta distance to the right
        // then get the position vector moved to the right by a delta right
        let dright = self.camera_right.multiply_by_constant(right);
        let position_vector_plus_right = (current_direction_vector.clone() + dright)?;

        // get the unit vector of the right position vector
        // multiply the unit vector by the desired radius
        let position_plus_right_unit = position_vector_plus_right.normalise()?;
        let position_plus_right_radius = position_plus_right_unit.multiply_by_constant(camera_target_radius);

        // get the new position vector from the target to the position which is plus right and on the circle
        let new_position_vector = (self.camera_target.clone() + position_plus_right_radius.clone())?;
        self.camera_position = new_position_vector;

        // get the new right direction, and normalise it
        self.camera_right = ((self.camera_position.clone() - self.camera_target.clone())?).cross_product(&self.camera_up)?
                                                                                          .normalise()?;


                                                                                          
        
        // UP AXIS
        // take camera pos p(x, y, z)
        // along the axes forward and up
        // right is normal to the plane
        // thus remains constant
        //
        // define points:
        //     T(target pos),     P(current_camera_pos)
        //     Q(new_camera_pos), U(current_camera_pos + Δright)
        // 
        // we want the vector <TQ>
        // if you take the unit vector <TU>/||<TU>
        // and multiply it by the radius of the circle
        // you get <TQ>
        // as in <TQ> = r * <TU>/||<TU>
        // <TU> is just <T(P+U)> - <TP>

        // get the direction from the camera to the target
        // get the radius of the circle
        let current_direction_vector = (self.camera_position.clone()-self.camera_target.clone())?;
        let camera_target_radius = current_direction_vector.magnitude()?;

        // get the delta distance to the up
        // then get the position vector moved to the up by a delta up
        let dup = self.camera_up.multiply_by_constant(up);
        let position_vector_plus_up = (current_direction_vector.clone() + dup)?;

        // get the unit vector of the up position vector
        // multiply the unit vector by the desired radius
        let position_plus_up_unit = position_vector_plus_up.normalise()?;
        let position_plus_up_radius = position_plus_up_unit.multiply_by_constant(camera_target_radius);

        // get the new position vector from the target to the position which is plus up and on the circle
        let new_position_vector = (self.camera_target.clone() + position_plus_up_radius.clone())?;
        self.camera_position = new_position_vector;

        // get the new up direction, and normalise it
        let new_camera_dir = (self.camera_position.clone() - self.camera_target.clone())?;
        self.camera_up = self.camera_right.cross_product(&new_camera_dir)?.normalise()?;


        Ok(())
    }
}