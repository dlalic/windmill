use crate::model::line::Line;
use crate::model::point::{Collision, Point};
use crate::model::windmill_point::WindmillPoint;

const COLLISION_TOLERANCE: f64 = 20.0;

pub struct Windmill {
    radius: f64,
    rotation: f64,
    speed: f64,
    points: Vec<WindmillPoint>,
    pivot: Option<Point>,
}

impl Windmill {
    pub fn reset() -> Windmill {
        Windmill {
            radius: 100.0,
            rotation: 0.0,
            speed: 1.0,
            points: vec![],
            pivot: None,
        }
    }

    fn register_new_pivot(&mut self, cursor: WindmillPoint) {
        for point in &mut self.points {
            point.reset_hit_count();
            if point.get_point() == cursor.get_point() {
                self.pivot = Option::from(*point.get_point());
                point.increase_hit_count();
            }
        }
    }

    fn detect_new_pivot(&mut self) {
        if let Some(line) = self.line() {
            for point in &mut self.points {
                let orientation = point.get_orientation(&line.a, &line.b);
                if point.is_orientation_switched(orientation) {
                    self.pivot = Option::from(*point.get_point());
                    point.increase_hit_count();
                    break;
                }
                point.set_orientation(orientation);
            }
        }
    }

    pub fn update_radius(&mut self, radius: f64) {
        self.radius = radius;
    }

    pub fn update_rotation(&mut self, rotation: f64) {
        self.rotation += self.speed * rotation;
        self.detect_new_pivot();
    }

    pub fn update_speed(&mut self, speed: f64) {
        self.speed += speed;
        if self.speed < 0.1 {
            self.speed = 0.1
        }
    }

    pub fn register_new_point(&mut self, cursor: &Point) {
        let collision = self
            .points
            .iter()
            .find(|point| cursor.is_colliding(point.get_point(), COLLISION_TOLERANCE));
        match collision {
            None => {
                let point = WindmillPoint::new(cursor);
                self.points.push(point);
                if self.points.len() == 1 {
                    self.register_new_pivot(point);
                }
            }
            Some(&point) => {
                self.register_new_pivot(point);
            }
        }
    }

    pub fn get_points(&self) -> &Vec<WindmillPoint> {
        &self.points
    }

    pub fn get_pivot(&self) -> &Option<Point> {
        &self.pivot
    }

    pub fn line(&self) -> Option<Line> {
        // TODO: memoize
        self.pivot
            .map(|pivot| Line::new(&pivot, self.radius, self.rotation))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_that_clicking_on_the_point_makes_it_a_pivot() {
        let mut windmill = Windmill::reset();
        let point_a = Point::zero();
        windmill.register_new_point(&point_a);
        let point_b = Point {
            x: COLLISION_TOLERANCE - 1.0,
            y: COLLISION_TOLERANCE - 1.0,
        };
        windmill.register_new_point(&point_b);
        assert_eq!(windmill.points.len(), 1);
        assert_eq!(windmill.get_pivot().unwrap(), point_a);
    }

    #[test]
    fn test_that_new_pivot_is_detected_after_updating_rotation() {
        let mut windmill = Windmill::reset();
        let point_a = Point::zero();
        let point_b = Point {
            x: COLLISION_TOLERANCE,
            y: COLLISION_TOLERANCE,
        };
        windmill.register_new_point(&point_a);
        windmill.register_new_point(&point_b);
        let epsilon = 0.01;
        windmill.update_rotation(PI / 4.0 - epsilon);
        assert_eq!(windmill.get_pivot().unwrap(), point_a);
        windmill.update_rotation(PI / 4.0 + epsilon);
        assert_eq!(windmill.get_pivot().unwrap(), point_b);
    }
}
