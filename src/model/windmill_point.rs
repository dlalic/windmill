use crate::model::point::Point;

#[derive(Debug, Copy, Clone)]
pub struct WindmillPoint {
    pub point: Point,
    pub orientation: f64,
    pub hit_count: i32,
}

impl WindmillPoint {
    pub fn new(point: &Point) -> WindmillPoint {
        WindmillPoint {
            point: *point,
            orientation: 0.0,
            hit_count: 0,
        }
    }
}
