use crate::model::point::Point;

pub trait Collinear {
    fn is_collinear(&self, a: &Point, b: &Point) -> bool;
}
