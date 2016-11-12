extern crate rust_opt;
extern crate rand;
//extern crate redis;
//use redis::Commands;

use rust_opt::benchmark::Func;
//use std::io::prelude::*;
//use std::fs::File;
use rust_opt::optimizers::gene::Population;
//use rust_opt::optimizers::gd::GD;

use rust_opt::benchmark;
fn print_stats<T: Func>( foundarg: Vec<f32>, problem: T ) {
    println!("Found Minarg: {:?}, Actual Minarg: {:?}", foundarg, problem.minarg() );
    println!("Found Min: {:?}, Actual Min: {:?}", problem.func( &foundarg ), problem.min() );
}
// pub fn get_job( s: String ) -> redis::RedisResult<String> {
//     let client = try!(redis::Client::open("redis://raspberrypi.local/"));
//     let con = try!(client.get_connection());
//     con.get( s )
// }
// fn get_jobs()-> redis::RedisResult<String> {
//     let client = try!(redis::Client::open("redis://raspberrypi.local/"));
//     let con = try!(client.get_connection());
//     con.get( "jobs" )
// }
// fn get_job_list() -> Vec<String> {
//     let jobs = get_jobs().unwrap();
//     let x: Vec<&str> = jobs.split(',').collect();
//     let mut rn = Vec::new();
//     for i in 0..x.len() {
//         rn.push( x[i].to_string() );
//     }
//     rn
// }
// fn parse_job( job: String ) {
//     let x: Vec<&str> = job.split(',').collect();
//     let mut list = Vec::new();
//     for i in 0..x.len() {
//         list.push( x[i].to_string() );
//     }
//     match list[0].as_ref() {
//         "PSO" => println!( "PSO algorithm!" ),
//         "EVOLVE" => println!( "Evolution algorithm!" ),
//         "GENEPSO" => println!( "Genetic PSO algorithm!" ),
//         _ => println!( "Undefined" ),
//     }
// }

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
