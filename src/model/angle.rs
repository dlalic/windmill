use crate::model::point::Point;

pub trait Angle {
    fn angle(&self, other: &Point) -> f64;
}
