use numeracy::matrices::{Matrix, S2};


use crate::enums::{
    DataFormat, DrawMode,
    DrawType, AttributeLoc::At,
    UpdateVertexAttrib::{PerInstance, PerVertex},
    ObjectColour
};
use crate::opengl::abstractions::{WithVao, WithVbo};
use crate::opengl::gl::Gl;

use std::sync::LazyLock;


static MAT_ONES_LAZYLOCK:LazyLock<Matrix<f32, 2, S2<4, 1>>> = LazyLock::new(|| Matrix::from_2darray([[1.; 4]]));


 
 
pub struct SimpleColourObject<const NUM_VERTICES:usize> {
   position_matrix:Matrix<f32, 2, S2<3, NUM_VERTICES>>,
   
   _colour_matrix:ObjectColour<1, NUM_VERTICES>,
   transformation_matrix:Matrix<f32, 2, S2<4, 4>>,
   prior_transformation_matrix:Matrix<f32, 2, S2<4, 4>>,
   
   object_vao:u32,
   _position_matrix_vbo:u32,
   _colour_matrix_vbo:u32,
   transformation_matrix_vbo:u32,
}
 
impl<const NUM_VERTICES:usize> SimpleColourObject<NUM_VERTICES> {

    pub fn new(
        opengl:&Gl,
        positions:Matrix<f32, 2, S2<3, NUM_VERTICES>>,
        colour:ObjectColour<1, NUM_VERTICES>,
        transformation:Matrix<f32, 2, S2<4, 4>>,
     ) -> Self {
 

        let dtype_size = positions.dtype_memsize() as i32;

        let with_object_vao = WithVao::new(opengl);

        let with_positions_vbo = WithVbo::new(opengl);
        with_positions_vbo.buffer_data(&positions, DrawType::DynamicDraw);
        DataFormat::Position3(At(0), PerVertex).set_vertex_attribs(opengl, dtype_size);

        let with_colours_vbo = WithVbo::new(opengl);
        match colour {
            ObjectColour::None => {
                // OnceLock as otherwise memory seems to be dropped inconveniently
                with_colours_vbo.buffer_data(&MAT_ONES_LAZYLOCK, DrawType::DynamicDraw);
                DataFormat::Colour4(At(1), PerInstance(1)).set_vertex_attribs(opengl, dtype_size);
            },
            ObjectColour::Constant(ref colour_mat) => {
                with_colours_vbo.buffer_data(&colour_mat, DrawType::DynamicDraw);
                DataFormat::Colour4(At(1), PerInstance(1)).set_vertex_attribs(opengl, dtype_size);
            },
            ObjectColour::ConstantPerInstance(ref colour_mat) => {
                with_colours_vbo.buffer_data(&colour_mat[0], DrawType::DynamicDraw);
                DataFormat::Colour4(At(1), PerInstance(1)).set_vertex_attribs(opengl, dtype_size);
            }
            ObjectColour::PerVertex(ref mat) => {
                with_colours_vbo.buffer_data(&mat, DrawType::DynamicDraw);
                DataFormat::Colour4(At(1), PerVertex).set_vertex_attribs(opengl, dtype_size);
            },
        }

        let with_transformations_vbo = WithVbo::new(opengl);
        with_transformations_vbo.buffer_data(&transformation, DrawType::DynamicDraw);
        // takes up locations 8, 9, 10, 11
        DataFormat::TranformationMatrix4x4(At(8), PerInstance(1)).set_vertex_attribs(opengl, dtype_size);






 
        Self {
            position_matrix: positions,
            _colour_matrix: colour,
            //texture_coords_matrix: texture_coords,
            transformation_matrix: transformation.clone(),
            prior_transformation_matrix: transformation,
 
            object_vao: with_object_vao.get_vao(),
            _position_matrix_vbo: with_positions_vbo.get_vbo(),
            _colour_matrix_vbo: with_colours_vbo.get_vbo(),
            transformation_matrix_vbo: with_transformations_vbo.get_vbo(),
         }
    }
    
    pub fn draw<'a>(&'a self, opengl:&Gl) {

        let with_vao = WithVao::existing(opengl, self.object_vao);
        let with_transformation_vbo = WithVbo::existing(opengl, self.transformation_matrix_vbo);

        if self.transformation_matrix != self.prior_transformation_matrix {
            with_transformation_vbo.buffer_sub_data(&self.transformation_matrix);
        }

        with_vao.draw_instanced(DrawMode::GlTriangles, &self.position_matrix, 1);
    }
}