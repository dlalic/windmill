use crate::model::hit_counter::HitCounter;
use crate::model::point::Point;

#[derive(Debug, Copy, Clone)]
pub struct WindmillPoint {
    pub point: Point,
    pub orientation: f64,
    hit_count: i32,
}

impl WindmillPoint {
    pub fn new(point: &Point) -> WindmillPoint {
        WindmillPoint {
            point: *point,
            orientation: 0.0,
            hit_count: 0,
        }
    }

    pub fn is_orientation_switched(&self, new_orientation: f64) -> bool {
        // Check if the result of multiplication is < 0 instead of
        // if orientation < 0 && new_orientation > 0 || orientation > 0 && new_orientation < 0
        let result = self.orientation * new_orientation;
        result.trunc() != 0.0 && result.is_sign_negative()
    }
}

impl HitCounter for WindmillPoint {
    fn hit_count(&self) -> i32 {
        self.hit_count
    }

    fn increase_hit_count(&mut self) {
        self.hit_count += 1;
    }

    fn reset_hit_count(&mut self) {
        self.hit_count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_that_orientation_switch_is_detected_for_negative_to_positive() {
        let mut point = WindmillPoint::new(&Point::zero());
        point.orientation = -1.0;
        assert!(point.is_orientation_switched(1.0));
    }

    #[test]
    fn test_that_orientation_switch_is_detected_for_positive_to_negative() {
        let mut point = WindmillPoint::new(&Point::zero());
        point.orientation = 1.0;
        assert!(point.is_orientation_switched(-1.0));
    }

    #[test]
    fn test_that_orientation_switch_ignores_0() {
        let mut point = WindmillPoint::new(&Point::zero());
        point.orientation = -0.0;
        assert!(!point.is_orientation_switched(0.0));
    }
}
