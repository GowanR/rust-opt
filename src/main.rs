extern crate rust_opt;

fn main() {
    //use Swarm::gene_pso::Swarm;
    use rust_opt::benchmark::Func;

    use rust_opt::gene::Population;

    use rust_opt::benchmark::Sphere;
    use rust_opt::benchmark::Rastrigin;

    let mut pop = Population::new( 1000, 2, Sphere );
    let solution = pop.opt( 1000 );
    println!("Min: {:?}", solution);
    println!("Value: {}", Rastrigin.func(&solution));

    /*
    let mut s = Swarm::new(100, -10f32, 10f32, 1000, Sphere);
    let mut minimum = s.opt(1000);
    println!("Min: {:?}", minimum);
    println!("Value: {}", Rastrigin.func(&minimum));
    */
}
