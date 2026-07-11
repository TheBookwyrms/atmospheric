use numeracy::matrices::Matrix;
use numeracy::vectors::Vector;
use numeracy::enums::MatrixError;

use crate::enums::{CameraAxis, CameraMode, CameraVector};


pub struct CameraInfoMatrix {
    ///  matrix containing the information pertaining to the camera's position and directional axes
    /// column 1 = camera position
    /// column 2 = camera target
    /// column 3 = camera right
    /// column 4 = camera up
    camera_matrix:Matrix<f32>
}

impl CameraInfoMatrix {
    pub fn get_camera_matrix(self) -> Matrix<f32> {
        self.camera_matrix
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

    fn matmul_by_left(&mut self, left:Matrix<f32>) -> Result<(), MatrixError> {
        self.camera_matrix = left.matmul(&self.camera_matrix)?;
        Ok(())
    }
    fn translate_position(&mut self, translation:Vector<f32>) {
        let (tx, ty, tz) = (translation[0], translation[1], translation[2]);
        let translation_mat = Matrix::from_2darray([
            [tx, 0., 0., 0.],
            [ty, 0., 0., 0.],
            [tz, 0., 0., 0.],
            [0., 0., 0., 0.],
        ]);
        self.camera_matrix += translation_mat;
    }
    fn translate_position_target(&mut self, translation:Vector<f32>) {
        let (tx, ty, tz) = (translation[0], translation[1], translation[2]);
        let translation_mat = Matrix::from_2darray([
            [tx, tx, 0., 0.],
            [ty, ty, 0., 0.],
            [tz, tz, 0., 0.],
            [0., 0., 0., 0.],
        ]);
        self.camera_matrix += translation_mat;
    }

    fn set_camera(&mut self, vector:CameraVector, value:Vector<f32>) {
        let column_idx = match vector {
            CameraVector::Position => 0,
            CameraVector::Target => 1,
            CameraVector::Right => 2,
            CameraVector::Up => 3,
        };
        
        let (vx, vy, vz) = (value[0], value[1], value[2]);

        self.camera_matrix[[column_idx, 0]] = vx;
        self.camera_matrix[[column_idx, 1]] = vy;
        self.camera_matrix[[column_idx, 2]] = vz;
    }

    pub fn get_camera(&self, vector:CameraVector) -> Vector<f32> {
        let cam_vec_4d = match vector {
            CameraVector::Position => self.camera_matrix.get_col(0),
            CameraVector::Target => self.camera_matrix.get_col(1),
            CameraVector::Right => self.camera_matrix.get_col(2),
            CameraVector::Up => self.camera_matrix.get_col(3),
        }.unwrap().to_vector().unwrap();
        Vector::from_slice(&cam_vec_4d.array[0..3])
    }

    pub fn get_camera_view_vector(&self) -> Vector<f32> {
        let view_vec_4d = (self.camera_matrix.get_col(0).unwrap() - self.camera_matrix.get_col(1).unwrap()).unwrap();
        Vector::from_slice(&view_vec_4d.array[0..3])
    }
}




pub struct Camera {
    pub render_distance:u32,
    pub zoom:f32,
    pub pan_sensitivity:f32,
    pub angle_sensitivity:f32,
    pub panning:bool,
    pub angling:bool,
    pub camera_mode:CameraMode,
    pub camera_info_matrix:CameraInfoMatrix,
}

impl Camera {
    pub fn new(camera_mode:CameraMode) -> Camera {

        let camera_position = Vector::from_1darray([0.0, 0.0, 10.0, 1.0]);
        //let camera_position = Vector::from_1darray([0.0, 0.0, 20.0, 1.0]);
        let camera_target = Vector::from_1darray([0.0, 0.0, -10.0, 1.0]);
        let camera_target = Vector::from_1darray([0.0, 0.0, 0.0, 1.0]);
        let camera_up = Vector::from_1darray([0.0, 1.0, 0.0, 1.0]);
        let camera_right = Vector::from_1darray([1.0, 0.0, 0.0, 1.0]);

        Camera {
            render_distance:512,
            zoom:20.0,
            //zoom:5.0,
            //zoom:1.0,
            //pan_sensitivity  :0.001,
            //angle_sensitivity:0.01,
            pan_sensitivity  :0.0075,
            angle_sensitivity:0.0025,
            panning:false, angling:false,
            camera_mode:camera_mode,
            camera_info_matrix:CameraInfoMatrix::instantiate(
                camera_position, camera_target, camera_right, camera_up
            ),
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

    pub fn get_look_at_matrix(&self) -> Result<Matrix<f32>, MatrixError> {
        let r = self.camera_info_matrix.get_camera(CameraVector::Right);
        let u = self.camera_info_matrix.get_camera(CameraVector::Up);
        let d = self.camera_info_matrix.get_camera_view_vector().normalise()?;

        let look_at_matrix_left = Matrix::from_2darray([
            [r[0], r[1], r[2], 0.],
            [u[0], u[1], u[2], 0.],
            [d[0], d[1], d[2], 0.],
            [  0.,   0.,   0., 1.],
        ]);
        let look_at_matrix_right = Matrix::translate(
            self.camera_info_matrix
                .get_camera(CameraVector::Position)
                .multiply_by_constant(-1.0)
        );

        let look_at_matrix = look_at_matrix_left.matmul(&look_at_matrix_right)?;
        Ok(look_at_matrix)
    }

    pub fn get_camera_view_matrix(&self) -> Result<Matrix<f32>, MatrixError> {
        let view_matrix = self.get_look_at_matrix()?;
        Ok(view_matrix)
    }

    /// translates the camera position and camera target by a value V=<x, y, z>
    /// along the x, y, and z axes
    pub fn translation_by_xyz(&mut self, translation:Vector<f32>) {
        self.camera_info_matrix.translate_position(translation);
    }

    /// translates the camera position and camera target a certain amount
    /// forwards, to the right, and upwards based off of its
    /// internal axes
    pub fn translation_by_internal_axes(&mut self, forward:f32, right:f32, up:f32) -> Result<(), MatrixError> {
        let df = self.camera_info_matrix.get_camera_view_vector().normalise()?.multiply_by_constant(forward);
        let dr = self.camera_info_matrix.get_camera(CameraVector::Right).normalise()?.multiply_by_constant(right);
        let du = self.camera_info_matrix.get_camera(CameraVector::Up).normalise()?.multiply_by_constant(up);
        self.camera_info_matrix.translate_position_target(df);
        self.camera_info_matrix.translate_position_target(dr);
        self.camera_info_matrix.translate_position_target(du);
        Ok(())
    }

    /// rotates the camera about the origin along an arbitrary axis
    pub fn rotation_about_origin(&mut self, axis:Vector<f32>, rotation:f32) -> Result<(), MatrixError> {
        let rotate = Matrix::rotate_about_arbitrary_axis(axis, rotation);
        self.camera_info_matrix.matmul_by_left(rotate)
    }

    /// rotates the camera about the target along its internal axes
    /// as such, the target position is invariant under this transformation
    pub fn rotation_about_target(&mut self, axis:CameraAxis, rotation:f32) -> Result<(), MatrixError> {
        let axis = match axis {
            CameraAxis::Up => self.camera_info_matrix.get_camera(CameraVector::Right).multiply_by_constant(-1.0),
            CameraAxis::Right => self.camera_info_matrix.get_camera(CameraVector::Up),
            CameraAxis::Forward => self.camera_info_matrix.get_camera_view_vector(),
        };
        
        let plain_rotate = Matrix::rotate_about_arbitrary_axis(axis, rotation);

        let target_pos_original = self.camera_info_matrix.get_camera(CameraVector::Target);
        self.camera_info_matrix.translate_position(target_pos_original.multiply_by_constant(-1.0));
        self.camera_info_matrix.matmul_by_left(plain_rotate)?;
        self.camera_info_matrix.translate_position(target_pos_original);
        Ok(())
    }


    pub fn rotation_about_origin_on_xyz(&mut self, rotation:Vector<f32>) -> Result<(), MatrixError> {
        let rotate_about_origin = Matrix::rotate(rotation)?;
        self.camera_info_matrix.matmul_by_left(rotate_about_origin)
    }


    /// a more detailed derivation for the equations used in this function are also present in documentation_resources/derivation_for_camera_move_delta.pdf
    fn move_position_by_delta_on_axis(&self, axis:CameraAxis, delta:f32) -> Result<Vector<f32>, MatrixError> {
        let current_axis = match axis {
            CameraAxis::Forward => Err(MatrixError::InvalidAxis),
            CameraAxis::Right   => Ok(self.camera_info_matrix.get_camera(CameraVector::Right)),
            CameraAxis::Up      => Ok(self.camera_info_matrix.get_camera(CameraVector::Up)),
        }?.normalise()?;

        // get the direction from the camera to the target
        // get the radius of this circle
        let camera_direction = self.camera_info_matrix.get_camera_view_vector();
        let radius = camera_direction.magnitude();

        // get the delta distance on the axis of motion
        // then get the direction vector moved on that axis by the delta movement
        let delta_vector = current_axis.multiply_by_constant(delta);
        let intermediate_direction = (camera_direction + delta_vector)?;

        // get the unit vector of the new direction vector
        // multiply the unit vector by the desired radius to place it back on the circle
        let new_direction = intermediate_direction.normalise()?;
        let new_position_vector_at_radius = new_direction.multiply_by_constant(radius);
        
        // get the change in position by subtracting the old direction from the new direction
        new_position_vector_at_radius - self.camera_info_matrix.get_camera_view_vector()
    }


    pub fn translate_relative_to_the_target(&mut self, forward:f32, right:f32, up:f32) -> Result<(), MatrixError> {

        

        // FORWARDS AXIS
        // adds a distance along the forwards axis to the current vector
        // if this forces the camera to pass the target
        // then the camera is flipped (looking back to the target)
        // and the right is flipped as well, to ensure right handed axes

        self.camera_info_matrix.translate_position(
            self.camera_info_matrix.get_camera_view_vector()
                                                .normalise()?
                                                .multiply_by_constant(-forward) 
                                                // negative to make positive deltas move forwards
        );
        let new_camera_dir = self.camera_info_matrix.get_camera_view_vector();
        //self.camera_info_matrix.set_camera(CameraVector::Right, new_camera_dir.cross_product(&self.camera_info_matrix.get_camera(CameraVector::Up))?.normalise()?);
        self.camera_info_matrix.set_camera(CameraVector::Right, self.camera_info_matrix.get_camera(CameraVector::Up).cross_product(&new_camera_dir)?.normalise()?);



        


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


        self.camera_info_matrix.translate_position(self.move_position_by_delta_on_axis(CameraAxis::Right, right)?);
        let new_camera_dir = self.camera_info_matrix.get_camera_view_vector();
        //self.camera_info_matrix.set_camera(CameraVector::Right, new_camera_dir.cross_product(&self.camera_info_matrix.get_camera(CameraVector::Up))?.normalise()?);
        self.camera_info_matrix.set_camera(CameraVector::Right, self.camera_info_matrix.get_camera(CameraVector::Up).cross_product(&new_camera_dir)?.normalise()?);
        
                                                                        
        
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

        self.camera_info_matrix.translate_position(self.move_position_by_delta_on_axis(CameraAxis::Up, up)?);
        let new_camera_dir = self.camera_info_matrix.get_camera_view_vector();
        //self.camera_info_matrix.set_camera(CameraVector::Up, self.camera_info_matrix.get_camera(CameraVector::Right).cross_product(&new_camera_dir)?.normalise()?);
        self.camera_info_matrix.set_camera(CameraVector::Up, new_camera_dir.cross_product(&self.camera_info_matrix.get_camera(CameraVector::Right))?.normalise()?);
        
        Ok(())
    }
}