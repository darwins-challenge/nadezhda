//! A population of individuals
//!
//! The fitness of a population in an environment is tracked over multiple
//! generations.
//!
//! # Examples
//! Below we show how to generate a initial population, and access the
//! underlying individuals.
//!
//! ```rust
//! extern crate rand;
//! extern crate nadezhda;
//!
//! use nadezhda::grammar::Program;
//! use nadezhda::population::Population;
//!
//! fn main() {
//!     let expected_length = 100;
//!     let population: Population = Population::new(expected_length);
//!     let Population(individuals) = population;
//!
//!     assert_eq!(individuals.len(), expected_length as usize);
//! }
//! ```

use rand;
use super::grammar::Program;

/// A Population is nothing more than a Vector of Programs
#[derive(Debug, Clone)]
pub struct Population(pub Vec<Program>);

impl Population {
    /// Create a population of a given size
    pub fn new(size: i32) -> Population {
        let mut population: Vec<Program> = vec!();

        for _ in 0..size {
            let program: Program = rand::random();

            population.push(program);
        }

        Population(population)
    }
}
