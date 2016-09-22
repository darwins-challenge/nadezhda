//! Evolve a Population of Individuals in an Environment
//!
//! Evolution is the culmination of individuals in a population competing in a
//! harsh environment for limited resources. Here the environment is determined
//! by the fitness function, the individuals are programs.

use rand;
use rand::distributions::{Range, IndependentSample};
use super::super::grammar::Program;
use super::super::population::Population;
use super::super::environment::Environment;
use super::super::fitness::{Config, ScoreCard, Score};
use super::crossover::crossover;

/// Return the next generation of the population
pub fn succesion(mut rng: &mut rand::Rng, environment: Environment, population: Population) -> Population {
    let config: Config = Config {
        program_length_weight: -1,
        food_distance_weight: -1,
    };
    let Population(last_generation) = population;
    let mut scored_last_generation: Vec<(Program, Score)> = last_generation
        .iter()
        .map(|program| (program.clone(), config.score(program.clone(), environment.clone())))
        .collect();
    scored_last_generation.sort_by(|a,b| b.1.cmp(&a.1));

    let mut generation: Vec<Program> = vec!();
    for index in 0..2 {
        let program: Program = scored_last_generation[index].0.clone();
        generation.push(program);
    }
    let index_range = Range::new(0, scored_last_generation.len());
    while generation.len() < last_generation.len() {
        let left_index = index_range.ind_sample(&mut rng);
        let right_index = index_range.ind_sample(&mut rng);
        let left = scored_last_generation[left_index].0.clone();
        let right = scored_last_generation[right_index].0.clone();
        let (left_child, right_child) = crossover(&mut rng, &left, &right);
        generation.push(left_child);
        if generation.len() < last_generation.len() {
            generation.push(right_child);
        }
    }

    Population(generation)
}
