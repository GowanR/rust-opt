
extern crate rand;

use benchmark::Func;
use self::rand::Rng;
use self::rand::ThreadRng;
use util::bounded;
use util::u;

#[derive(Debug, Clone)]
struct Particle {
    vel: Vec<f32>,
    pos: Vec<f32>,
    best: Vec<f32>,
    dna: Gene,
    new_best: f32,
    old_best: f32,
    min_integral: f32,
}
#[derive(Debug, Clone)]
struct Gene {
    current: f32,
    global: f32,
    local: f32,
}
impl Gene {
    fn mutate( g: Particle ) -> Particle {
        Particle {
            vel: g.vel,
            pos: g.pos,
            best: g.best,
            dna: Gene {
                current: g.dna.current + u(),
                global: g.dna.global + u(),
                local: g.dna.local + u(),
            },
            new_best: g.new_best,
            old_best: g.old_best,
            min_integral: g.min_integral,
        }
    }
}
pub struct Swarm<T: Func> {
    particles: Vec<Particle>,
    pub best: Vec<f32>,
    function: T,
    random: ThreadRng,
}
/// Constructs a new `Particle`.
///
/// # Examples
///
/// ```
/// let p = Particle::new( 10, -10f32, 10f32 );
/// ```
impl Particle {
    fn new(d: i32, l: f32, u: f32, rnd: &mut ThreadRng) -> Particle {
        let mut p = Vec::new();
        let mut v = Vec::new();
        for _ in 0..d {
            let v_rand = bounded(rnd.gen(), -(u as f32).abs(), (l as f32).abs());
            let p_rand = bounded(rnd.gen(), l as f32, u as f32);
            v.push(v_rand);
            p.push(p_rand);
        }
        Particle {
            vel: v,
            pos: p.clone(),
            best: p.clone(),
            new_best: 0_f32,
            old_best: 0_f32,
            min_integral: 0_f32,
            dna: Gene {
                current: bounded( rnd.gen(), 0_f32, 4_f32 ),
                global:  bounded( rnd.gen(), 0_f32, 4_f32 ),
                local:   bounded( rnd.gen(), 0_f32, 4_f32 ),
            }
        }
    }
    fn update_velocity(&mut self, parent_best: &Vec<f32>, parent_rnd: &mut ThreadRng ) {
        let w = self.dna.current;
        let g = self.dna.global;
        let p = self.dna.local;
        let g_rand: f32 = parent_rnd.gen();
        let p_rand: f32 = parent_rnd.gen();
        for i in 0..self.vel.len() {
            self.vel[i] = ( w * self.vel[i] ) + ( p * p_rand * ( self.best[i] - self.pos[i] ) ) + ( ( g * g_rand * ( parent_best[i] - self.pos[i] ) ) );
        }
    }
}

impl<T: Func> Swarm<T> {
    /// Used to make a new `Swarm`.
    /// # Examples
    /// ```
    /// let dimentions = 10_i32; // Make a 10 dimentional function
    /// let lower_bound = -10_f32; // Define lower bound
    /// let upper_bound = 10_f32; // Define upper bound
    /// let number_of_particles = 100_i32; // Make 100 `Particles` is the `Swarm`
    /// use Swarm::benchmark::Rastrigin;
    /// use Swarm::pso::Swarm;
    /// let mut swarm = Swarm::new( dimentions, lower_bound, upper_bound, number_of_particles, Rastrigin );
    /// let min = swarm.opt( 100 ); //optimize with 100 iterations
    /// ```
    pub fn new(d: i32, l: f32, u: f32, n: i32, function: T ) -> Swarm<T> {
        let mut rnd = rand::thread_rng();
        let mut parts = Vec::new();
        let dim = if function.d().is_positive() {
            function.d()
        } else {
            d
        };
        for _ in 0..n {
            parts.push(Particle::new(dim, l, u, &mut rnd));
        }
        Swarm {
            best: parts[0].pos.clone(),
            particles: parts,
            function: function,
            random: rnd,
        }
    }
    pub fn iterate(&mut self) {
        for part in &mut self.particles {
            part.update_velocity( &self.best, &mut self.random );
            for i in 0..part.pos.len() {
                part.pos[i] += part.vel[i];
            }
            if self.function.func(&part.pos) < self.function.func(&part.best) {
                part.best = part.pos.clone();
                part.new_best = self.function.func(&part.pos);
                part.min_integral += part.new_best - part.old_best;
            }
            part.old_best = part.new_best.clone();
            if self.function.func(&part.best) < self.function.func(&self.best) {
                self.best = part.best.clone();
            }
        }
        // Genetic computations.
        self.particles.sort_by(| a, b | a.min_integral.partial_cmp( &b.min_integral ).unwrap());
        let best = self.particles.clone();
        let ( best, _ ) = best.split_at( self.particles.len()/2 );
        {
            let mut ind = &mut self.particles;
            for i in 0..ind.len() {
                if i % 2 == 0 {
                    ind[i]     = Gene::mutate( best[i/2].clone() );
                    ind[i + 1] = Gene::mutate( best[i/2].clone() );
                }
            }
        }
    }
    /// Has swarm optimize.
    /// #Examples
    /// ```
    /// use Swarm::benchmark::Rastrigin;
    /// let mut s = Swarm::new(100, -10f32, 10f32, 100, Rastrigin );
    /// let mut minimum = s.opt(100);
    /// println!("Min: {:?}", minimum);
    /// println!("Value: {}", Rastrigin.func(&mut minimum));
    /// ```
    pub fn opt(&mut self, n: i32) -> Vec<f32> {
        for _ in 0..n {
            self.iterate();
        }
        self.best.clone()
    }
}
