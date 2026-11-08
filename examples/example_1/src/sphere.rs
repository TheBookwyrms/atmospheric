use std::f32::consts::PI;

use numeracy::{general_math::ranges::linspace, matrices::{Matrix, S2}, vectors::Vector};

pub struct TripleRingSphere {
    pub position_matrix : Matrix<f32, 2, S2<3, {3*10*60}>>,
    pub colour_matrix   : Matrix<f32, 2, S2<4, {3*10*60}>>,
    pub normal_matrix   : Matrix<f32, 2, S2<3, {3*10*60}>>,
    pub radius:f32,
}
impl TripleRingSphere {
    pub fn new(radius:f32, xyz_rgb:Matrix<f32, 2, S2<3,3>>) -> Self {
        let xyz_rgb = Matrix::from_2darray([[1.,0.,0.],[0.,1.,0.],[0.,0.,1.]]);
        const RINGS_PER_AXIS:usize = 10;
        const POINTS_PER_RING:usize = 60;
        let mut x_circ = Matrix::null(S2::<3,{RINGS_PER_AXIS*POINTS_PER_RING}>);
        let mut y_circ = Matrix::null(S2::<3,{RINGS_PER_AXIS*POINTS_PER_RING}>);
        let mut z_circ = Matrix::null(S2::<3,{RINGS_PER_AXIS*POINTS_PER_RING}>);
        let mut x_circ_col = Matrix::null(S2::<4,{RINGS_PER_AXIS*POINTS_PER_RING}>);
        let mut y_circ_col = Matrix::null(S2::<4,{RINGS_PER_AXIS*POINTS_PER_RING}>);
        let mut z_circ_col = Matrix::null(S2::<4,{RINGS_PER_AXIS*POINTS_PER_RING}>);
        //for (i, height) in linspace(-radius, radius, RINGS_PER_AXIS, true).into_iter().enumerate() {
        for (i, height) in linspace(0., 2.*radius, RINGS_PER_AXIS, true).into_iter().enumerate() {
            for (j,radians) in linspace(0., 2.*PI, POINTS_PER_RING, true).into_iter().enumerate() {

                let r = f32::sqrt(2.*radius*height.abs()-height.powf(2.));

                for ((offset, circ), circ_col) in (0..3).zip([&mut x_circ, &mut y_circ, &mut z_circ]).zip([&mut x_circ_col, &mut y_circ_col, &mut z_circ_col]) {                   
                    
                    // in order x_circ, z_circ, y_circ
                    let (ix, iy, iz) = ((offset)%3, (offset+1)%3, (offset+2)%3);
                    circ[[ix, i*POINTS_PER_RING +j]] = r*f32::cos(radians);
                    circ[[iy, i*POINTS_PER_RING +j]] = r*f32::sin(radians);
                    circ[[iz, i*POINTS_PER_RING +j]] = height-radius;

                    circ_col[[0, i*POINTS_PER_RING +j]] = xyz_rgb.get_col(offset).unwrap().array[0];
                    circ_col[[1, i*POINTS_PER_RING +j]] = xyz_rgb.get_col(offset).unwrap().array[1];
                    circ_col[[2, i*POINTS_PER_RING +j]] = xyz_rgb.get_col(offset).unwrap().array[2];
                    circ_col[[3, i*POINTS_PER_RING +j]] = 1.;
                }
                //x_circ[[0, i*POINTS_PER_RING +j]] = r*f32::cos(radians);
                //x_circ[[1, i*POINTS_PER_RING +j]] = r*f32::sin(radians);
                //x_circ[[2, i*POINTS_PER_RING +j]] = height-radius;
//
                //y_circ[[0, i*POINTS_PER_RING +j]] = height-radius;
                //y_circ[[1, i*POINTS_PER_RING +j]] = r*f32::cos(radians);
                //y_circ[[2, i*POINTS_PER_RING +j]] = r*f32::sin(radians);
//
                //z_circ[[0, i*POINTS_PER_RING +j]] = r*f32::sin(radians);
                //z_circ[[1, i*POINTS_PER_RING +j]] = height-radius;
                //z_circ[[2, i*POINTS_PER_RING +j]] = r*f32::cos(radians);
                //x_circ.extend([radius*f32::cos(radians), radius*f32::sin(radians), height]);
                //y_circ.extend([height, radius*f32::cos(radians), radius*f32::sin(radians)]);
                //z_circ.extend([radius*f32::sin(radians), height, radius*f32::cos(radians)]);
                //x_circ_col.extend(xyz_rgb.get_col(0).unwrap().array);
                //y_circ_col.extend(xyz_rgb.get_col(1).unwrap().array);
                //z_circ_col.extend(xyz_rgb.get_col(2).unwrap().array);
            }
        }

        let position_matrix: Matrix<f32, 2, S2<3,1800>> = x_circ.expand_vertically::<600,1200>(y_circ).expand_vertically(z_circ);
        let colour_matrix: Matrix<f32, 2, S2<4,1800>> = x_circ_col.expand_vertically::<600,1200>(y_circ_col).expand_vertically(z_circ_col);
        let normal_matrix: Matrix<f32, 2, S2<3,1800>> = position_matrix.clone();

        Self { position_matrix, colour_matrix, normal_matrix, radius }












        //const NUM_RINGS:usize = 10;
        //const POINTS_PER_RING:usize = 60;
        //let mut x_circ = vec![];
        //let mut y_circ = vec![];
        //let mut z_circ = vec![];
        //let mut x_circ_col = vec![];
        //let mut y_circ_col = vec![];
        //let mut z_circ_col = vec![];
        //for height in linspace(-radius, radius, NUM_RINGS, true) {
        //    for radians in linspace(0., 2.*PI, 60, true) {
        //        x_circ.extend([radius*f32::cos(radians), radius*f32::sin(radians), height]);
        //        y_circ.extend([height, radius*f32::cos(radians), radius*f32::sin(radians)]);
        //        z_circ.extend([radius*f32::sin(radians), height, radius*f32::cos(radians)]);
        //        x_circ_col.extend(xyz_rgb.get_col(0).unwrap().array);
        //        y_circ_col.extend(xyz_rgb.get_col(1).unwrap().array);
        //        z_circ_col.extend(xyz_rgb.get_col(2).unwrap().array);
        //    }
        //}
        //let x_circ = Matrix::from_vec_with_shape(x_circ, S2::<3,{NUM_RINGS*POINTS_PER_RING}>);
        //let y_circ = Matrix::from_vec_with_shape(y_circ, S2::<3,{NUM_RINGS*POINTS_PER_RING}>);
        //let z_circ = Matrix::from_vec_with_shape(z_circ, S2::<3,{NUM_RINGS*POINTS_PER_RING}>);
        //let x_circ_col = Matrix::from_vec_with_shape(x_circ_col, S2::<3,{NUM_RINGS*POINTS_PER_RING}>);
        //let y_circ_col = Matrix::from_vec_with_shape(y_circ_col, S2::<3,{NUM_RINGS*POINTS_PER_RING}>);
        //let z_circ_col = Matrix::from_vec_with_shape(z_circ_col, S2::<3,{NUM_RINGS*POINTS_PER_RING}>);

    }
}


