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
    gamma: f32,
}
impl<T: Func> GD<T> {
    pub fn new(d: i32, f: T) -> GD<T> {
        let mut old: Vec<f32> = Vec::new();
        let mut new: Vec<f32> = Vec::new();
        let dim = if f.d().is_positive() {
            f.d()
        } else {
            d
        };
        for _ in 0..dim {
            old.push(5_f32);
            new.push(10_f32);
        }
        GD {
            function: f,
            old: old,
            new: new,
            gamma: 0.01_f32,
        }
    }
    pub fn set_gamma( &mut self, g: f32 ) {
        self.gamma = g;
    }
    pub fn opt(&mut self, n: i32) -> Vec<f32> {
        let gamma = self.gamma;
        for _ in 0..n {
            self.old = self.new.clone();
            for i in 0..self.new.len() {
                self.new[i] = -gamma *
                              (self.function.func(&self.old) - self.function.func(&self.new));
            }
        }
        return self.new.clone();
    }
}
