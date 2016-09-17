//! simulate the behavior of the cockroach in a certain environment.
//!
//! The brain of the cockroach determines how the environment changes when the
//! cockroach is allowed to execute her desired behavior.
//!
//! # Examples
//! Below we see how a `Program` changes the cockroaches `Environment`.
//!
//! ```rust
//! #[macro_use]
//! extern crate nadezhda;
//!
//! use nadezhda::grammar::Program;
//! use nadezhda::environment::Environment;
//! use nadezhda::simulate::Executable;
//!
//! fn main() {
//!     let program: Program = forward!(forward!(forward!(stop!())));
//!     let start: Environment = Environment::new(5);
//!
//!     let finish: Environment = program.execute_on(start);
//!
//!     assert_eq!(finish, Environment {
//!         cockroach_location: 3,
//!         food_location: 5
//!     });
//! }
//! ```

use super::environment::Environment;
use super::grammar::Program;

/// The contract how a cockroach can change an environment
pub trait Executable {
    /// An execution is performed on ab `Environment`. It will return the
    /// `Environment` which result in executing this `Executable`.
    fn execute_on(&self, environment: Environment) -> Environment;
}

impl Executable for Program {
    fn execute_on(&self, environment: Environment) -> Environment {
        match *self {
            Program::Forward(ref contained_program) => {
                let next_environment =
                Environment { cockroach_location : environment.cockroach_location + 1,
                  .. environment
                };
                contained_program.execute_on(next_environment)
            },
            Program::Backward(ref contained_program) => {
                let next_environment =
                Environment { cockroach_location : environment.cockroach_location - 1,
                  .. environment
                };
                contained_program.execute_on(next_environment)
            },
            Program::Stop => {
                Environment { .. environment }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::grammar::Program;
    use super::super::environment::Environment;
    use super::*;

    #[test]
    fn stop_correctly_changes_environment() {
        let program: Program = Program::Stop;
        let environment: Environment = Environment::new(5);

        let result: Environment = program.execute_on(environment);

        assert_eq!(result, Environment {
            cockroach_location: 0,
            food_location: 5,
        });
    }

    #[test]
    fn forward_correctly_changes_environment() {
        let program: Program = Program::Forward(Box::new(Program::Stop));
        let environment: Environment = Environment::new(5);

        let result: Environment = program.execute_on(environment);

        assert_eq!(result, Environment {
            cockroach_location: 1,
            food_location: 5,
        });
    }

    #[test]
    fn backward_correctly_changes_environment() {
        let program: Program = Program::Backward(Box::new(Program::Stop));
        let environment: Environment = Environment::new(5);

        let result: Environment = program.execute_on(environment);

        assert_eq!(result, Environment {
            cockroach_location: -1,
            food_location: 5,
        });
    }
}
