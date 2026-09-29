use crate::image_processing::Image;
use crate::materials::Material;
use numeracy::matrices::{Matrix, S1, S2, S3,};


use crate::enums::{
    ContextError, DataFormat, DrawMode,
    DrawType, AttributeLoc::At,
    OpenglTexture, UpdateVertexAttrib::{PerInstance, PerVertex},
    ObjectColour, ObjectMaterials, ObjectTexture
};
use crate::opengl::abstractions::{PreparedTexture, Programs, TextureSetup, Textures, WithVao, WithVbo};
use crate::opengl::gl::Gl;

use std::sync::LazyLock;

#[cfg(target_os = "linux")]
include!(concat!(env!("OUT_DIR"), "/compiled_assets.rs"));

#[cfg(target_os = "windows")]
include!(concat!(env!("OUT_DIR"), "\\compiled_assets.rs"));

static MAT_ONES_LAZYLOCK:LazyLock<Matrix<f32, 2, S2<4, 1>>> = LazyLock::new(|| Matrix::from_2darray([[1.; 4]]));

 
 
 pub struct InstancingObject<'a, const NUM_INSTANCES:usize, const NUM_VERTICES:usize> {
    position_matrix:&'a Matrix<f32, 2, S2<3, NUM_VERTICES>>,
    _normals_matrix:&'a Matrix<f32, 2, S2<3, NUM_VERTICES>>,
    
    _colour_matrix:ObjectColour<NUM_INSTANCES, NUM_VERTICES>,
    //texture_coords_matrix:ObjectTextureCoords<NUM_VERTICES>,
    _materials_matrix:ObjectMaterials<NUM_VERTICES>,

    /// vec of mat4, length N, for position and normals
    //transformation_matrices:Vec<Matrix<f32, 2, S2<4, 4>>>,
    transformation_matrices:[Matrix<f32, 2, S2<4, 4>>;NUM_INSTANCES],
    prior_transformation_matrices:[Matrix<f32, 2, S2<4, 4>>;NUM_INSTANCES],
    
    object_vao:u32,
    _position_matrix_vbo:u32,
    _colour_matrix_vbo:u32,
    _texture_matrix_vbo:u32,
    _normals_matrix_vbo:u32,
    _materials_matrix_vbo:u32,
    transformation_matrices_vbo:u32,

    diffuse_texture:PreparedTexture,
    specular_texture:PreparedTexture,
 }
 
impl<'a, const NUM_INSTANCES:usize, const NUM_VERTICES:usize> InstancingObject<'a, NUM_INSTANCES, NUM_VERTICES> {

