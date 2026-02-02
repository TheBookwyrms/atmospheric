use numeracy::{enums::MatrixDataTypes, matrices::Matrix};

fn arr7d(pos:[f32; 3], col:[f32; 3], a:f32) -> [f32; 7] {
    [pos[0], pos[1], pos[2], col[0], col[1], col[2], a]
}

pub fn create_cube_vertices(centre:(f32, f32, f32), side_len:f32, v:bool) -> [[f32; 7]; 8] {
    let (x, y, z) = centre;

    let top_front_right    = [x+side_len/2.0, y+side_len/2.0, z+side_len/2.0];
    let top_back_right     = [x+side_len/2.0, y+side_len/2.0, z-side_len/2.0];
    let top_front_left     = [x-side_len/2.0, y+side_len/2.0, z+side_len/2.0];
    let top_back_left      = [x-side_len/2.0, y+side_len/2.0, z-side_len/2.0];
    let bottom_front_right = [x+side_len/2.0, y-side_len/2.0, z+side_len/2.0];
    let bottom_back_right  = [x+side_len/2.0, y-side_len/2.0, z-side_len/2.0];
    let bottom_front_left  = [x-side_len/2.0, y-side_len/2.0, z+side_len/2.0];
    let bottom_back_left   = [x-side_len/2.0, y-side_len/2.0, z-side_len/2.0];

    let c0 = [0.2, 0.4, 0.6];
    let c1 = [0.4, 0.6, 0.8];
    let c2 = [0.6, 0.8, 0.2];
    let c3 = [0.8, 0.2, 0.4];
    //let [c0, c1, c2, c3] = if v {
    //    [[1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0]]
    //} else {
    //    [[0.2, 0.4, 0.6], [0.4, 0.6, 0.8], [0.6, 0.8, 0.2], [0.8, 0.2, 0.4]]
    //    //[[0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0, 0.0]]
    //};

    let a = 1.0;

    let cb = [0.0; 3];

    let tbr = arr7d(top_back_right,     c0, a);
    let tfr = arr7d(top_front_right,    c1, a);
    let tfl = arr7d(top_front_left,     c2, a);
    let tbl = arr7d(top_back_left,      c3, a);
    let bbr = arr7d(bottom_back_right,  c2, a);
    let bfr = arr7d(bottom_front_right, c3, a);
    let bfl = arr7d(bottom_front_left,  c0, a);
    let bbl = arr7d(bottom_back_left,   c1, a);

    [tbr, tfr, tfl, tbl, bbr, bfr, bfl, bbl]
}


pub fn ebo_cube(centre:(f32, f32, f32), side_len:f32) -> (Matrix<f32>, Matrix<i32>) {

    let vertices_matrix = Matrix {
        shape:vec![7, 8],
        array:create_cube_vertices(centre, side_len, true).concat(),
        dtype:MatrixDataTypes::F32
    };

    let indices_arr = [
        0, 3, 1, // top 1
        2, 3, 1, // top 2
        2, 6, 1, // front 1
        5, 6, 1, // front 2
        5, 6, 4, // bottom 1
        7, 6, 4, // bottom 2
        7, 3, 4, // back 1
        0, 3, 4, // back 2
        0, 1, 4, // right 1
        5, 1, 4, // right 2
        2, 3, 6, // left 1
        7, 3, 6, // left 2
    ];

    let indices_matrix = Matrix {
        shape:vec![3, 12],
        array:indices_arr.to_vec(),
        dtype:MatrixDataTypes::I32
    };

    (vertices_matrix, indices_matrix)

}

 

 

pub fn cube(centre:(f32, f32, f32), side_len:f32) -> Matrix<f32> {
    
    let [
        tbr, tfr, tfl, tbl, bbr, bfr, bfl, bbl
    ] = create_cube_vertices(centre, side_len, false);


    let top1     = [tbr, tbl, tfr];
    let top2     = [tfl, tbl, tfr];

    let front1   = [tfl, bfl, tfr];
    let front2   = [bfr, bfl, tfr];

    let bottom1  = [bfr, bfl, bbr];
    let bottom2  = [bbl, bfl, bbr];

    let back1    = [bbl, tbl, bbr];
    let back2    = [tbr, tbl, bbr];

    let right1   = [tbr, tfr, bbr];
    let right2   = [bfr, tfr, bbr];
    
    let left1    = [tfl, tbl, bfl];
    let left2    = [bbl, tbl, bfl];


    let mut cube = vec![];

    // y-axis squares
    cube.extend(top1.concat());
    cube.extend(top2.concat());
    cube.extend(bottom1.concat());
    cube.extend(bottom2.concat());

    // z-axis squares
    cube.extend(front1.concat());
    cube.extend(front2.concat());
    cube.extend(back1.concat());
    cube.extend(back2.concat());

    // x-axis squares
    cube.extend(right1.concat());
    cube.extend(right2.concat());
    cube.extend(left1.concat());
    cube.extend(left2.concat());


    Matrix { shape: vec![7, cube.len()/7], array: cube, dtype: numeracy::enums::MatrixDataTypes::F32 }
}
