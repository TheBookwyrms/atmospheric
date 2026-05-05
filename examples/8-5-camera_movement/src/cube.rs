use numeracy::matrices::Matrix;

fn arr7d(pos:[f32; 3], col:[f32; 3], a:f32) -> [f32; 7] {
    [pos[0], pos[1], pos[2], col[0], col[1], col[2], a]
}

fn arr3_of_arr7_from_arr3_of_arr3(positions:[[f32;3];3], col:[f32;3], a:f32) -> [[f32;7];3] {
    [
        arr7d(positions[0], col, a),
        arr7d(positions[1], col, a),
        arr7d(positions[2], col, a),
    ]
}

//pub fn create_cube_vertices(centre:(f32, f32, f32), side_len:f32, v:bool) -> [[f32; 7]; 8] {
pub fn create_cube_vertices(centre:(f32, f32, f32), side_len:f32, v:bool) -> [[f32; 3]; 8] {
    let (x, y, z) = centre;

    let top_front_right    = [x+side_len/2.0, y+side_len/2.0, z+side_len/2.0];
    let top_back_right     = [x+side_len/2.0, y+side_len/2.0, z-side_len/2.0];
    let top_front_left     = [x-side_len/2.0, y+side_len/2.0, z+side_len/2.0];
    let top_back_left      = [x-side_len/2.0, y+side_len/2.0, z-side_len/2.0];
    let bottom_front_right = [x+side_len/2.0, y-side_len/2.0, z+side_len/2.0];
    let bottom_back_right  = [x+side_len/2.0, y-side_len/2.0, z-side_len/2.0];
    let bottom_front_left  = [x-side_len/2.0, y-side_len/2.0, z+side_len/2.0];
    let bottom_back_left   = [x-side_len/2.0, y-side_len/2.0, z-side_len/2.0];

    let c0 = [1.0, 0.0, 0.0];
    let c1 = [0.0, 1.0, 0.0];
    let c2 = [0.0, 1.0, 1.0];
    let c3 = [1.0, 0.0, 0.0];
    let c4 = [1.0, 1.0, 1.0];
    let c5 = [0.0, 0.0, 1.0];
    //let [c0, c1, c2, c3] = if v {
    //    [[1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0]]
    //} else {
    //    [[0.2, 0.4, 0.6], [0.4, 0.6, 0.8], [0.6, 0.8, 0.2], [0.8, 0.2, 0.4]]
    //    //[[0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0, 0.0]]
    //};

    let a = 1.0;

    let cb = [0.0; 3];

    //let tbr = arr7d(top_back_right,     c0, a);
    //let tfr = arr7d(top_front_right,    c1, a);
    //let tfl = arr7d(top_front_left,     c2, a);
    //let tbl = arr7d(top_back_left,      c3, a);
    //let bbr = arr7d(bottom_back_right,  c2, a);
    //let bfr = arr7d(bottom_front_right, c3, a);
    //let bfl = arr7d(bottom_front_left,  c0, a);
    //let bbl = arr7d(bottom_back_left,   c1, a);
    let tbr = top_back_right;
    let tfr = top_front_right;
    let tfl = top_front_left;
    let tbl = top_back_left;
    let bbr = bottom_back_right;
    let bfr = bottom_front_right;
    let bfl = bottom_front_left;
    let bbl = bottom_back_left;

    [tbr, tfr, tfl, tbl, bbr, bfr, bfl, bbl]
}


pub fn ebo_cube(centre:(f32, f32, f32), side_len:f32) -> (Matrix<f32>, Matrix<i32>) {

    let vertices_matrix = Matrix {
        shape:vec![7, 8],
        array:create_cube_vertices(centre, side_len, true).concat(),
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
    };

    (vertices_matrix, indices_matrix)

}

 

 

pub fn cube(centre:(f32, f32, f32), side_len:f32) -> Matrix<f32> {
    
    let [
        tbr, tfr, tfl, tbl, bbr, bfr, bfl, bbl
    ] = create_cube_vertices(centre, side_len, false);

    
    let c0 = [1.0, 0.0, 0.0];
    let c1 = [0.0, 1.0, 0.0];
    let c2 = [0.0, 0.0, 1.0];
    let c3 = [1.0, 1.0, 0.0];
    let c4 = [1.0, 0.0, 1.0];
    let c5 = [0.0, 1.0, 1.0];

    let top1     = arr3_of_arr7_from_arr3_of_arr3([tbr, tbl, tfr], c0, 1.0);
    let top2     = arr3_of_arr7_from_arr3_of_arr3([tfl, tbl, tfr], c0, 1.0);
    let front1   = arr3_of_arr7_from_arr3_of_arr3([tfl, bfl, tfr], c1, 1.0);
    let front2   = arr3_of_arr7_from_arr3_of_arr3([bfr, bfl, tfr], c1, 1.0);
    let bottom1  = arr3_of_arr7_from_arr3_of_arr3([bfr, bfl, bbr], c2, 1.0);
    let bottom2  = arr3_of_arr7_from_arr3_of_arr3([bbl, bfl, bbr], c2, 1.0);
    let back1    = arr3_of_arr7_from_arr3_of_arr3([bbl, tbl, bbr], c3, 1.0);
    let back2    = arr3_of_arr7_from_arr3_of_arr3([tbr, tbl, bbr], c3, 1.0);
    let right1   = arr3_of_arr7_from_arr3_of_arr3([tbr, tfr, bbr], c4, 1.0);
    let right2   = arr3_of_arr7_from_arr3_of_arr3([bfr, tfr, bbr], c4, 1.0);
    let left1    = arr3_of_arr7_from_arr3_of_arr3([tfl, tbl, bfl], c5, 1.0);
    let left2    = arr3_of_arr7_from_arr3_of_arr3([bbl, tbl, bfl], c5, 1.0);


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


    Matrix { shape: vec![7, cube.len()/7], array: cube }
}
