use numeracy::matrices::{Matrix, S2};


use crate::{enums::{AttributeLoc::At, DataFormat, DrawMode, DrawType, ObjectColour, UpdateVertexAttrib::{PerInstance, PerVertex}}};
use crate::opengl::abstractions::{WithVao, WithVbo};
use crate::opengl::gl::Gl;

use std::sync::LazyLock;


static MAT_ONES_LAZYLOCK:LazyLock<Matrix<f32, 2, S2<4, 1>>> = LazyLock::new(|| Matrix::from_2darray([[1.; 4]]));
 

#[derive(Debug, Clone)]
pub struct SimpleColourObject<const NUM_VERTICES:usize> {
   origin_position_matrix:Matrix<f32, 2, S2<3, NUM_VERTICES>>,
   
   _colour_matrix:ObjectColour<NUM_VERTICES>,
   transformation_matrix:Matrix<f32, 2, S2<4, 4>>,
   prior_transformation_matrix:Matrix<f32, 2, S2<4, 4>>,
   
   object_vao:u32,
   _oeigin_position_matrix_vbo:u32,
   _colour_matrix_vbo:u32,
   transformation_matrix_vbo:u32,

   draw_mode:DrawMode
}
 
impl<const NUM_VERTICES:usize> SimpleColourObject<NUM_VERTICES> {

    pub fn new(
        opengl:&Gl,
        origin_positions:Matrix<f32, 2, S2<3, NUM_VERTICES>>,
        colour:ObjectColour<NUM_VERTICES>,
        transformation:Matrix<f32, 2, S2<4, 4>>,
        draw_mode:DrawMode,
     ) -> Self {
 

        let dtype_size = origin_positions.dtype_memsize() as i32;

        let with_object_vao = WithVao::new(opengl);

        let with_positions_vbo = WithVbo::new(opengl);
        with_positions_vbo.buffer_data(&origin_positions, DrawType::DynamicDraw);
        DataFormat::Position3(At(0), PerVertex).set_vertex_attribs(opengl, dtype_size);

        let with_colours_vbo = WithVbo::new(opengl);
        match colour {
            ObjectColour::None => {
                // OnceLock as otherwise memory seems to be dropped inconveniently
                with_colours_vbo.buffer_data(&MAT_ONES_LAZYLOCK, DrawType::DynamicDraw);
                DataFormat::Colour4(At(1), PerInstance(1)).set_vertex_attribs(opengl, dtype_size);
            },
            ObjectColour::Constant(ref colour_mat) => {
                with_colours_vbo.buffer_data(colour_mat, DrawType::DynamicDraw);
                DataFormat::Colour4(At(1), PerInstance(1)).set_vertex_attribs(opengl, dtype_size);
            },
            ObjectColour::ConstantPerInstance(ref colour_mat) => {
                with_colours_vbo.buffer_data(&colour_mat[0], DrawType::DynamicDraw);
                DataFormat::Colour4(At(1), PerInstance(1)).set_vertex_attribs(opengl, dtype_size);
            }
            ObjectColour::PerVertexPerInstance(ref mat) => {
                todo!()
            }
            ObjectColour::PerVertex(ref mat) => {
                with_colours_vbo.buffer_data(mat, DrawType::DynamicDraw);
                DataFormat::Colour4(At(1), PerVertex).set_vertex_attribs(opengl, dtype_size);
            },
        }

        let with_transformations_vbo = WithVbo::new(opengl);
        with_transformations_vbo.buffer_data(&transformation, DrawType::DynamicDraw);
        // takes up locations 2, 3, 4, 5
        DataFormat::TranformationMatrix4x4(At(2), PerInstance(1)).set_vertex_attribs(opengl, dtype_size);






 
        Self {
            origin_position_matrix: origin_positions,
            _colour_matrix: colour,
            //texture_coords_matrix: texture_coords,
            transformation_matrix: transformation.clone(),
            prior_transformation_matrix: transformation,
 
            object_vao: with_object_vao.get_vao(),
            _oeigin_position_matrix_vbo: with_positions_vbo.get_vbo(),
            _colour_matrix_vbo: with_colours_vbo.get_vbo(),
            transformation_matrix_vbo: with_transformations_vbo.get_vbo(),

            draw_mode,
         }
    }
    
    pub fn draw(&self, opengl:&Gl) {

        let with_vao = WithVao::existing(opengl, self.object_vao);
        let with_transformation_vbo = WithVbo::existing(opengl, self.transformation_matrix_vbo);

        if self.transformation_matrix != self.prior_transformation_matrix {
            with_transformation_vbo.buffer_sub_data(&self.transformation_matrix);
        }

        with_vao.draw_instanced(self.draw_mode, &self.origin_position_matrix, 1);
    }
}