use crate::model::angle::Angle;
use crate::model::collinear::Collinear;
use crate::model::polar::Polar;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Collinear for Point {
    fn is_collinear(&self, a: &Point, b: &Point) -> bool {
        return false;
    }
}

impl Angle for Point {
    fn angle(&self, other: &Point) -> f64 {
        return (self.y - other.y).atan2(self.x - other.x);
    }
}

impl Polar for Point {
    fn from_polar(r: f64, phi: f64) -> Point {
        Point {
            x: r * phi.cos(),
            y: r * phi.sin(),
        }
    }
}
