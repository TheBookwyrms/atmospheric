pub trait Point {
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
    fn get_z(&self) -> f32;
    fn get_xyz(&self) -> (f32, f32, f32);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointThing {
    pub x:f32,
    pub y:f32,
    pub z:f32,
    pub r:f32,
    pub g:f32,
    pub b:f32,
    pub a:f32,
}
impl PointThing {
    pub fn new(x:f32, y:f32, z:f32) -> Self {
        Self { x, y, z, r:1., g:1., b:1., a:1. }
    }
    pub fn to_vec(&self) -> Vec<f32> {
        vec![self.x, self.y, self.z, self.r, self.g, self.b, self.a]
    }
}
impl Point for PointThing {
    fn get_x(&self) -> f32 { self.x }
    fn get_y(&self) -> f32 { self.y }
    fn get_z(&self) -> f32 { self.z }
    fn get_xyz(&self) -> (f32, f32, f32) { ( self.x, self.y, self.z ) }
}


#[derive(Debug)]
pub enum TreeError {
    CannotSubdivideNode,
    CannotSubdivideValue,
    NodeIsNone,
    PointOutOfBounds,
    NodeAlreadySubdivided,
    QuadTreeNotInitialised,
    SituationNotYetEncounteredOrTested,
}