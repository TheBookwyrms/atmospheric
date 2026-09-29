use numeracy::{matrices::{Matrix, S2}, vectors::Vector};

/// r, g, b, a
fn get_face_colours() -> [[f32; 4]; 6] {    
    let c0 = [1.0, 0.0, 0.0, 1.0];
    let c1 = [0.0, 1.0, 0.0, 1.0];
    let c2 = [0.0, 0.0, 1.0, 1.0];
    let c3 = [1.0, 1.0, 0.0, 1.0];
    let c4 = [1.0, 0.0, 1.0, 1.0];
    let c5 = [0.0, 1.0, 1.0, 1.0];

    //let c0 = [1., 1., 1., 1.0];
    //let c1 = [1., 1., 1., 1.0];
    //let c2 = [1., 1., 1., 1.0];
    //let c3 = [1., 1., 1., 1.0];
    //let c4 = [1., 1., 1., 1.0];
    //let c5 = [1., 1., 1., 1.0];

    [c0, c1, c2, c3, c4, c5]
}

fn get_normals() -> [[f32;3];12] {
    let norm_top1    = [ 0.,  1.,  0.];
    let norm_top2    = [ 0.,  1.,  0.];
    let norm_front1  = [ 0.,  0.,  1.];
    let norm_front2  = [ 0.,  0.,  1.];
    let norm_bottom1 = [ 0., -1.,  0.];
    let norm_bottom2 = [ 0., -1.,  0.];
    let norm_back1   = [ 0.,  0., -1.];
    let norm_back2   = [ 0.,  0., -1.];
    let norm_right1  = [ 1.,  0.,  0.];
    let norm_right2  = [ 1.,  0.,  0.];
    let norm_left1   = [-1.,  0.,  0.];
    let norm_left2   = [-1.,  0.,  0.];

    [
        norm_top1, norm_top2, norm_front1, norm_front2,
        norm_bottom1, norm_bottom2, norm_back1, norm_back2,
        norm_right1, norm_right2, norm_left1, norm_left2,
    ]
}

fn get_tex_face_vals(texture_size:f32) -> [[f32; 2]; 4] {
    let tex_tr = [texture_size, texture_size]; // texture top right
    let tex_br = [texture_size,          0.0]; // texture bottom right
    let tex_bl = [         0.0,          0.0]; // texture bottom left
    let tex_tl = [         0.0, texture_size]; // texture top left

    [tex_tr, tex_br, tex_bl, tex_tl]
}

pub fn create_cube_vertices(centre:(f32, f32, f32), side_len:f32) -> [[f32; 3]; 8] {
    let (x, y, z) = centre;

    let top_front_right    = [x+side_len/2.0, y+side_len/2.0, z+side_len/2.0];
    let top_back_right     = [x+side_len/2.0, y+side_len/2.0, z-side_len/2.0];
    let top_front_left     = [x-side_len/2.0, y+side_len/2.0, z+side_len/2.0];
    let top_back_left      = [x-side_len/2.0, y+side_len/2.0, z-side_len/2.0];
    let bottom_front_right = [x+side_len/2.0, y-side_len/2.0, z+side_len/2.0];
    let bottom_back_right  = [x+side_len/2.0, y-side_len/2.0, z-side_len/2.0];
    let bottom_front_left  = [x-side_len/2.0, y-side_len/2.0, z+side_len/2.0];
    let bottom_back_left   = [x-side_len/2.0, y-side_len/2.0, z-side_len/2.0];


    [
        top_back_right,
        top_front_right,
        top_front_left,
        top_back_left,
        bottom_back_right,
        bottom_front_right,
        bottom_front_left,
        bottom_back_left,
    ]
}


pub struct Cube {
    position_matrix : Matrix<f32, 2, S2<3, 36>>,
    colour_matrix   : Matrix<f32, 2, S2<4, 36>>,
    normal_matrix   : Matrix<f32, 2, S2<3, 36>>,
    texture_matrix  : Matrix<f32, 2, S2<2, 36>>,
}
impl Cube {
    pub fn from_matrices(
        position_matrix : Matrix<f32, 2, S2<3, 36>>,
        colour_matrix   : Matrix<f32, 2, S2<4, 36>>,
        normal_matrix   : Matrix<f32, 2, S2<3, 36>>,
        texture_matrix  : Matrix<f32, 2, S2<2, 36>>
    ) -> Self {
        Self { position_matrix, colour_matrix, normal_matrix, texture_matrix }
    }
    pub fn get_position_matrix(&self) -> &Matrix<f32, 2, S2<3, 36>> { &self.position_matrix }
    pub fn get_colour_matrix(&self)   -> &Matrix<f32, 2, S2<4, 36>> { &self.colour_matrix }
    pub fn get_normal_matrix(&self)   -> &Matrix<f32, 2, S2<3, 36>> { &self.normal_matrix }
    pub fn get_texture_matrix(&self)  -> &Matrix<f32, 2, S2<2, 36>> { &self.texture_matrix }