// 
// //
// //class Sphere:
//     def build_sphere_coords(self, radius):
//         heights = np.linspace(0, 2*radius, num=10)
//         degrees = np.linspace(0, 360, num=60)
//         self.positions = []
//         self.colours = []
//         for i, h in enumerate(heights):
//             h2 = np.abs(h)
//             r = np.sqrt(h2*(2*radius-h2))
//             for ind, d in enumerate(degrees):
//                 # x, y, z, r, g, b, a
//                 x_circ = np.array([
//                     r*np.cos(np.radians(d)),
//                     r*np.sin(np.radians(d)),
//                     h-radius])
//                 y_circ = np.array([
//                     h-radius,
//                     r*np.cos(np.radians(d)),
//                     r*np.sin(np.radians(d))])
//                 z_circ = np.array([
//                     r*np.cos(np.radians(d)),
//                     h-radius,
//                     r*np.sin(np.radians(d))])
// 
//                 self.positions.append(x_circ)
//                 self.positions.append(y_circ)
//                 self.positions.append(z_circ)
//                 self.colours.append([xc[0], xc[1], xc[2]])
//                 self.colours.append([yc[0], yc[1], yc[2]])
//                 self.colours.append([zc[0], zc[1], zc[2]])
// 
//         vertices = np.array(self.positions, dtype=np.float32)
//         self.vertices = vertices
//         colours = np.array(self.colours, dtype=np.float32)
//         data = np.ones((len(vertices), 6), dtype=np.float32)
//         data[:, :3] = vertices
//         data[:, 3:] = colours
//         self.data = data
//    def __init__(self, radius=1, x_i=0, y_i=0, z_i=0, x_c=[1,0,0], y_c=[0,1,0], z_c=[0,0,1]):
//        self.build_sphere_coords(radius, x_i, y_i, z_i, x_c, y_c, z_c)
//
//        self.vbo = glGenBuffers(1)
//        glBindBuffer(GL_ARRAY_BUFFER, self.vbo)
//        glBufferData(GL_ARRAY_BUFFER, self.data.nbytes, self.data, GL_DYNAMIC_DRAW)
//        glBindBuffer(GL_ARRAY_BUFFER, 0)
//
//
//        self.radius = radius
//        self.m = (np.random.randint(3, 8) - np.random.random()) * radius
//        self.prev_s = np.array([x_i, y_i, z_i], dtype=np.float32)
//        self.curr_s = np.array([x_i, y_i, z_i], dtype=np.float32)
//        self.next_s = np.array([x_i, y_i, z_i], dtype=np.float32)
//        self.curr_v = np.array([0, 0, 0], dtype=np.float32)
//        self.next_v = np.array([0, 0, 0], dtype=np.float32)
//        self.curr_a = np.array([0, 0, 0], dtype=np.float32)
//        self.next_a = np.array([0, 0, 0], dtype=np.float32)
//
//        self.trail_s = np.array(([x_i, y_i, z_i, 1, 1, 1],)*256, dtype=np.float32)
//        self.trail_vbo = glGenBuffers(1)
//        glBindBuffer(GL_ARRAY_BUFFER, self.trail_vbo)
//        glBufferData(GL_ARRAY_BUFFER, self.trail_s.nbytes, self.trail_s, GL_DYNAMIC_DRAW)
//        glBindBuffer(GL_ARRAY_BUFFER, 0)
//
//
//    def update_point_and_trail_vbo(self):
//
//        glBindBuffer(GL_ARRAY_BUFFER, self.vbo)
//        glBufferSubData(GL_ARRAY_BUFFER, 0, self.data.nbytes, self.data)
//        glBindBuffer(GL_ARRAY_BUFFER, 0)
//
//        glBindBuffer(GL_ARRAY_BUFFER, self.trail_vbo)
//        glBufferSubData(GL_ARRAY_BUFFER, 0, self.trail_s.nbytes, self.trail_s)
//        glBindBuffer(GL_ARRAY_BUFFER, 0)