    pub fn new(
        opengl          : &Gl,
        positions       : &'a Matrix<f32, 2, S2<3, NUM_VERTICES>>,
        normals         : &'a Matrix<f32, 2, S2<3, NUM_VERTICES>>,
        colour          : ObjectColour<NUM_INSTANCES, NUM_VERTICES>,
        materials       : ObjectMaterials<NUM_VERTICES>,
        texture         : ObjectTexture<NUM_VERTICES>,
        transformations : [Matrix<f32, 2, S2<4, 4>>;NUM_INSTANCES],
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
                DataFormat::Colour4(At(1), PerInstance(transformations.len() as u32)).set_vertex_attribs(opengl, dtype_size);
            },
            ObjectColour::Constant(ref mat) => {
                with_colours_vbo.buffer_data(&mat, DrawType::DynamicDraw);
                DataFormat::Colour4(At(1), PerInstance(transformations.len() as u32)).set_vertex_attribs(opengl, dtype_size);
            },
            ObjectColour::ConstantPerInstance(ref vec_mat) => {
                // just completely lying about the length of this array
                let mut arrs = Vec::with_capacity(NUM_INSTANCES);
                vec_mat.iter().for_each(|m| arrs.push(m.get_view_of_array()));
                let colours = Matrix::from_vec_with_shape(
                    arrs.concat()
                    ,//.concat(),
                    S3::<4, 1, NUM_INSTANCES>
                );
                with_colours_vbo.buffer_data(&colours, DrawType::DynamicDraw);
                DataFormat::Colour4(At(1), PerInstance(1)).set_vertex_attribs(opengl, dtype_size);
            }
            ObjectColour::PerVertex(ref mat) => {
                with_colours_vbo.buffer_data(&mat, DrawType::DynamicDraw);
                DataFormat::Colour4(At(1), PerVertex).set_vertex_attribs(opengl, dtype_size);
            },
        }

        let with_normals_vbo = WithVbo::new(opengl);
        with_normals_vbo.buffer_data(&normals, DrawType::DynamicDraw);
        DataFormat::Normal3(At(2), PerVertex).set_vertex_attribs(opengl, dtype_size);

        let with_texture_coords_vbo = WithVbo::new(opengl);
        let (diffuse_texture, specular_texture) = match texture {
            ObjectTexture::None => {
                let default_diffuse  = TextureSetup::get_prepared_default(opengl, WHITE_PPM.clone());
                let default_specular = TextureSetup::get_prepared_default(opengl, WHITE_PPM.clone());
                
                with_texture_coords_vbo.buffer_data(&Matrix::from_1darray([0.5; 2]), DrawType::DynamicDraw);
                DataFormat::Texture2(At(3), PerInstance(transformations.len() as u32)).set_vertex_attribs(opengl, dtype_size);
                
                (default_diffuse, default_specular)
            },
            ObjectTexture::PerVertex(diffuse_image, specular_image, ref mat) => {
                let diffuse  = TextureSetup::get_prepared_default(opengl, diffuse_image);
                let specular = TextureSetup::get_prepared_default(opengl, specular_image);

                with_texture_coords_vbo.buffer_data(&mat, DrawType::DynamicDraw);
                DataFormat::Texture2(At(3), PerVertex).set_vertex_attribs(opengl, dtype_size);

                (diffuse, specular)
            },
        };

        let with_materials_vbo = WithVbo::new(opengl);
        match materials {
            ObjectMaterials::None => {
                with_materials_vbo.buffer_data(&Matrix::from_1darray(Material::Default.get_material_qualities().get_components_array()), DrawType::DynamicDraw);
                // takes up locations 4, 5, 6, 7
                DataFormat::Material3331(At(4), PerInstance(transformations.len() as u32)).set_vertex_attribs(opengl, dtype_size);
            },
            ObjectMaterials::Constant(material) => {
                let data = Matrix::from_1darray(material.get_material_qualities().get_components_array());
                with_materials_vbo.buffer_data(&data, DrawType::DynamicDraw);
                // takes up locations 4, 5, 6, 7
                DataFormat::Material3331(At(4), PerInstance(transformations.len() as u32)).set_vertex_attribs(opengl, dtype_size);

            },
            ObjectMaterials::PerVertex(ref materials_matrix) => {
                let mut material_values = vec![];
                materials_matrix.array.iter().for_each(
                    |material| {
                        material_values.extend(
                            material.get_material_qualities().get_components_array()
                        );
                    }
                );

                // takes up locations 4, 5, 6, 7
                // also lying about data length, because no const generics
                with_materials_vbo.buffer_data(&Matrix::<f32, 1, S1<1>>::from_vec(material_values), DrawType::DynamicDraw);
                DataFormat::Material3331(At(4), PerVertex).set_vertex_attribs(opengl, dtype_size);
            },
        }

        //let transformation_data = Matrix::from_vec_with_shape(
        //    transformations.map(|t| t.get_view_of_array().to_vec()).concat(),
        //    S3::<4, 4, NUM_INSTANCES>
        //);
        let mut arrs = Vec::with_capacity(NUM_INSTANCES);
        transformations.iter().for_each(|m| arrs.push(m.get_view_of_array()));
        let transformation_data = Matrix::from_vec_with_shape(
            arrs.concat(),
            S3::<4, 4, NUM_INSTANCES>
        );
        let with_transformations_vbo = WithVbo::new(opengl);
        with_transformations_vbo.buffer_data(&transformation_data, DrawType::DynamicDraw);
        // takes up locations 8, 9, 10, 11
        DataFormat::TranformationMatrix4x4(At(8), PerInstance(1)).set_vertex_attribs(opengl, dtype_size);






 
        Self {
            position_matrix: positions,
            _colour_matrix: colour,
            //texture_coords_matrix: texture_coords,
            _normals_matrix: normals,
            _materials_matrix: materials,
            transformation_matrices: transformations.clone(),
            prior_transformation_matrices: transformations,
 
            object_vao: with_object_vao.get_vao(),
            _position_matrix_vbo: with_positions_vbo.get_vbo(),
            _colour_matrix_vbo: with_colours_vbo.get_vbo(),
            _texture_matrix_vbo: with_texture_coords_vbo.get_vbo(),
            _normals_matrix_vbo: with_normals_vbo.get_vbo(),
            _materials_matrix_vbo: with_materials_vbo.get_vbo(),
            transformation_matrices_vbo: with_transformations_vbo.get_vbo(),
 
            diffuse_texture,
            specular_texture,
         }
    }
    
    pub fn draw(&'a self, opengl:&Gl, textures:&mut Textures<'a>, programs:&Programs) -> Result<(), ContextError> {

        textures.activate(
            opengl, OpenglTexture::Texture0, &self.diffuse_texture, programs
        )?;
        textures.activate(
            opengl, OpenglTexture::Texture1, &self.specular_texture, programs
        )?;


        let with_vao = WithVao::existing(opengl, self.object_vao);
        let with_transformations_vbo = WithVbo::existing(opengl, self.transformation_matrices_vbo);


        if self.transformation_matrices != self.prior_transformation_matrices {
            let mut arrs = Vec::with_capacity(NUM_INSTANCES);
            self.transformation_matrices.iter().for_each(|m| arrs.push(m.get_view_of_array()));
            let transformation_data = Matrix::from_vec_with_shape(
                arrs.concat(),
                S3::<4, 4, NUM_INSTANCES>
            );

            with_transformations_vbo.buffer_sub_data(&transformation_data);
        }

        with_vao.draw_instanced(DrawMode::GlTriangles, &self.position_matrix, self.transformation_matrices.len().try_into().unwrap());
    
        Ok(())
    }
}