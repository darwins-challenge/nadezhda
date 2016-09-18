//! A population of individuals
//!
//! The fitness of a population in an environment is tracked over multiple
//! generations.

use super::grammar::Program;

/// A Population is nothing more than a Vector of Programs
pub type Population = Vec<Program>;
