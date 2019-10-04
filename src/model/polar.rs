use crate::model::point::Point;

pub trait Polar {
    fn from_polar(rho: f64, phi: f64) -> Point;
}
