//! Provides an array of benchmark functions for optimization algorithms.

pub trait Func {
    fn func(&self, &Vec<f32>) -> f32;
    fn d( &self ) -> i32;
    fn minarg( &self ) -> Vec<f32>;
    fn min( &self ) -> f32;
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

#[derive(Copy, Clone)]
pub struct Rosenbrock;

#[derive(Copy, Clone)]
pub struct McCormick;

#[derive(Copy, Clone)]
pub struct Ackley;

#[derive(Copy, Clone)]
pub struct DropWave;

impl Func for Rastrigin {
    fn func(&self, x: &Vec<f32>) -> f32 {
        use std::f64::consts::PI;
        let mut sum: f32 = 0.0;
        for i in 0..x.len() {
            sum += x[i].powi(2) - (10.0 * ((PI as f32) * 2.0 * x[i]).cos());
        }
        sum + ((x.len() as f32) * 10.0)
    }
    fn d( &self ) -> i32 { -1 }
    fn minarg( &self ) -> Vec<f32> { vec![ 0_f32 ] }
    fn min( &self ) -> f32 { 0_f32 }
}
impl Func for Sphere {
    fn func(&self, x: &Vec<f32>) -> f32 {
        let mut sum = 0_f32;
        for i in 0..x.len() {
            sum += x[i].powi(2);
        }
        sum
    }
    fn d( &self ) -> i32 { -1 }
    fn minarg( &self ) -> Vec<f32> { vec![ 0_f32 ] }
    fn min( &self ) -> f32 { 0_f32 }
}

impl Func for Rosenbrock {
    fn func(&self, x: &Vec<f32>) -> f32 {
        let mut sum = 0_f32;
        for i in 0..x.len() {
            if !(i % 2 == 0) {
                sum += ( 1_f32 - x[i-1] ).powi(2) + 100_f32 * ( x[i] - x[i-1].powi(2) ).powi(2);
            }
        }
        sum
    }
    fn d( &self ) -> i32 { -1 }
    fn minarg( &self ) -> Vec<f32> { vec![ 1_f32 ] }
    fn min( &self ) -> f32 { 0_f32 }
}

impl Func for McCormick {
    fn func(&self, x: &Vec<f32>) -> f32 {
        (x[0] + x[1]).sin() + ( x[0] - x[1] ).powi(2) + (-1.5*x[0]) + (2.5_f32*x[1]) + 1_f32
    }
    fn d( &self ) -> i32 { 2 }
    fn minarg( &self ) -> Vec<f32> { vec! [-0.54719_f32, -1.54719_f32] }
    fn min( &self ) -> f32 { -1.9133_f32 }
}

impl Func for Ackley {
    fn func( &self, x: &Vec<f32> ) -> f32 {
        use std::f32::consts::PI;
        let a = 20_f32;
        let b = 0.2_32;
        let c = 2_f32 * (PI);
        let mut s1 = 0_f32;
        let mut s2 = 0_f32;
        for i in 0..x.len() {
            s1 += x[i].powi(2);
            s2 += (c*x[i]).cos()
        }
        let inv_d: f32 = 1_f32/(x.len() as f32);
        (-a * ( -b * inv_d * s1 ).exp()) - ( inv_d * s2 ).exp() + a + (1 as f32).exp()
    }
    fn d( &self ) -> i32 { -1 }
    fn minarg( &self ) -> Vec<f32> { vec! [0_f32] }
    fn min( &self ) -> f32 { 0_f32 }
}

impl Func for DropWave {
    fn func( &self, x: &Vec<f32>) -> f32 {
        -1_f32 * ((1_f32 + (12_f32*(x[0].powi(2)+x[1].powi(2)).sqrt()).cos())
        /
        (0.5_f32*(x[0].powi(2)+x[1].powi(2))+2_f32))
    }
    fn d( &self ) -> i32 { 2 }
    fn minarg( &self ) -> Vec<f32> { vec! [0_f32, 0_f32] }
    fn min( &self ) -> f32 { -1_f32 }
}
