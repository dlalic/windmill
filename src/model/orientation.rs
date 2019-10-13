use crate::model::point::Point;

pub trait Orientation {
    fn orientation(&self, a: Point, b: Point) -> f64;
}
