//! General utilities.

extern crate rand;
use self::rand::random;
// Used for turning random [0,1] -> [l,u]
pub fn bounded(r: f32, l: f32, u: f32) -> f32 {
    r * (u - l) + l
}
pub fn u() -> f32 {
    bounded( random::<f32>(), -1., 1. )
}
