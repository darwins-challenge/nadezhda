extern crate rand;
extern crate nadezhda;

use std::env;
use std::process;

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
    if env::args().len() < 3 {
        println!("Usage: evolve FOOD POPULATION");
        process::exit(1);
    }

    let food_string: String = env::args().nth(1).unwrap();
    let food: i32 = match i32::from_str_radix(&food_string, 10) {
        Ok(value) => value,
        Err(_) => {
            println!("FOOD argument is not an integer");
            process::exit(2);
        }
    };

    let population_count_string: String = env::args().nth(2).unwrap();
    let population_count: i32 = match i32::from_str_radix(&population_count_string, 10) {
        Ok(value) => value,
        Err(_) => {
            println!("POPULATION argument is not an integer");
            process::exit(3);
        }
    };

    let mut rng = rand::thread_rng();
    let environment: Environment = Environment::new(food);
    let mut last_population: Population = Population::new(population_count);

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
