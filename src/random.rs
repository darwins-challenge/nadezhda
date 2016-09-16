//! Create random Abstract Syntax Trees (AST) for Nadezhda grammar.
//!
//! `random::Rand` is implemented for all important structures. This allows to
//! randomly generate a `Program`.
//!
//! # Examples
//!
//! ```rust
//! extern crate rand;
//! extern crate nadezhda;
//!
//! use rand::Rng;
//!
//! use nadezhda::grammar::Program;
//!
//! fn main() {
//!     let generated: Program = rand::random();
//!     println!("{:?}", generated);
//! }
//! ```

use rand;
use super::grammar::Program;

impl rand::Rand for Program {
    fn rand<R: rand::Rng>(rng: &mut R) -> Self {
        let random_number = rng.gen_range(0, 3);
        match random_number {
            0 => {
                let nested_program: Program = Program::rand(rng);
                Program::Forward(Box::new(nested_program))
            },
            1 => {
                let nested_program: Program = Program::rand(rng);
                Program::Backward(Box::new(nested_program))
            },
            _ => Program::Stop,
        }
    }
}
