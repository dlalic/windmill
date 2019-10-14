use crate::model::point::Point;

pub trait Collision {
    fn is_colliding(&self, other: &Point) -> bool;
}
