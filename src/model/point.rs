use crate::model::orientation::Orientation;
use crate::model::polar::Polar;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn zero() -> Point {
        Point { x: 0.0, y: 0.0 }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_that_orientation_is_negative_when_point_is_left_of_line() {
        let point = Point { x: 1.0, y: 1.0 };
        let line_a = Point { x: 0.0, y: 10.0 };
        let line_b = Point { x: 10.0, y: 0.0 };
        assert!(point.orientation(line_a, line_b).is_sign_negative());
    }

    #[test]
    fn test_that_orientation_is_positive_when_point_is_right_of_line() {
        let point = Point { x: 10.0, y: 10.0 };
        let line_a = Point { x: 0.0, y: 10.0 };
        let line_b = Point { x: 10.0, y: 0.0 };
        assert!(point.orientation(line_a, line_b).is_sign_positive());
    }

    #[test]
    fn test_that_orientation_is_zero_when_point_is_on_line() {
        let point = Point { x: 5.0, y: 5.0 };
        let line_a = Point { x: 0.0, y: 10.0 };
        let line_b = Point { x: 10.0, y: 0.0 };
        assert_eq!(point.orientation(line_a, line_b), 0.0);
    }
}
