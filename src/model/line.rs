use crate::model::point::{Point, Polar};

pub struct Line {
    pub(crate) a: Point,
    pub(crate) b: Point,
}

impl Line {
    pub fn new(pivot: &Point, radius: f64, rotation: f64) -> Line {
        let polar = Point::from_polar(radius, rotation);
        let line_a = Point {
            x: pivot.x + polar.x,
            y: pivot.y + polar.y,
        };
        let line_b = Point {
            x: pivot.x - polar.x,
            y: pivot.y - polar.y,
        };
        Line {
            a: line_a,
            b: line_b,
        }
    }
}
