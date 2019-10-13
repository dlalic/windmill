use crate::model::polar::Polar;
use crate::model::orientation::{Orientation};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Polar for Point {
    fn from_polar(r: f64, phi: f64) -> Point {
        Point {
            x: r * phi.cos(),
            y: r * phi.sin(),
        }
    }
}

impl Orientation for Point {
    fn orientation(&self, a: Point, b: Point) -> f64 {
        // Approximate 2D orientation, courtesy of https://www.cs.cmu.edu/~quake/robust.html
        let acx = a.x - self.x;
        let bcx = b.x - self.x;
        let acy = a.y - self.y;
        let bcy = b.y - self.y;
        return acx * bcy - acy * bcx;
    }
}