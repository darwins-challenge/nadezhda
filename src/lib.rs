#![deny(missing_docs)]
//! Nadezhda is a project to model a robot space cockroach.
//!
//! > The cockroach moves along a line. Each turn it either takes a step
//! > forward, takes a step backwards, or stops.
//!
//! See [fly me to the moon](https://leanpub.com/flymetothemoon) for a extended
//! description.

extern crate rand;

pub mod grammar;
pub mod dsl;
pub mod random;
