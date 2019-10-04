use crate::model::point::Point;

pub struct WindmillPoint {
    pub point: Point,
    pub hit_count: i32,
}

impl WindmillPoint {
    pub fn new(point: &Point) -> WindmillPoint {
        WindmillPoint {
            point: *point,
            hit_count: 0,
        }
    }
}
