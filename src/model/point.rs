use crate::model::axis::Axis;
use crate::model::collinear::Collinear;
use crate::model::projection::Projection;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Collinear for Point {
    fn is_collinear(&self, a: &Point, b: &Point) -> bool {
        return false;
    }
}

impl Projection for Point {
    fn projection(&self, axis: Axis, rotation: f64) -> Point {
        match axis {
            Axis::X => {
                let px = self.x - self.y / rotation.tan();
                Point { x: px, y: 0.0 }
            }
            Axis::Y => {
                unimplemented!();
            }
        }
    }
}
