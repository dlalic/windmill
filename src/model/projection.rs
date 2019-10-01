use crate::model::axis::Axis;
use crate::model::point::Point;

pub trait Projection {
    fn projection(&self, axis: Axis, rotation: f64) -> Point;
}
