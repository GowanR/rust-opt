//! Provides an array of benchmark functions for optimization algorithms.


pub trait Func {
    fn func(&self, &Vec<f32>) -> f32;
}

/// An `N-Dimentional` Rastrigin.
/// # Examples
/// ```
/// use Swarm::pso::Swarm;
/// use Swarm::benchmark::Rastrigin;
/// let mut s = Swarm::new(100, -10f32, 10f32, 100, Rastrigin );
/// let mut minimum = s.opt(100);
/// ```
#[derive(Copy, Clone)]
pub struct Rastrigin;

/// An `N-Dimentional` Sphere.
/// # Examples
/// ```
/// use Swarm::pso::Swarm;
/// use Swarm::benchmark::Sphere;
/// let mut s = Swarm::new(100, -10f32, 10f32, 100, Sphere );
/// let mut minimum = s.opt(100);
/// ```
#[derive(Copy, Clone)]
pub struct Sphere;

impl Func for Rastrigin {
    fn func(&self, x: &Vec<f32>) -> f32 {
        use std::f64::consts::PI;
        let mut sum: f32 = 0.0;
        for i in 0..x.len() {
            sum += x[i].powi(2) - (10.0 * ((PI as f32) * 2.0 * x[i]).cos());
        }
        sum + ((x.len() as f32) * 10.0)
    }
}
impl Func for Sphere {
    fn func(&self, x: &Vec<f32>) -> f32 {
        let mut sum = 0_f32;
        for i in 0..x.len() {
            sum += x[i].powi(2);
        }
        sum
    }
}
