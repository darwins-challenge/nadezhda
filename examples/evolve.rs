extern crate rand;
extern crate nadezhda;

use nadezhda::environment::Environment;
use nadezhda::population::Population;
use nadezhda::darwin::evolve::succesion;

fn main() {
    let environment: Environment = Environment::new(5);
    let last_population: Population = Population::new(5);

    let next_population = succesion(environment, last_population);

    println!("{:?}", next_population);

}
