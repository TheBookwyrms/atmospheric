use std::{clone, f32::consts::{E, SQRT_2}};

use numeracy::matrices::Matrix;
use numeracy::vectors::Vector;
use numeracy::enums::MatrixError;

use crate::enums::{CameraAxis, CameraMode};


pub struct CameraInfoMatrix {
    ///  matrix containing the information pertaining to the camera's position and directional axes
    /// column 1 = camera position
    /// column 2 = camera target
    /// column 3 = camera right
    /// column 4 = camera up
    camera_matrix:Matrix<f32>
}

impl CameraInfoMatrix {
    pub fn get_camera_matrix(&self) -> Matrix<f32> {
        self.camera_matrix.clone()
    }
    pub fn instantiate(position:Vector<f32>, target:Vector<f32>, right:Vector<f32>, up:Vector<f32>) -> Self {
        Self {
            camera_matrix : Matrix {
                shape:vec![4, 4],
                array:[
                    position.array,
                    target.array,
                    right.array,
                    up.array
                ].concat()
            }.transpose().unwrap()
        }
    }

    pub fn update(&mut self, position:Vector<f32>, target:Vector<f32>, right:Vector<f32>, up:Vector<f32>) {
        self.camera_matrix = Matrix {
            shape:vec![4, 4],
            array:[
                position.array,
                target.array,
                right.array,
                up.array
            ].concat()
        }.transpose().unwrap()
    }

    pub fn matmul_by_left(&mut self, left:Matrix<f32>) -> Result<(), MatrixError> {
        self.camera_matrix = left.matmul(&self.camera_matrix)?;
        Ok(())
    }
    pub fn translate_position(&mut self, translation:Vector<f32>) {
        let (tx, ty, tz) = (translation[0], translation[1], translation[2]);
        let translation_mat = Matrix::from_2darray([
            [tx, 0., 0., 0.],
            [ty, 0., 0., 0.],
            [tz, 0., 0., 0.],
            [0., 0., 0., 0.],
        ]);
        self.camera_matrix += translation_mat;
    }
    pub fn translate_position_target(&mut self, translation:Vector<f32>) {
        let (tx, ty, tz) = (translation[0], translation[1], translation[2]);
        let translation_mat = Matrix::from_2darray([
            [tx, tx, 0., 0.],
            [ty, ty, 0., 0.],
            [tz, tz, 0., 0.],
            [0., 0., 0., 0.],
        ]);
        self.camera_matrix += translation_mat;
    }
    pub fn multiply_right_by_value(&mut self, value:f32) {
        self.camera_matrix[[2, 0]] *= value;
        self.camera_matrix[[2, 1]] *= value;
        self.camera_matrix[[2, 2]] *= value;
    }
    pub fn set_position(&mut self, position:Vector<f32>) {
        self.camera_matrix[[0, 0]] = position[0];
        self.camera_matrix[[0, 1]] = position[1];
        self.camera_matrix[[0, 2]] = position[2];
    }
    pub fn set_right(&mut self, position:Vector<f32>) {
        self.camera_matrix[[2, 0]] = position[0];
        self.camera_matrix[[2, 1]] = position[1];
        self.camera_matrix[[2, 2]] = position[2];
    }
    pub fn set_up(&mut self, position:Vector<f32>) {
        self.camera_matrix[[3, 0]] = position[0];
        self.camera_matrix[[3, 1]] = position[1];
        self.camera_matrix[[3, 2]] = position[2];
    }

    pub fn get_camera_position(&self) -> Vector<f32> {
        self.camera_matrix.get_col(0).unwrap().to_vector().unwrap()
    }
    pub fn get_camera_target(&self) -> Vector<f32> {
        self.camera_matrix.get_col(1).unwrap().to_vector().unwrap()
    }
    pub fn get_camera_right(&self) -> Vector<f32> {
        self.camera_matrix.get_col(2).unwrap().to_vector().unwrap()
    }
    pub fn get_camera_up(&self) -> Vector<f32> {
        self.camera_matrix.get_col(3).unwrap().to_vector().unwrap()
    }

    pub fn get_camera_position3d(&self) -> Vector<f32> {
        Vector::from_slice(&self.get_camera_position().array[0..3])
    }
    pub fn get_camera_target3d(&self) -> Vector<f32> {
        Vector::from_slice(&self.get_camera_target().array[0..3])
    }
    pub fn get_camera_right3d(&self) -> Vector<f32> {
        Vector::from_slice(&self.get_camera_right().array[0..3])
    }
    pub fn get_camera_up3d(&self) -> Vector<f32> {
        Vector::from_slice(&self.get_camera_up().array[0..3])
    }
}




