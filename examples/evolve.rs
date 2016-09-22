extern crate rand;
extern crate nadezhda;

use nadezhda::grammar::Program;
use nadezhda::environment::Environment;
use nadezhda::population::Population;
use nadezhda::evaluate::Evaluator;
use nadezhda::darwin::evolve::succesion;

fn survivors(population: Population, environment: Environment) -> Vec<Program> {
    let Population(generation) = population;
    let survivors: Vec<Program> = generation
        .iter()
        .map(|program| (program.clone(), program.evaluate_on(environment.clone())))
        .filter(|&(_, ref env)| env.cockroach_location - env.food_location == 0)
        .map(|(program, _)| program)
        .collect();

    survivors
}

fn main() {
    let mut rng = rand::thread_rng();
    let environment: Environment = Environment::new(5);
    let mut last_population: Population = Population::new(5);

    let mut generation_count = 0;
    while generation_count < 100 {
        let next_population = succesion(&mut rng, environment.clone(), last_population.clone());

        let survivors: Vec<Program> = survivors(next_population.clone(), environment.clone());
        last_population = next_population;

        if survivors.len() > 0 { break; }
        generation_count += 1;
    }

    let survivors: Vec<Program> = survivors(last_population.clone(), environment.clone());
    if survivors.len() > 0 {
        println!("{} {} {:?}", generation_count, survivors.len(), survivors[0]);
    } else {
        println!("{} {:?}", generation_count, last_population);
    }
}
