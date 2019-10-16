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

pub trait Polar {
    fn from_polar(rho: f64, phi: f64) -> Point;
}

impl Polar for Point {
    fn from_polar(r: f64, phi: f64) -> Point {
        Point {
            x: r * phi.cos(),
            y: r * phi.sin(),
        }
    }
}

pub trait Orientation {
    fn orientation(&self, a: &Point, b: &Point) -> f64;
}

impl Orientation for Point {
    fn orientation(&self, a: &Point, b: &Point) -> f64 {
        // Approximate 2D orientation, courtesy of https://www.cs.cmu.edu/~quake/robust.html
        let acx = a.x - self.x;
        let bcx = b.x - self.x;
        let acy = a.y - self.y;
        let bcy = b.y - self.y;
        acx * bcy - acy * bcx
    }
}

pub trait Collision {
    fn is_colliding(&self, other: &Point, tolerance: f64) -> bool;
}

impl Collision for Point {
    fn is_colliding(&self, other: &Point, tolerance: f64) -> bool {
        (self.x - other.x).abs() < tolerance && (self.y - other.y).abs() < tolerance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const COLLISION_TOLERANCE: f64 = 20.0;

    #[test]
    fn test_that_orientation_is_negative_when_point_is_left_of_line() {
        let point = Point { x: 1.0, y: 1.0 };
        let line_a = Point { x: 0.0, y: 10.0 };
        let line_b = Point { x: 10.0, y: 0.0 };
        assert!(point.orientation(&line_a, &line_b).is_sign_negative());
    }

    #[test]
    fn test_that_orientation_is_positive_when_point_is_right_of_line() {
        let point = Point { x: 10.0, y: 10.0 };
        let line_a = Point { x: 0.0, y: 10.0 };
        let line_b = Point { x: 10.0, y: 0.0 };
        assert!(point.orientation(&line_a, &line_b).is_sign_positive());
    }

    #[test]
    fn test_that_orientation_is_zero_when_point_is_on_line() {
        let point = Point { x: 5.0, y: 5.0 };
        let line_a = Point { x: 0.0, y: 10.0 };
        let line_b = Point { x: 10.0, y: 0.0 };
        assert_eq!(point.orientation(&line_a, &line_b), 0.0);
    }

    #[test]
    fn test_that_collision_is_detected_when_point_is_near() {
        let point_a = Point::zero();
        let point_b = Point {
            x: COLLISION_TOLERANCE - 0.1,
            y: COLLISION_TOLERANCE - 0.1,
        };
        assert!(point_a.is_colliding(&point_b, COLLISION_TOLERANCE));
    }

    #[test]
    fn test_that_collision_is_not_detected_when_point_is_above_tolerance() {
        let point_a = Point::zero();
        let point_b = Point {
            x: COLLISION_TOLERANCE,
            y: COLLISION_TOLERANCE,
        };
        assert!(!point_a.is_colliding(&point_b, COLLISION_TOLERANCE));
    }

    #[test]
    fn test_that_collision_is_detected_when_points_are_same() {
        let point = Point { x: 1.0, y: 1.0 };
        assert!(point.is_colliding(&point, COLLISION_TOLERANCE));
    }
}
