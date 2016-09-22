extern crate rand;
extern crate nadezhda;

use nadezhda::environment::Environment;
use nadezhda::population::Population;
use nadezhda::darwin::evolve::succesion;

fn main() {
    let mut rng = rand::thread_rng();
    let environment: Environment = Environment::new(5);
    let last_population: Population = Population::new(5);

    println!("{:?}", last_population);

    let next_population = succesion(&mut rng, environment, last_population);

    println!("{:?}", next_population);

}
