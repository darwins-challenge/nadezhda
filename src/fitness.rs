//! Determine how well a cockroach has performed in a environment
//!
//! A score can be given to a robot cockroach brain program.
//!
//! # Examples
//! To give an example of how to use a scorecard
//!
//! ```rust
//! #[macro_use]
//! extern crate nadezhda;
//!
//! use nadezhda::grammar::Program;
//! use nadezhda::environment::Environment;
//! use nadezhda::fitness::{Config, Score, ScoreCard};
//!
//! fn main() {
//!     let program = forward!(forward!(stop!()));
//!     let environment = Environment::new(3);
//!     let config = Config {
//!         program_length_weight: 1,
//!         food_distance_weight: 1,
//!     };
//!
//!     let score = config.score(program, environment);
//!
//!     assert_eq!(score, 4 as Score);
//! }
//! ```

use super::grammar::Program;
use super::environment::Environment;
use super::evaluate::Evaluator;

/// A score is just a number
pub type Score = i32;

/// A scorecard can keep score a program on an environment
pub trait ScoreCard {
    /// return a score
    fn score(&self, program: Program, environment: Environment) -> Score;
}

trait Length {
    fn length(&self) -> i32;
}

impl Length for Program {
    fn length(&self) -> i32 {
        1 + match *self {
            Program::Forward(ref contained_program) => {
                contained_program.length()
            },
            Program::Backward(ref contained_program) => {
                contained_program.length()
            },
            Program::Stop => 0
        }
    }
}

/// Configuration for a default implementation for a ScoreCard
pub struct Config {
    /// The weight with which the program length adds to the score
    pub program_length_weight: i32,
    /// The weight with which the distance to the source of food adds to the score
    pub food_distance_weight: i32,
}

impl ScoreCard for Config {
    fn score (&self, program: Program, environment: Environment) -> Score {
        let program_length = program.length();

        let final_environment = program.evaluate_on(environment);
        let food_distance = (final_environment.cockroach_location - final_environment.food_location).abs();

        self.program_length_weight * program_length + self.food_distance_weight * food_distance
    }
}

#[cfg(test)]
mod test {
    use super::super::grammar::Program;
    use super::super::environment::Environment;
    use super::*;

    #[test]
    fn should_determine_score() {
        let program = Program::Stop;
        let environment = Environment::new(5);
        let config = Config {
            program_length_weight: 1,
            food_distance_weight: 1,
        };

        let score = config.score(program, environment);

        assert_eq!(score, 6 as Score);
    }
}
