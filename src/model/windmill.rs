use crate::model::point::Point;

pub struct Windmill {
    pub rotation: f64,
    pub points: Vec<Point>,
    pub pivot: Point,
}
