use numeracy::matrices::{Matrix, S2};

use crate::objects::ntree::{Point, PointThing, TreeError};



#[derive(Debug, Clone, PartialEq)]
pub struct Octree<T> {
    pub front_top_left     : OcTreeItem<T>,
    pub front_top_right    : OcTreeItem<T>,
    pub front_bottom_left  : OcTreeItem<T>,
    pub front_bottom_right : OcTreeItem<T>,
    pub back_top_left      : OcTreeItem<T>,
    pub back_top_right     : OcTreeItem<T>,
    pub back_bottom_left   : OcTreeItem<T>,
    pub back_bottom_right  : OcTreeItem<T>,
    pub out_of_bounds_points:Vec<T>,
    pub centre       : PointThing,
    pub half_side_length : f32,
}
#[derive(Debug, Clone, PartialEq)]
pub enum OcTreeItem<T> {
    Node(Box<Octree<T>>),
    Value(Vec<T>),
    None
}
impl<T:Point+Clone> OcTreeItem<T> {
    pub fn get_values(self) -> Vec<T> {
        match self {
            OcTreeItem::None => vec![],
            OcTreeItem::Value(vec) => vec,
            OcTreeItem::Node(node) => node.get_all_values(),
        }
    }
    pub fn get_bounds(self) -> Vec<(PointThing, f32)> {
        match self {
            OcTreeItem::None => vec![],
            OcTreeItem::Value(vec) => vec![],
            OcTreeItem::Node(node) => node.get_all_bounds(),
        }
    }
}

