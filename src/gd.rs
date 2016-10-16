//! Quick gradient decent implemementation.
//! # Examples
//! ```
//! use Swarm::gd::GD;
//! use Swarm::benchmark::Sphere;
//! let mut decent = GD::new( 1000, Sphere );
//! let mut minimum = decent.train( 100 );
//! println!("{:?}", minimum);
//! println!("Value: {}", Sphere.func(&minimum) );
//! ```

use benchmark::Func;
pub struct GD<T: Func> {
    function: T,
    old: Vec<f32>,
    new: Vec<f32>,
}
impl<T: Func> GD<T> {
    pub fn new(d: i32, f: T) -> GD<T> {
        let mut old: Vec<f32> = Vec::new();
        let mut new: Vec<f32> = Vec::new();
        for _ in 0..d {
            old.push(0_f32);
            new.push(10_f32);
        }
        GD {
            function: f,
            old: old,
            new: new,
        }
    }
    pub fn train(&mut self, n: i32) -> Vec<f32> {
        let gamma = 0.01_f32;
        for _ in 0..n {
            self.old = self.new.clone();
            for i in 0..self.new.len() {
                self.new[i] = -gamma *
                              (self.function.func(&self.old) -
                               self.function.func(&self.new));
            }
        }
        return self.new.clone();
    }
}
