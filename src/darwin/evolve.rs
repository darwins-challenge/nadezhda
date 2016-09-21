//! Evolve a Population of Individuals in an Environment
//!
//! Evolution is the culmination of individuals in a population competing in a
//! harsh environment for limited resources. Here the environment is determined
//! by the fitness function, the individuals are programs.

use super::super::grammar::Program;
use super::super::population::Population;
use super::super::environment::Environment;
use super::super::fitness::{Config, ScoreCard, Score};

/// Return the next generation of the population
pub fn succesion(environment: Environment, population: Population) -> Population {
    let config: Config = Config {
        program_length_weight: -1,
        food_distance_weight: -1,
    };
    let Population(last_generation) = population;
    let mut scored_last_generation: Vec<(Program, Score)> = last_generation
        .iter()
        .map(|program| (program.clone(), config.score(program.clone(), environment.clone())))
        .collect();
    scored_last_generation.sort_by(|a,b| a.1.cmp(&b.1));
    let next_generation = scored_last_generation
        .iter()
        .map(|a| a.0.clone())
        .collect();

    Population(next_generation)
}
