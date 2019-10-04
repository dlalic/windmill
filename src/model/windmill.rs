use crate::model::point::Point;
use crate::model::windmill_point::WindmillPoint;

pub struct Windmill {
    pub rotation: f64,
    pub points: Vec<WindmillPoint>,
    pub pivot: Point,
    pub line: [Point; 2],
}
