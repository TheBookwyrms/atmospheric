use std::f32::consts::PI;

use numeracy::{general_math::ranges::linspace, matrices::{Matrix, S2}, vectors::Vector};

pub struct TripleRingSphere {
    position_matrix : Matrix<f32, 2, S2<3, 36>>,
    colour_matrix   : Matrix<f32, 2, S2<4, 36>>,
    normal_matrix   : Matrix<f32, 2, S2<3, 36>>,
}
impl TripleRingSphere {
    pub fn new(radius:f32, xyz_rgb:Matrix<f32, 2, S2<3,3>>) {//} -> Self {
        const NUM_RINGS:usize = 10;
        let mut x_circ = vec![];
        let mut y_circ = vec![];
        let mut z_circ = vec![];
        let mut x_circ_col = vec![];
        let mut y_circ_col = vec![];
        let mut z_circ_col = vec![];
        for height in linspace(-radius, radius, NUM_RINGS, true) {
            for radians in linspace(0., 2.*PI, 60, true) {
                x_circ.push(Vector::from_1darray([radius*f32::cos(radians), radius*f32::sin(radians), height]));
                y_circ.push(Vector::from_1darray([height, radius*f32::cos(radians), radius*f32::sin(radians)]));
                z_circ.push(Vector::from_1darray([radius*f32::sin(radians), height, radius*f32::cos(radians)]));
                x_circ_col.push(xyz_rgb.get_col(0).unwrap());
                y_circ_col.push(xyz_rgb.get_col(0).unwrap());
                z_circ_col.push(xyz_rgb.get_col(0).unwrap());
            }
        }

        println!("{},{},{},{},{},{}", x_circ.len(),y_circ.len(),z_circ.len(),x_circ_col.len(),y_circ_col.len(),z_circ_col.len())
    }
}



//
//class Sphere:
//    def build_sphere_coords(self, radius, x, y, z, xc, yc, zc):
//        heights = np.linspace(0, 2*radius, num=10)
//        degrees = np.linspace(0, 360, num=60)
//        self.positions = []
//        self.colours = []
//        for i, h in enumerate(heights):
//            h2 = np.abs(h)
//            r = np.sqrt(h2*(2*radius-h2))
//            for ind, d in enumerate(degrees):
//                # x, y, z, r, g, b, a
//                x_circ = np.array([
//                    r*np.cos(np.radians(d))+x,
//                    r*np.sin(np.radians(d))+y,
//                    h-radius+z])
//                y_circ = np.array([
//                    h-radius+x,
//                    r*np.cos(np.radians(d))+y,
//                    r*np.sin(np.radians(d))+z])
//                z_circ = np.array([
//                    r*np.cos(np.radians(d))+x,
//                    h-radius+y,
//                    r*np.sin(np.radians(d))+z])
//
//                self.positions.append(x_circ)
//                self.positions.append(y_circ)
//                self.positions.append(z_circ)
//                self.colours.append([xc[0], xc[1], xc[2]])
//                self.colours.append([yc[0], yc[1], yc[2]])
//                self.colours.append([zc[0], zc[1], zc[2]])
//
//        vertices = np.array(self.positions, dtype=np.float32)
//        self.vertices = vertices
//        colours = np.array(self.colours, dtype=np.float32)
//        data = np.ones((len(vertices), 6), dtype=np.float32)
//        data[:, :3] = vertices
//        data[:, 3:] = colours
//        self.data = data
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