pub struct Camera {
    pub render_distance:u32,
    pub angle_xyz:Vector<f32>,
    pub pan_xyz:Vector<f32>,
    pub zoom:f32,
    pub pan_sensitivity:f32,
    pub angle_sensitivity:f32,
    pub panning:bool, pub angling:bool,
    pub background_colour:(f32, f32, f32),
    //pub camera_position:Vector<f32>,
    //pub camera_target:Vector<f32>,
    //pub camera_up:Vector<f32>,
    //pub camera_right:Vector<f32>,
    pub camera_mode:CameraMode,
    pub camera_info_matrix:CameraInfoMatrix,
}

impl Camera {
    pub fn new(camera_mode:CameraMode) -> Camera {

        let camera_position = Vector::from_1darray([0.0, 0.0, 10.0, 1.0]);
        let camera_target = Vector::from_1darray([0.0, 0.0, -10.0, 1.0]);
        let camera_up = Vector::from_1darray([0.0, 1.0, 0.0, 1.0]);
        let camera_right = Vector::from_1darray([1.0, 0.0, 0.0, 1.0]);

        Camera {
            render_distance:512,
            //angle_xyz:Vector::from_1darray([90.0, 0.0, 0.0]),
            angle_xyz:Vector::from_1darray([0.0, 0.0, 0.0]),
            pan_xyz:Vector::from_1darray([0.0, 0.0, 0.0]),
            //pan_xyz:(0.0, 0.0, -80.0),
            zoom:20.0,
            //zoom:5.0,
            //zoom:1.0,
            pan_sensitivity:0.001,
            angle_sensitivity:0.01,
            panning:false, angling:false,
            background_colour:(0.5, 0.5, 0.5),
            //camera_position:Vector::from_1darray([0.0, 0.0, -10.0]),
            //camera_target:Vector::from_1darray([0.0, 0.0, 10.0]),
            //camera_position: Vector::from_1darray([0.0, 0.0,  10.0]),
            //camera_target  : Vector::from_1darray([0.0, 0.0, -10.0]),
            //camera_up      : Vector::from_1darray([0.0, 1.0,   0.0]),
            //camera_right   : Vector::from_1darray([1.0, 0.0,   0.0]),
            camera_mode:camera_mode,
            camera_info_matrix:CameraInfoMatrix::instantiate(
                camera_position, camera_target, camera_right, camera_up
            ),
        }
    }

    //pub fn update_camera_info_matrix(&mut self) {
    //    self.camera_info_matrix.update(
    //        self.camera_position.clone().extend(vec![1.0]), self.camera_target.clone().extend(vec![1.0]),
    //        self.camera_right.clone().extend(vec![1.0]), self.camera_up.clone().extend(vec![1.0])
    //    );
    //}

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

    pub fn get_look_at_matrix(&self) -> Result<Matrix<f32>, MatrixError> {

        //let cam_pos = Vector::from_slice(&self.camera_info_matrix.get_camera_position().array[0..3]);
        //let cam_target = Vector::from_slice(&self.camera_info_matrix.get_camera_target().array[0..3]);
        //let cam_up = Vector::from_slice(&self.camera_info_matrix.get_camera_up().array[0..3]);
        //let cam_right = Vector::from_slice(&self.camera_info_matrix.get_camera_right().array[0..3]);
        let cam_pos = self.camera_info_matrix.get_camera_position3d();
        let cam_target = self.camera_info_matrix.get_camera_target3d();
        let cam_up = self.camera_info_matrix.get_camera_up3d();
        let cam_right = self.camera_info_matrix.get_camera_right3d();

        let negative_camera_pos = cam_pos.multiply_by_constant(-1.0);
        let negative_camera_direction = (negative_camera_pos.multiply_by_constant(-1.0) - cam_target)?.normalise()?;
        //let camera_up = self.camera_up.clone().normalise()?;
        let camera_up = cam_up;
        let camera_right = cam_right;
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
        let view_matrix = self.get_look_at_matrix()?;
        Ok(view_matrix)
    }

