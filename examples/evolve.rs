extern crate rand;
extern crate nadezhda;

use nadezhda::grammar::Program;
use nadezhda::environment::Environment;
use nadezhda::population::Population;
use nadezhda::evaluate::Evaluator;
use nadezhda::darwin::evolve::succesion;

fn main() {
    let mut rng = rand::thread_rng();
    let environment: Environment = Environment::new(5);
    let mut last_population: Population = Population::new(5);

    let mut generation_count = 0;
    while generation_count < 100 {
        let next_population = succesion(&mut rng, environment.clone(), last_population.clone());
        let Population(generation) = next_population.clone();
        let survivors: Vec<Program> = generation
            .iter()
            .map(|program| (program.clone(), program.evaluate_on(environment.clone())))
            .filter(|&(_, ref env)| env.cockroach_location - env.food_location == 0)
            .map(|(program, _)| program)
            .collect();

        if survivors.len() > 0 { break; }

        last_population = next_population;
        generation_count += 1;
    }
    println!("{} {:?}", generation_count, last_population);

}
