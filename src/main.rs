extern crate rust_opt;

use rust_opt::benchmark::Func;

fn print_stats<T: Func>( foundarg: Vec<f32>, problem: T ) {
    println!("Found Minarg: {:?}, Actual Minarg: {:?}", foundarg, problem.minarg() );
    println!("Found Min: {:?}, Actual Min: {:?}", problem.func( &foundarg ), problem.min() );
}

fn main() {
    use rust_opt::benchmark::Func;

    use rust_opt::gene::Population;
    use rust_opt::gd::GD;

    use rust_opt::benchmark;

    let problem = benchmark::Ackley.clone();

    let mut gd = GD::new( 2, problem );
    gd.set_gamma( 0.001_f32 );
    let mut solution = gd.opt( 1000 );

    println!( "Gradient Decent" );
    print_stats( solution, problem );
    println!("");

    let mut pop = Population::new( 1000, 2, problem );
    solution = pop.opt( 1000 );

    println!( "Evolution" );
    print_stats( solution, problem );
    println!("");
    {
        use rust_opt::pso::Swarm;
        let mut s = Swarm::new(2, -10f32, 10f32, 1000, problem);
        solution = s.opt(1000);

        println!( "PSO" );
        print_stats( solution, problem );
    }
    {
        use rust_opt::gene_pso::Swarm;
        let mut s = Swarm::new(2, -10f32, 10f32, 1000, problem);
        solution = s.opt(1000);

        println!( "Genetic PSO" );
        print_stats( solution, problem );
    }
    println!("");
    println!("f({:?}) = {}", problem.minarg(), problem.func( &problem.minarg() ) )
}