    /// VERIFIED
    /// VERIFIED
    /// VERIFIED
    /// VERIFIED
    /// VERIFIED
    pub fn translate_independant_of_external_information(&mut self, translation:Vector<f32>) {
        self.camera_info_matrix.translate_position(translation);
        //self.camera_position += translation.clone();
        //self.camera_target   += translation;
        //self.update_camera_info_matrix();
    }



    /// VERIFIED
    /// VERIFIED
    /// VERIFIED
    /// VERIFIED
    /// VERIFIED
    pub fn rotation_about_origin_on_arbitrary_axis(&mut self, axis:Vector<f32>, rotation:f32) -> Result<(), MatrixError> {
        let rotate = Matrix::rotate_about_arbitrary_axis(axis, rotation);
        self.camera_info_matrix.matmul_by_left(rotate)
        //self.camera_position = Vector::from_slice(&rotate.matmul(&self.camera_position.clone().to_matrix().reshape(vec![1, 3])?.expand_along_axis(Matrix::from_2darray([[1.0]]), 1)?)?.array[0..3]);
        //self.camera_target   = Vector::from_slice(&rotate.matmul(&self.camera_target.clone().to_matrix().reshape(vec![1, 3])?.expand_along_axis(Matrix::from_2darray([[1.0]]), 1)?)?.array[0..3]);
        //self.camera_up       = Vector::from_slice(&rotate.matmul(&self.camera_up.clone().to_matrix().reshape(vec![1, 3])?.expand_along_axis(Matrix::from_2darray([[1.0]]), 1)?)?.array[0..3]);
        //self.camera_right    = Vector::from_slice(&rotate.matmul(&self.camera_right.clone().to_matrix().reshape(vec![1, 3])?.expand_along_axis(Matrix::from_2darray([[1.0]]), 1)?)?.array[0..3]);
        //self.update_camera_info_matrix();
        //Ok(())
    }

    /// VERIFIED (mostly)
    /// VERIFIED (mostly)
    /// VERIFIED (mostly)
    /// VERIFIED (mostly)
    /// VERIFIED (mostly)
    pub fn rotation_relative_to_the_target_to_the(&mut self, rotation:f32, axis:CameraAxis) -> Result<(), MatrixError> {
        //let axis0 = match axis {
        //    CameraAxis::Up => Ok(self.camera_right.clone().multiply_by_constant(-1.0)),
        //    CameraAxis::Right => Ok(self.camera_up.clone()),
        //    CameraAxis::Forward => self.camera_position.clone()-self.camera_target.clone(),
        //}?;
        //println!("{}", axis0);
        let axis = match axis {
            CameraAxis::Up => Ok(self.camera_info_matrix.get_camera_right3d().multiply_by_constant(-1.0)),
            CameraAxis::Right => Ok(self.camera_info_matrix.get_camera_up3d()),
            CameraAxis::Forward => self.camera_info_matrix.get_camera_position3d()-self.camera_info_matrix.get_camera_target3d(),
        }?;
        //println!("{}", axis);

        //panic!();
        //let rotate_about_target = Matrix::rotate_around_p_on_arbitrary_axis(self.camera_target.clone(), axis.clone(), rotation)?;
        
        let plain_rotate = Matrix::rotate_about_arbitrary_axis(axis, rotation);

        let target_pos_original = self.camera_info_matrix.get_camera_target3d();
        self.camera_info_matrix.translate_position(target_pos_original.multiply_by_constant(-1.0));
        self.camera_info_matrix.matmul_by_left(plain_rotate)?;
        self.camera_info_matrix.translate_position(target_pos_original);
        Ok(())

        //self.camera_position = Vector::from_slice(&rotate_about_target.matmul(&self.camera_position.clone().to_matrix().reshape(vec![1, 3])?.expand_along_axis(Matrix::from_2darray([[1.0]]), 1)?)?.array[0..3]);
        //self.camera_target   = Vector::from_slice(&rotate_about_target.matmul(&self.camera_target.clone().to_matrix().reshape(vec![1, 3])?.expand_along_axis(Matrix::from_2darray([[1.0]]), 1)?)?.array[0..3]);
        //self.camera_up       = Vector::from_slice(&plain_rotate.matmul(&self.camera_up.clone().to_matrix().reshape(vec![1, 3])?.expand_along_axis(Matrix::from_2darray([[1.0]]), 1)?)?.array[0..3]);
        //self.camera_right    = Vector::from_slice(&plain_rotate.matmul(&self.camera_right.clone().to_matrix().reshape(vec![1, 3])?.expand_along_axis(Matrix::from_2darray([[1.0]]), 1)?)?.array[0..3]);
        
        //println!("{}", self.camera_info_matrix.camera_matrix);
        //println!();
        
        //self.camera_position = Vector::from_slice(&rotate_about_target.matmul(&self.camera_info_matrix.get_camera_position().to_matrix().reshape(vec![1, 4])?)?.array[0..3]);
        //self.camera_target   = Vector::from_slice(&rotate_about_target.matmul(&self.camera_info_matrix.get_camera_target().to_matrix().reshape(vec![1, 4])?)?.array[0..3]);
        //self.camera_up       = Vector::from_slice(&plain_rotate.matmul(&self.camera_info_matrix.get_camera_up().to_matrix().reshape(vec![1, 4])?)?.array[0..3]);
        //self.camera_right    = Vector::from_slice(&plain_rotate.matmul(&self.camera_info_matrix.get_camera_right().to_matrix().reshape(vec![1, 4])?)?.array[0..3]);
        //
        //self.update_camera_info_matrix();
        //Ok(())
    }