    pub fn new(centre:(f32, f32, f32), side_len:f32, texture_size:f32) -> Cube {

        let [
            tbr, tfr, tfl, tbl, bbr, bfr, bfl, bbl
        ] = create_cube_vertices(centre, side_len);

        let [c0, c1, c2, c3, c4, c5] = get_face_colours();
        let [tex_tr, tex_br, tex_bl, tex_tl] = get_tex_face_vals(texture_size);

        let [
            ntr1, ntr2, nfr1, nfr2,
            nbo1, nbo2, nba1, nba2,
            nri1, nri2, nle1, nle2
        ] = get_normals();


        
        let top1    = ([tbr, tbl, tfr], [c0;3], [ntr1;3], [tex_tr, tex_tl, tex_br]);
        let top2    = ([tfl, tbl, tfr], [c0;3], [ntr2;3], [tex_bl, tex_tl, tex_br]);
        let front1  = ([tfl, bfl, tfr], [c1;3], [nfr1;3], [tex_tl, tex_bl, tex_tr]);
        let front2  = ([bfr, bfl, tfr], [c1;3], [nfr2;3], [tex_br, tex_bl, tex_tr]);
        let bottom1 = ([bfr, bfl, bbr], [c2;3], [nbo1;3], [tex_tr, tex_tl, tex_br]);
        let bottom2 = ([bbl, bfl, bbr], [c2;3], [nbo2;3], [tex_bl, tex_tl, tex_br]);
        let back1   = ([bbl, tbl, bbr], [c3;3], [nba1;3], [tex_tl, tex_bl, tex_tr]);
        let back2   = ([tbr, tbl, bbr], [c3;3], [nba2;3], [tex_br, tex_bl, tex_tr]);
        let right1  = ([tbr, tfr, bbr], [c4;3], [nri1;3], [tex_tr, tex_tl, tex_br]);
        let right2  = ([bfr, tfr, bbr], [c4;3], [nri2;3], [tex_bl, tex_tl, tex_br]);
        let left1   = ([tfl, tbl, bfl], [c5;3], [nle1;3], [tex_tr, tex_tl, tex_br]);
        let left2   = ([bbl, tbl, bfl], [c5;3], [nle2;3], [tex_bl, tex_tl, tex_br]);

        let position_array = Vector::<f32, {3*36}>::from_vec(vec![
            top1.0, top2.0, front1.0, front2.0,
            bottom1.0, bottom2.0, back1.0, back2.0,
            right1.0, right2.0, left1.0, left2.0,
        ].concat().concat());

        let colour_array = Vector::<f32, {4*36}>::from_vec(vec![
            top1.1, top2.1, front1.1, front2.1,
            bottom1.1, bottom2.1, back1.1, back2.1,
            right1.1, right2.1, left1.1, left2.1,
        ].concat().concat());

        let normal_array = Vector::<f32, {3*36}>::from_vec(vec![
            top1.2, top2.2, front1.2, front2.2,
            bottom1.2, bottom2.2, back1.2, back2.2,
            right1.2, right2.2, left1.2, left2.2,
        ].concat().concat());

        let texture_array = Vector::<f32, {2*36}>::from_vec(vec![
            top1.3, top2.3, front1.3, front2.3,
            bottom1.3, bottom2.3, back1.3, back2.3,
            right1.3, right2.3, left1.3, left2.3,
        ].concat().concat());


        let position_matrix = Matrix::from_vector(position_array).reshape(S2::<3, 36>).unwrap();
        let colour_matrix   = Matrix::from_vector(colour_array).reshape(S2::<4, 36>).unwrap();
        let normal_matrix   = Matrix::from_vector(normal_array).reshape(S2::<3, 36>).unwrap();
        let texture_matrix  = Matrix::from_vector(texture_array).reshape(S2::<2, 36>).unwrap();

        Cube::from_matrices(position_matrix, colour_matrix, normal_matrix, texture_matrix)
    }
}