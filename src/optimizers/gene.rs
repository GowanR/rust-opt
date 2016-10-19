//! Module for general purpose numerical evolution for optimization.

extern crate rand;

use benchmark::Func;
use self::rand::Rng;
use util::bounded;

#[derive(Clone)]
struct Gene<T: Func> {
    sequence: Vec<f32>,
    problem: T,
}

pub struct Population<T: Func> {
    individuals: Vec<Gene<T>>,
}

impl<T: Func> Gene<T> {
    pub fn new( d: i32, problem: T ) -> Gene<T> {
        let mut seq = Vec::new();
        let mut rnd = rand::thread_rng();
        let dim = if problem.d().is_positive() {
            problem.d()
        } else {
            d
        };
        for _ in 0..dim {
            seq.push( bounded( rnd.gen(), -10_f32, 10_f32 ) );
        }
        Gene {
            sequence: seq,
            problem: problem,
        }
    }
    fn fitness( &self ) -> f32 {
        self.problem.func( &self.sequence )
    }
    fn mutate_compare( best: Gene<T>, worst: Gene<T> ) -> Gene<T> {
        let mut seq = Vec::new();
        for i in 0..best.sequence.len() {
            //let x = (/*best.sequence[i]*/ 1_f32 - worst.sequence[i] );
            seq.push( /*bounded( rand::random::<f32>(), -0.5_f32, 0.5_f32 ) +*/ best.sequence[i]+best.sequence[i] );
        }
        Gene {
            sequence: seq,
            problem: best.problem,
        }
    }
    fn mutate( individual: Gene<T> ) -> Gene<T> {
        let mut rnd = rand::thread_rng();
        let mut seq = Vec::new();
        for i in 0..individual.sequence.len() {
            seq.push( individual.sequence[i] + bounded( rnd.gen(), -1_f32, 1_f32 ) );
        }
        Gene {
            sequence: seq,
            problem: individual.problem,
        }
    }
}

impl<T: Func + Clone> Population<T> {
    pub fn new( n:i32, d: i32, problem: T ) -> Population<T> {
        let mut ind = Vec::new();
        for _ in 0..n {
            ind.push( Gene::new( d, problem.clone() ) );
        }
        Population {
            individuals: ind,
        }
    }
    pub fn iterate( &mut self ) {
        self.individuals.sort_by(| a, b | a.fitness().partial_cmp( &b.fitness()).unwrap());
        let best = self.individuals.clone();
        let ( best, _ ) = best.split_at( self.individuals.len()/2 );
        {
            let mut ind = &mut self.individuals;
            for i in 0..ind.len() {
                if i % 2 == 0 {
                    ind[i]     = Gene::mutate( best[i/2].clone() );
                    ind[i + 1] = Gene::mutate( best[i/2].clone() );
                }
            }
        }
    }
    pub fn comp_iterate( &mut self ) {
        self.individuals.sort_by(| a, b | a.fitness().partial_cmp( &b.fitness()).unwrap());
        let best = self.individuals.clone();
        let worst = self.individuals.clone();
        let ( best, worst ) = best.split_at( self.individuals.len()/2 );
        {
            let mut ind = &mut self.individuals;
            for i in 0..ind.len() {
                if i % 2 == 0 {
                    // ind[i]     = Gene::mutate( best[i/2].clone() );
                    // ind[i + 1] = Gene::mutate( best[i/2].clone() );
                    ind[i]     = Gene::mutate_compare( best[i/2].clone(), worst[i/2].clone() );
                    ind[i + 1] = Gene::mutate_compare( best[i/2].clone(), worst[i/2].clone() );
                }
            }
        }
    }
    pub fn comp_opt( &mut self, n: i64 ) -> Vec<f32> {
        println!( "comp_opt() => ()" );
        for i in 0..n {
            //println!("itter {}", i );
            self.comp_iterate();
        }
        self.individuals.sort_by(|a, b| a.fitness().partial_cmp( &b.fitness()).unwrap());
        self.individuals[0].sequence.clone()
    }
    pub fn opt( &mut self, n: i64 ) -> Vec<f32> {
        for _ in 0..n {
            self.iterate();
        }
        self.individuals.sort_by(|a, b| a.fitness().partial_cmp( &b.fitness()).unwrap());
        self.individuals[0].sequence.clone()
    }
}