    /// VERIFIED
    /// VERIFIED
    /// VERIFIED
    /// VERIFIED
    /// VERIFIED
    pub fn rotation_relative_to_the_origin(&mut self, rotation:Vector<f32>) -> Result<(), MatrixError> {
        let rotate_about_origin = Matrix::rotate(rotation)?;
        self.camera_info_matrix.matmul_by_left(rotate_about_origin)
        //self.camera_position = Vector::from_slice(&rotate_about_origin.matmul(&self.camera_position.clone().to_matrix().reshape(vec![1, 3])?.expand_along_axis(Matrix::from_2darray([[1.0]]), 1)?)?.array[0..3]);
        //self.camera_target = Vector::from_slice(&rotate_about_origin.matmul(&self.camera_target.clone().to_matrix().reshape(vec![1, 3])?.expand_along_axis(Matrix::from_2darray([[1.0]]), 1)?)?.array[0..3]);
        //self.camera_up = Vector::from_slice(&rotate_about_origin.matmul(&self.camera_up.clone().to_matrix().reshape(vec![1, 3])?.expand_along_axis(Matrix::from_2darray([[1.0]]), 1)?)?.array[0..3]);
        //self.camera_right = Vector::from_slice(&rotate_about_origin.matmul(&self.camera_right.clone().to_matrix().reshape(vec![1, 3])?.expand_along_axis(Matrix::from_2darray([[1.0]]), 1)?)?.array[0..3]);
        //
        //self.update_camera_info_matrix();
        //Ok(())
    }

    /// VERIFIED
    /// VERIFIED
    /// VERIFIED
    /// VERIFIED
    /// VERIFIED
    fn move_position_by_delta_on_axis(&self, axis:CameraAxis, delta:f32) -> Result<Vector<f32>, MatrixError> {
        //let current_axis = match axis {
        //    CameraAxis::Forward => Err(MatrixError::InvalidAxis),
        //    //CameraAxis::Forward => Ok((self.camera_position.clone()-self.camera_target.clone())?),
        //    CameraAxis::Right   => Ok(self.camera_right.clone()),
        //    CameraAxis::Up      => Ok(self.camera_up.clone()),
        //}?.normalise()?;
        let current_axis = match axis {
            CameraAxis::Forward => Err(MatrixError::InvalidAxis),
            //CameraAxis::Forward => Ok((self.camera_position.clone()-self.camera_target.clone())?),
            CameraAxis::Right   => Ok(self.camera_info_matrix.get_camera_right3d()),
            CameraAxis::Up      => Ok(self.camera_info_matrix.get_camera_up3d()),
        }?.normalise()?;

        // get the direction from the camera to the target
        // get the radius of the circle
        //let current_direction_vector = (self.camera_position.clone()-self.camera_target.clone())?;
        let current_direction_vector = (self.camera_info_matrix.get_camera_position3d()-self.camera_info_matrix.get_camera_target3d())?;
        let camera_target_radius = current_direction_vector.magnitude();

        // get the delta distance on the axis of motion
        // then get the position vector moved on that axis by the delta movement
        let delta_vector = current_axis.multiply_by_constant(delta);
        let position_plus_delta_vector = (current_direction_vector + delta_vector)?;

        // get the unit vector of the new position vector
        // multiply the unit vector by the desired radius
        // get the new position from the target to the position on the circle around the target
        let new_position_unit_vector = position_plus_delta_vector.normalise()?;
        let new_position_vector_at_radius = new_position_unit_vector.multiply_by_constant(camera_target_radius);
        //let new_position = (self.camera_target.clone() + new_position_vector_at_radius)?;
        let new_position = (self.camera_info_matrix.get_camera_target3d() + new_position_vector_at_radius)?;
        
        Ok(new_position)
    }


