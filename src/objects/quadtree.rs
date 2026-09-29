use crate::objects::ntree::{Point, PointThing, TreeError};
use numeracy::matrices::{Matrix, S2};

fn _subdivide_node_internal_match<T:Clone+Point>(
    current_node:QuadTreeItem<T>, centre:PointThing, half_side_len:f32, max_per_node:usize
) -> Result<QuadTreeItem<T>, TreeError> {

    let mut new_quadtree = QuadTree::new(centre, half_side_len);
    let new_new_quadtree = match current_node {
        QuadTreeItem::None => new_quadtree,
        QuadTreeItem::Node(node) => {Err(TreeError::NodeAlreadySubdivided)?; new_quadtree},
        QuadTreeItem::Value(points) => { new_quadtree.insert(points, max_per_node)? },
    };
    let mut new_current_node = QuadTreeItem::Node(Box::new(new_new_quadtree));
    Ok(new_current_node)
}

fn _insert_empty_tree<T:Clone+Point>(points:Vec<T>, max_per_node:usize, centre:PointThing, side_len:f32, relative_position:RelativePointPos) -> Result<QuadTreeItem<T>, TreeError> {
    if points.len() >= max_per_node {
        Ok(
            QuadTree::new(centre, side_len)
                   .subdivide_node(relative_position, max_per_node)?
                   .insert(points, max_per_node)?
                   .get_by(relative_position)?
        )
    }
    else {
        Ok(QuadTreeItem::Value(points))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuadTree<T> {
    pub top_left     : QuadTreeItem<T>,
    pub top_right    : QuadTreeItem<T>,
    pub bottom_left  : QuadTreeItem<T>,
    pub bottom_right : QuadTreeItem<T>,
    pub out_of_bounds_points:Vec<T>,
    pub centre       : PointThing,
    pub half_side_length : f32,
}
#[derive(Debug, Clone, PartialEq)]
pub enum QuadTreeItem<T> {
    Node(Box<QuadTree<T>>),
    Value(Vec<T>),
    None
}
impl<T:Point+Clone> QuadTreeItem<T> {
    pub fn get_values(self) -> Vec<T> {
        match self {
            QuadTreeItem::None => vec![],
            QuadTreeItem::Value(vec) => vec,
            QuadTreeItem::Node(node) => node.get_all_values(),
        }
    }
    pub fn get_bounds(self) -> Vec<(PointThing, f32)> {
        match self {
            QuadTreeItem::None => vec![],
            QuadTreeItem::Value(vec) => vec![],
            QuadTreeItem::Node(node) => node.get_all_bounds(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RelativePointPos {
    OutOfBounds,

    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}
impl RelativePointPos {
    const POSITIONS : [RelativePointPos;5] = [     
    Self::OutOfBounds,
    Self::TopLeft,
    Self::TopRight,
    Self::BottomLeft,
    Self::BottomRight,
    ];
}

impl<T:Clone+Point> QuadTree<T> {
    pub fn get_by(self, relative_position:RelativePointPos) -> Result<QuadTreeItem<T>, TreeError> {
        match relative_position {
            RelativePointPos::OutOfBounds => Err(TreeError::PointOutOfBounds),
            RelativePointPos::TopLeft     => Ok(self.top_left),
            RelativePointPos::TopRight    => Ok(self.top_right),
            RelativePointPos::BottomLeft  => Ok(self.bottom_left),
            RelativePointPos::BottomRight => Ok(self.bottom_right),
        }
    }
    pub fn new_on_origin(side_len:f32) -> Self {
        Self {
            top_left: QuadTreeItem::None,
            top_right: QuadTreeItem::None,
            bottom_left: QuadTreeItem::None,
            bottom_right: QuadTreeItem::None,
            out_of_bounds_points:vec![],
            centre: PointThing::new(0., 0., 0.),
            half_side_length:side_len/2.,
        }
    }
    pub fn new(centre:PointThing, side_len:f32) -> Self {
        Self {
            top_left: QuadTreeItem::None,
            top_right: QuadTreeItem::None,
            bottom_left: QuadTreeItem::None,
            bottom_right: QuadTreeItem::None,
            out_of_bounds_points:vec![],
            centre: centre,
            half_side_length:side_len/2.,
        }
    }
    pub fn get_relative_point_pos(&self, point:&T) -> RelativePointPos {
        let (px, py) = (point.get_x(), point.get_y());
        let half = self.half_side_length;
        let (xm, ym) = (self.centre.get_x(), self.centre.get_y());

        let (left, right) = (xm >= px, xm <= px);
        let (bottom, top) = (ym >= py, ym <= py);
        let (out_left, out_right) = (xm-half > px, xm+half < px);
        let (out_bottom, out_top) = (ym-half > py, ym+half < py);

        if out_bottom || out_left || out_right || out_top {
            RelativePointPos::OutOfBounds
        } else if top && bottom && left && right { // centre
            RelativePointPos::TopLeft
        } else if top && right {
            RelativePointPos::TopRight
        } else if bottom && left {
            RelativePointPos::BottomLeft
        } else if bottom && right {
            RelativePointPos::BottomRight
        } else { // top left
            RelativePointPos::TopLeft
        }
    }
    pub fn insert(mut self, points:Vec<T>, max_per_node:usize) -> Result<Self, TreeError> {
        let mut top_left_points = vec![];
        let mut top_right_points = vec![];
        let mut bottom_left_points = vec![];
        let mut bottom_right_points = vec![];
        let mut out_of_bounds_points = vec![];
        for point in points {
            match self.get_relative_point_pos(&point) {
                RelativePointPos::OutOfBounds => out_of_bounds_points.push(point),
                RelativePointPos::TopLeft => top_left_points.push(point),
                RelativePointPos::TopRight => top_right_points.push(point),
                RelativePointPos::BottomLeft => bottom_left_points.push(point),
                RelativePointPos::BottomRight => bottom_right_points.push(point),
            }
        }
        let corners_initial = [
            (self.top_left, top_left_points, RelativePointPos::TopLeft),
            (self.top_right, top_right_points, RelativePointPos::TopRight),
            (self.bottom_left, bottom_left_points, RelativePointPos::BottomLeft),
            (self.bottom_right, bottom_right_points, RelativePointPos::BottomRight),
        ];

        let mut resulting_quadtrees = [QuadTreeItem::None, QuadTreeItem::None, QuadTreeItem::None, QuadTreeItem::None];
        let (centre, side_len) = (self.centre, 2.*self.half_side_length);
        for (idx, (mut corner, points, relative_position)) in corners_initial.into_iter().enumerate() {
            let quad_tree_i = match corner {
                QuadTreeItem::None => {
                    _insert_empty_tree(points, max_per_node, centre, side_len, relative_position)?
                },
                QuadTreeItem::Value(mut vec) => {
                    vec.extend(points);
                    _insert_empty_tree(vec, max_per_node, centre, side_len, relative_position)?
                },
                QuadTreeItem::Node( mut node) => {
                    let new_node = node.insert(points, max_per_node)?;
                    QuadTreeItem::Node(Box::new(new_node))
                },
            };
            resulting_quadtrees[idx] = quad_tree_i;
        }

        let [
            resulting_quadtree_0, resulting_quadtree_1,
            resulting_quadtree_2, resulting_quadtree_3,
        ] = resulting_quadtrees;

        Ok(
            QuadTree {
                top_left             : resulting_quadtree_0,
                top_right            : resulting_quadtree_1,
                bottom_left          : resulting_quadtree_2,
                bottom_right         : resulting_quadtree_3,
                out_of_bounds_points : out_of_bounds_points,
                centre               : self.centre,
                half_side_length     : self.half_side_length,
            }
        )
    }
    pub fn subdivide_node(mut self, quarter:RelativePointPos, max_per_node:usize) -> Result<Self, TreeError> {
        let (px, py, pz) = self.centre.get_xyz();
        let half = self.half_side_length;
        let quart = half/2.;
        match quarter {
            RelativePointPos::OutOfBounds => Err(TreeError::PointOutOfBounds)?,
            RelativePointPos::TopLeft => {
                let new_centre = PointThing::new(px - quart, py + quart, pz);
                self.top_left = _subdivide_node_internal_match(self.top_left, new_centre, half, max_per_node)?;
            },
            RelativePointPos::TopRight => {
                let new_centre = PointThing::new(px + quart, py + quart, pz);
                self.top_right = _subdivide_node_internal_match(self.top_right, new_centre, half, max_per_node)?;
            },
            RelativePointPos::BottomLeft => {
                let new_centre = PointThing::new(px - quart, py - quart, pz);
                self.bottom_left = _subdivide_node_internal_match(self.bottom_left, new_centre, half, max_per_node)?;
            },
            RelativePointPos::BottomRight => {
                let new_centre = PointThing::new(px + quart, py - quart, pz);
                self.bottom_right = _subdivide_node_internal_match(self.bottom_right, new_centre, half, max_per_node)?;
            },
        }
        Ok(self)
    }
    pub fn get_all_values(&self) -> Vec<T> {
        let mut all_values = vec![];
        for position in RelativePointPos::POSITIONS {
            match position {
                RelativePointPos::OutOfBounds => all_values.extend(self.out_of_bounds_points.clone()),
                RelativePointPos::TopLeft     => all_values.extend(self.top_left.clone().get_values()),
                RelativePointPos::TopRight    => all_values.extend(self.top_right.clone().get_values()),
                RelativePointPos::BottomLeft  => all_values.extend(self.bottom_left.clone().get_values()),
                RelativePointPos::BottomRight => all_values.extend(self.bottom_right.clone().get_values()),
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
                RelativePointPos::TopLeft     => all_bounds.extend(self.top_left.clone().get_bounds()),
                RelativePointPos::TopRight    => all_bounds.extend(self.top_right.clone().get_bounds()),
                RelativePointPos::BottomLeft  => all_bounds.extend(self.bottom_left.clone().get_bounds()),
                RelativePointPos::BottomRight => all_bounds.extend(self.bottom_right.clone().get_bounds()),
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
            let ((xl, xh), (yl, yh)) = ((px - half, px + half), (py - half, py + half));
            all_lines_data.extend(vec![
                xl, yl, 0., 1., 1., 1., 1., // bottom line
                xh, yl, 0., 1., 1., 1., 1., // bottom line

                xl, yh, 0., 1., 1., 1., 1., // top line
                xh, yh, 0., 1., 1., 1., 1., // top line

                xl, yl, 0., 1., 1., 1., 1., // left line
                xl, yh, 0., 1., 1., 1., 1., // left line

                xh, yl, 0., 1., 1., 1., 1., // right line
                xh, yh, 0., 1., 1., 1., 1., // right line
            ]);
        }
        Matrix::from_vec_with_shape(all_lines_data, S2::<7, LINES_UPPER_BOUND>)
    }
}
impl QuadTree<PointThing> {
    pub fn get_all_points<const NUM_POINTS:usize>(&self) -> Matrix<f32, 2, S2<7, NUM_POINTS>> {
        let all_points = self.get_all_values();
        let mut all_points_data = vec![];
        for point in all_points {
            all_points_data.extend(point.to_vec());
        }
        Matrix::from_vec_with_shape(all_points_data, S2::<7, NUM_POINTS>)
    }
}