fn _insert_empty_tree<T:Clone+Point>(points:Vec<T>, max_per_node:usize, centre:PointThing, side_len:f32, relative_position:RelativePointPos) -> Result<OcTreeItem<T>, TreeError> {
    if points.len() >= max_per_node {
        Ok(
            Octree::new(centre, side_len)
                   .subdivide_node(relative_position, max_per_node)?
                   .insert(points, max_per_node)?
                   .get_by(relative_position)?
        )
    }
    else {
        Ok(OcTreeItem::Value(points))
    }
}
fn _subdivide_node_internal_match<T:Clone+Point>(
    current_node:OcTreeItem<T>, centre:PointThing, half_side_len:f32, max_per_node:usize
) -> Result<OcTreeItem<T>, TreeError> {

    let mut new_octree = Octree::new(centre, half_side_len);
    let new_new_octree = match current_node {
        OcTreeItem::None => new_octree,
        OcTreeItem::Node(node) => {Err(TreeError::NodeAlreadySubdivided)?; new_octree},
        OcTreeItem::Value(points) => { new_octree.insert(points, max_per_node)? },
    };
    let mut new_current_node = OcTreeItem::Node(Box::new(new_new_octree));
    Ok(new_current_node)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RelativePointPos {
    OutOfBounds,

    FrontTopLeft,
    FrontTopRight,
    FrontBottomLeft,
    FrontBottomRight,

    BackTopLeft,
    BackTopRight,
    BackBottomLeft,
    BackBottomRight,
}
impl RelativePointPos {
    const POSITIONS : [RelativePointPos;9] = [     
    Self::OutOfBounds,
    Self::FrontTopLeft,
    Self::FrontTopRight,
    Self::FrontBottomLeft,
    Self::FrontBottomRight,
    Self::BackTopLeft,
    Self::BackTopRight,
    Self::BackBottomLeft,
    Self::BackBottomRight,
    ];
}

impl<T:Clone+Point> Octree<T> {
    pub fn get_by(self, relative_position:RelativePointPos) -> Result<OcTreeItem<T>, TreeError> {
        match relative_position {
            RelativePointPos::OutOfBounds => Err(TreeError::PointOutOfBounds),
            RelativePointPos::FrontTopLeft     => Ok(self.front_top_left),
            RelativePointPos::FrontTopRight    => Ok(self.front_top_right),
            RelativePointPos::FrontBottomLeft  => Ok(self.front_bottom_left),
            RelativePointPos::FrontBottomRight => Ok(self.front_bottom_right),
            RelativePointPos::BackTopLeft     => Ok(self.back_top_left),
            RelativePointPos::BackTopRight    => Ok(self.back_top_right),
            RelativePointPos::BackBottomLeft  => Ok(self.back_bottom_left),
            RelativePointPos::BackBottomRight => Ok(self.back_bottom_right),
        }
    }
    pub fn new_on_origin(side_len:f32) -> Self {
        Self {
            front_top_left: OcTreeItem::None,
            front_top_right: OcTreeItem::None,
            front_bottom_left: OcTreeItem::None,
            front_bottom_right: OcTreeItem::None,
            back_top_left: OcTreeItem::None,
            back_top_right: OcTreeItem::None,
            back_bottom_left: OcTreeItem::None,
            back_bottom_right: OcTreeItem::None,
            out_of_bounds_points:vec![],
            centre: PointThing::new(0., 0., 0.),
            half_side_length:side_len/2.,
        }
    }
    pub fn new(centre:PointThing, side_len:f32) -> Self {
        Self {
            front_top_left: OcTreeItem::None,
            front_top_right: OcTreeItem::None,
            front_bottom_left: OcTreeItem::None,
            front_bottom_right: OcTreeItem::None,
            back_top_left: OcTreeItem::None,
            back_top_right: OcTreeItem::None,
            back_bottom_left: OcTreeItem::None,
            back_bottom_right: OcTreeItem::None,
            out_of_bounds_points:vec![],
            centre: centre,
            half_side_length:side_len/2.,
        }
    }
    pub fn get_relative_point_pos(&self, point:&T) -> RelativePointPos {
        let (px, py, pz) = (point.get_x(), point.get_y(), point.get_z());
        let half = self.half_side_length;
        let (xm, ym, zm) = (self.centre.get_x(), self.centre.get_y(), self.centre.get_z());
        //let [[xl, xh], [yl, yh], [zl, zh]] = self.bounds.get_bounds_limits();
        //let middle = self.bounds.middle();
        //let (xm, ym, zm) = (middle.get_x(), middle.get_y(), middle.get_z());

        let (left, right) = (xm >= px, xm <= px);
        let (bottom, top) = (ym >= py, ym <= py);
        let (back, front) = (zm >= pz, zm <= pz);
        let (out_left, out_right) = (px-half >= px, px+half <= px);
        let (out_bottom, out_top) = (py-half >= py, py+half <= py);
        let (out_back, out_front) = (pz-half >= pz, pz+half <= pz);

        let top_left = top && left;
        let top_right = top && right;
        let bottom_left = bottom && left;
        let bottom_right = bottom && right;

        if out_bottom || out_left || out_right || out_top || out_back || out_front {
            RelativePointPos::OutOfBounds
        } else if front {
            if right {
                if top {
                    RelativePointPos::FrontTopRight
                } else { // bottom
                    RelativePointPos::FrontBottomRight
                }
            } else { // left
                if top {
                    RelativePointPos::FrontTopLeft
                } else { // bottom
                    RelativePointPos::FrontBottomLeft
                }
            }
        } else { // back
            if right {
                if top {
                    RelativePointPos::BackTopRight
                } else { // bottom
                    RelativePointPos::BackBottomRight
                }
            } else { // left
                if top {
                    RelativePointPos::BackTopLeft
                } else { // bottom
                    RelativePointPos::BackBottomLeft
                }
            }
        }
    }
    pub fn insert(mut self, points:Vec<T>, max_per_node:usize) -> Result<Self, TreeError> {
        let mut front_top_left_points = vec![];
        let mut front_top_right_points = vec![];
        let mut front_bottom_left_points = vec![];
        let mut front_bottom_right_points = vec![];
        
        let mut back_top_left_points = vec![];
        let mut back_top_right_points = vec![];
        let mut back_bottom_left_points = vec![];
        let mut back_bottom_right_points = vec![];
        
        let mut out_of_bounds_points = vec![];
        for point in points {
            match self.get_relative_point_pos(&point) {
                RelativePointPos::OutOfBounds => out_of_bounds_points.push(point),
                RelativePointPos::FrontTopLeft => front_top_left_points.push(point),
                RelativePointPos::FrontTopRight => front_top_right_points.push(point),
                RelativePointPos::FrontBottomLeft => front_bottom_left_points.push(point),
                RelativePointPos::FrontBottomRight => front_bottom_right_points.push(point),
                RelativePointPos::BackTopLeft => back_top_left_points.push(point),
                RelativePointPos::BackTopRight => back_top_right_points.push(point),
                RelativePointPos::BackBottomLeft => back_bottom_left_points.push(point),
                RelativePointPos::BackBottomRight => back_bottom_right_points.push(point),
            }
        }
        let corners_initial = [
            (self.front_top_left, front_top_left_points, RelativePointPos::FrontTopLeft),
            (self.front_top_right, front_top_right_points, RelativePointPos::FrontTopRight),
            (self.front_bottom_left, front_bottom_left_points, RelativePointPos::FrontBottomLeft),
            (self.front_bottom_right, front_bottom_right_points, RelativePointPos::FrontBottomRight),
            (self.back_top_left, back_top_left_points, RelativePointPos::BackTopLeft),
            (self.back_top_right, back_top_right_points, RelativePointPos::BackTopRight),
            (self.back_bottom_left, back_bottom_left_points, RelativePointPos::BackBottomLeft),
            (self.back_bottom_right, back_bottom_right_points, RelativePointPos::BackBottomRight),
        ];

        let mut resulting_octrees = [
            OcTreeItem::None, OcTreeItem::None, OcTreeItem::None, OcTreeItem::None,
            OcTreeItem::None, OcTreeItem::None, OcTreeItem::None, OcTreeItem::None,
        ];
        let (centre, side_len) = (self.centre, 2.*self.half_side_length);
        for (idx, (mut corner, points, relative_position)) in corners_initial.into_iter().enumerate() {
            let oc_tree_i = match corner {
                OcTreeItem::None => {
                    _insert_empty_tree(points, max_per_node, centre, side_len, relative_position)?
                },
                OcTreeItem::Value(mut vec) => {
                    vec.extend(points);
                    _insert_empty_tree(vec, max_per_node, centre, side_len, relative_position)?
                },
                OcTreeItem::Node( mut node) => {
                    let new_node = node.insert(points, max_per_node)?;
                    OcTreeItem::Node(Box::new(new_node))
                },
            };
            resulting_octrees[idx] = oc_tree_i;
        }

        let [
            resulting_octree_0, resulting_octree_1,
            resulting_octree_2, resulting_octree_3,
            resulting_octree_4, resulting_octree_5,
            resulting_octree_6, resulting_octree_7,
        ] = resulting_octrees;

        Ok(
            Octree {
                front_top_left       : resulting_octree_0,
                front_top_right      : resulting_octree_1,
                front_bottom_left    : resulting_octree_2,
                front_bottom_right   : resulting_octree_3,
                back_top_left        : resulting_octree_4,
                back_top_right       : resulting_octree_5,
                back_bottom_left     : resulting_octree_6,
                back_bottom_right    : resulting_octree_7,
                out_of_bounds_points : out_of_bounds_points,
                centre               : self.centre,
                half_side_length     : self.half_side_length,
            }
        )
    }
    pub fn subdivide_node(mut self, octant:RelativePointPos, max_per_node:usize) -> Result<Self, TreeError> {
        let (px, py, pz) = self.centre.get_xyz();
        let half = self.half_side_length;
        let quart = half/2.;
        match octant {
            RelativePointPos::OutOfBounds => Err(TreeError::PointOutOfBounds)?,
            RelativePointPos::FrontTopLeft => {
                let new_centre = PointThing::new(px-quart, py+quart, pz+quart);
                self.front_top_left = _subdivide_node_internal_match(self.front_top_left, new_centre, half, max_per_node)?;
            },
            RelativePointPos::FrontTopRight => {
                let new_centre = PointThing::new(px+quart, py+quart, pz+quart);
                self.front_top_right = _subdivide_node_internal_match(self.front_top_right, new_centre, half, max_per_node)?;
            },
            RelativePointPos::FrontBottomLeft => {
                let new_centre = PointThing::new(px-quart, py-quart, pz+quart);
                self.front_bottom_left = _subdivide_node_internal_match(self.front_bottom_left, new_centre, half, max_per_node)?;
            },
            RelativePointPos::FrontBottomRight => {
                let new_centre = PointThing::new(px+quart, py-quart, pz+quart);
                self.front_bottom_right = _subdivide_node_internal_match(self.front_bottom_right, new_centre, half, max_per_node)?;
            },
            RelativePointPos::BackTopLeft => {
                let new_centre = PointThing::new(px-quart, py+quart, pz-quart);
                self.back_top_left = _subdivide_node_internal_match(self.back_top_left, new_centre, half, max_per_node)?;
            },
            RelativePointPos::BackTopRight => {
                let new_centre = PointThing::new(px+quart, py+quart, pz-quart);
                self.back_top_right = _subdivide_node_internal_match(self.back_top_right, new_centre, half, max_per_node)?;
            },
            RelativePointPos::BackBottomLeft => {
                let new_centre = PointThing::new(px-quart, py-quart, pz-quart);
                self.back_bottom_left = _subdivide_node_internal_match(self.back_bottom_left, new_centre, half, max_per_node)?;
            },
            RelativePointPos::BackBottomRight => {
                let new_centre = PointThing::new(px+quart, py-quart, pz-quart);
                self.back_bottom_right = _subdivide_node_internal_match(self.back_bottom_right, new_centre, half, max_per_node)?;
            },
        }
        Ok(self)
    }
    pub fn get_all_values(&self) -> Vec<T> {
        let mut all_values = vec![];
        for position in RelativePointPos::POSITIONS {
            match position {
                RelativePointPos::OutOfBounds      => all_values.extend(self.out_of_bounds_points.clone()),
                RelativePointPos::FrontTopLeft     => all_values.extend(self.front_top_left.clone().get_values()),
                RelativePointPos::FrontTopRight    => all_values.extend(self.front_top_right.clone().get_values()),
                RelativePointPos::FrontBottomLeft  => all_values.extend(self.front_bottom_left.clone().get_values()),
                RelativePointPos::FrontBottomRight => all_values.extend(self.front_bottom_right.clone().get_values()),
                RelativePointPos::BackTopLeft      => all_values.extend(self.back_top_left.clone().get_values()),
                RelativePointPos::BackTopRight     => all_values.extend(self.back_top_right.clone().get_values()),
                RelativePointPos::BackBottomLeft   => all_values.extend(self.back_bottom_left.clone().get_values()),
                RelativePointPos::BackBottomRight  => all_values.extend(self.back_bottom_right.clone().get_values()),
            }
        }
        all_values
    }
    pub fn get_all_bounds(&self) -> Vec<(PointThing, f32)> {
        let mut all_bounds = vec![];
        all_bounds.push((self.centre, self.half_side_length));
        for position in RelativePointPos::POSITIONS {
            match position {
                RelativePointPos::OutOfBounds => {},
                RelativePointPos::FrontTopLeft     => all_bounds.extend(self.front_top_left.clone().get_bounds()),
                RelativePointPos::FrontTopRight    => all_bounds.extend(self.front_top_right.clone().get_bounds()),
                RelativePointPos::FrontBottomLeft  => all_bounds.extend(self.front_bottom_left.clone().get_bounds()),
                RelativePointPos::FrontBottomRight => all_bounds.extend(self.front_bottom_right.clone().get_bounds()),
                RelativePointPos::BackTopLeft      => all_bounds.extend(self.back_top_left.clone().get_bounds()),
                RelativePointPos::BackTopRight     => all_bounds.extend(self.back_top_right.clone().get_bounds()),
                RelativePointPos::BackBottomLeft   => all_bounds.extend(self.back_bottom_left.clone().get_bounds()),
                RelativePointPos::BackBottomRight  => all_bounds.extend(self.back_bottom_right.clone().get_bounds()),
            }
        }
        all_bounds
    }
    /// NUM_POINTS * (2 point/line) * (12 line/box) for reasonable upper bound (likely won't exceed this (but theoretically possible))
    pub fn get_all_lines<const LINES_UPPER_BOUND:usize>(&self) -> Matrix<f32, 2, S2<7, LINES_UPPER_BOUND>> {
        let all_bounds = self.get_all_bounds();
        let mut all_lines_data = vec![];
        for (centre, half) in all_bounds {
            let (px, py, pz) = centre.get_xyz();
            let ((xl, xh), (yl, yh), (zl, zh)) = ((px - half, px + half), (py - half, py + half), (pz - half, pz + half));
            all_lines_data.extend(vec![
                xl, yl, zh, 1., 1., 1., 1., // front bottom line
                xh, yl, zh, 1., 1., 1., 1., // front bottom line

                xl, yh, zh, 1., 1., 1., 1., // front top line
                xh, yh, zh, 1., 1., 1., 1., // front top line

                xl, yl, zh, 1., 1., 1., 1., // front left line
                xl, yh, zh, 1., 1., 1., 1., // front left line

                xh, yl, zh, 1., 1., 1., 1., // front right line
                xh, yh, zh, 1., 1., 1., 1., // front right line

                xl, yl, zl, 1., 1., 1., 1., // back bottom line
                xh, yl, zl, 1., 1., 1., 1., // back bottom line

                xl, yh, zl, 1., 1., 1., 1., // back top line
                xh, yh, zl, 1., 1., 1., 1., // back top line

                xl, yl, zl, 1., 1., 1., 1., // back left line
                xl, yh, zl, 1., 1., 1., 1., // back left line

                xh, yl, zl, 1., 1., 1., 1., // back right line
                xh, yh, zl, 1., 1., 1., 1., // back right line

                xl, yh, zl, 1., 1., 1., 1., // left top
                xl, yh, zh, 1., 1., 1., 1., // left top

                xl, yl, zl, 1., 1., 1., 1., // left bottom
                xl, yl, zh, 1., 1., 1., 1., // left bottom

                xh, yh, zl, 1., 1., 1., 1., // right top
                xh, yh, zh, 1., 1., 1., 1., // right top

                xh, yl, zl, 1., 1., 1., 1., // right bottom
                xh, yl, zh, 1., 1., 1., 1., // right bottom
            ]);
        }
        Matrix::from_vec_with_shape(all_lines_data, S2::<7, LINES_UPPER_BOUND>)
    }
}
impl Octree<PointThing> {
    pub fn get_all_points<const NUM_POINTS:usize>(&self) -> Matrix<f32, 2, S2<7, NUM_POINTS>> {
        let all_points = self.get_all_values();
        let mut all_points_data = vec![];
        for point in all_points {
            all_points_data.extend(point.to_vec());
        }
        Matrix::from_vec_with_shape(all_points_data, S2::<7, NUM_POINTS>)
    }
}