    /// VERIFIED
    /// VERIFIED
    /// VERIFIED
    /// VERIFIED
    /// VERIFIED
    pub fn translate_relative_to_the_target(&mut self, forward:f32, right:f32, up:f32) -> Result<(), MatrixError> {

        

        // FORWARDS AXIS
        // adds a distance along the forwards axis to the current vector
        // if this forces the camera to pass the target
        // then the camera is flipped (looking back to the target)
        // and the right is flipped as well, to ensure right handed axes

        //println!("");
        //println!("begin");
        //let current_direction_vector = (self.camera_position.clone()-self.camera_target.clone())?;
        let current_direction_vector = (self.camera_info_matrix.get_camera_position3d()-self.camera_info_matrix.get_camera_target3d())?;
        let current_direction_unit = current_direction_vector.normalise()?;
        //println!("d1 {:?}", current_direction_unit);

        self.camera_info_matrix.translate_position(current_direction_unit.multiply_by_constant(forward));
        //self.camera_position += current_direction_unit.multiply_by_constant(forward);
                
        //let new_direction = (self.camera_position.clone()-self.camera_target.clone())?.normalise()?;
        let new_direction = (self.camera_info_matrix.get_camera_position3d()-self.camera_info_matrix.get_camera_target3d())?.normalise()?;
        if new_direction == current_direction_unit.multiply_by_constant(-1.0) {
            //println!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
            self.camera_info_matrix.multiply_right_by_value(-1.0);
            //self.camera_right = self.camera_right.multiply_by_constant(-1.0);
        }


        //self.update_camera_info_matrix();
        


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


        let new_position_vector = self.move_position_by_delta_on_axis(CameraAxis::Right, right)?;
        //self.camera_position = new_position_vector;
        self.camera_info_matrix.set_position(new_position_vector);

        //let new_camera_dir = (self.camera_position.clone() - self.camera_target.clone())?;
        let new_camera_dir = (self.camera_info_matrix.get_camera_position3d()-self.camera_info_matrix.get_camera_target3d())?;
        //println!("d2 {:?}", new_camera_dir.normalise()?);
        //println!("r  {:?}, u {:?}", self.camera_right, self.camera_up);
        //self.camera_right = self.camera_up.cross_product(&new_camera_dir)?.normalise()?;
        //self.camera_right = new_camera_dir.cross_product(&self.camera_up)?.normalise()?;
        //self.camera_right = new_camera_dir.cross_product(&self.camera_info_matrix.get_camera_up3d())?.normalise()?;
        self.camera_info_matrix.set_right(new_camera_dir.cross_product(&self.camera_info_matrix.get_camera_up3d())?.normalise()?);
        //self.update_camera_info_matrix();


                                                                                          
        
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

        let new_position_vector = self.move_position_by_delta_on_axis(CameraAxis::Up, up)?;

        //self.camera_position = new_position_vector;
        self.camera_info_matrix.set_position(new_position_vector);

        //let new_camera_dir = (self.camera_position.clone() - self.camera_target.clone())?;
        let new_camera_dir = (self.camera_info_matrix.get_camera_position3d()-self.camera_info_matrix.get_camera_target3d())?;

        //println!("d3 {:?}", new_camera_dir.normalise()?);
        //println!("r  {:?}, u {:?}", self.camera_right, self.camera_up);
        //self.camera_up = self.camera_right.cross_product(&new_camera_dir)?.normalise()?;
        //self.camera_up = self.camera_info_matrix.get_camera_right3d().cross_product(&new_camera_dir)?.normalise()?;
        self.camera_info_matrix.set_up(self.camera_info_matrix.get_camera_right3d().cross_product(&new_camera_dir)?.normalise()?);
        //self.camera_up = new_camera_dir.cross_product(&self.camera_right)?.normalise()?;
        //println!("r  {:?}, u {:?}", self.camera_right, self.camera_up);
        //println!("end");
        //println!("");


        //self.update_camera_info_matrix();
        Ok(())
    }
}