extern crate rust_opt;
extern crate rand;

use rust_opt::benchmark::Func;
use rust_opt::optimizers::gene::Population;
use rust_opt::benchmark;


fn print_stats<T: Func>( foundarg: Vec<f32>, problem: T ) {
    println!("Found Minarg: {:?}, Actual Minarg: {:?}", foundarg, problem.minarg() );
    println!("Found Min: {:?}, Actual Min: {:?}", problem.func( &foundarg ), problem.min() );
}

fn main() {

    let problem = benchmark::Rosenbrock.clone();
    let mut solution = Vec::new();
    {
        let mut pop = Population::new( 1000, 100, problem );
        solution = pop.opt( 1000 );

        println!( "Evolution" );
        print_stats( solution, problem );
    }
    println!("");
    {
        let mut pop = Population::new( 1000, 100, problem );
        solution = pop.comp_opt( 1000 );

        println!( "Progressive Evolution" );
        print_stats( solution, problem );
    }
    println!("");
    {
        use rust_opt::optimizers::pso::Swarm;
        let mut s = Swarm::new( 100, -10f32, 10f32, 1000, problem);
        solution = s.opt(1000);

        println!( "PSO" );
        print_stats( solution, problem );
    }
    println!("");
    {
        use rust_opt::optimizers::gene_pso::Swarm;
        let mut s = Swarm::new( 100, -10f32, 10f32, 1000, problem);
        solution = s.opt(1000);

        println!( "Genetic PSO" );
        print_stats( solution, problem );
    }
    println!("");
    {
        use rust_opt::optimizers::bayes_pso::Swarm;
        let mut s = Swarm::new( 100, -10f32, 10f32, 1000, problem);
        solution = s.opt(1000);

        println!( "Bayes Genetic PSO" );
        print_stats( solution, problem );
    }
    println!("");
    println!("f({:?}) = {}", problem.minarg(), problem.func( &problem.minarg() ) );
}
