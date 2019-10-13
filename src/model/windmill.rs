use crate::model::orientation::Orientation;
use crate::model::point::Point;
use crate::model::polar::Polar;
use crate::model::windmill_point::WindmillPoint;

pub struct Windmill {
    pub rotation: f64,
    pub speed: f64,
    pub points: Vec<WindmillPoint>,
    pub pivot: Point,
    pub line: [Point; 2],
}

impl Windmill {
    pub fn reset() -> Windmill {
        Windmill {
            rotation: 0.0,
            speed: 1.0,
            points: vec![],
            pivot: Point::zero(),
            line: [Point::zero(), Point::zero()],
        }
    }

    pub fn calculate_line(&mut self, radius: f64) {
        let polar = Point::from_polar(radius, self.rotation);
        self.line[0].x = self.pivot.x + polar.x;
        self.line[0].y = self.pivot.y + polar.y;
        self.line[1].x = self.pivot.x - polar.x;
        self.line[1].y = self.pivot.y - polar.y;
    }

    pub fn detect_new_pivot(&mut self) {
        for point in &mut self.points {
            if point.point == self.pivot {
                continue;
            }
            let orientation = point.point.orientation(self.line[0], self.line[1]);
            // Check if the result of multiplication is < 0 instead of
            // if orientation < 0 && previous > 0 || orientation > 0 && previous < 0
            let result = orientation * point.orientation;
            point.orientation = orientation;
            if result.trunc() != 0.0 && result.is_sign_negative() {
                self.pivot = point.point;
                point.hit_count += 1;
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_detect_new_pivot() {
        let mut windmill = Windmill::reset();
        let point = WindmillPoint::new(&Point { x: 5.0, y: 5.0 });
        windmill.points.push(point);
        windmill.rotation = PI / 4.0 - 0.01;
        windmill.calculate_line(10.0);
        windmill.detect_new_pivot();
        windmill.rotation = PI / 4.0 + 0.01;
        windmill.calculate_line(10.0);
        windmill.detect_new_pivot();
        assert_eq!(windmill.pivot, point.point);
    }
}
