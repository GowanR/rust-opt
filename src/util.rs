//! General utilities.

// Used for turning random [0,1] -> [l,u]
pub fn bounded(r: f32, l: f32, u: f32) -> f32 {
    r * (u - l) + l
}
