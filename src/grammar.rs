//! Nadezhda's brain control program grammar
//!
//! The brain of our space cockroach is controlled by a program. The allowed
//! programs are described by the following grammar.
//!
//! ```plain
//! S -> forward S
//!    | backward S
//!    | ''
//! ```
//!
//! # Examples
//! We  show how to create a `Program` by hand
//!
//! ```rust
//! extern crate nadezhda;
//!
//! use nadezhda::grammar::Program;
//!
//! fn main() {
//!     let program: Program = Program::Forward(
//!       Box::new(Program::Forward(
//!         Box::new(Program::Stop)
//!       ))
//!     );
//!
//!     let message = match program {
//!       Program::Forward(_)  => "right",
//!       Program::Backward(_) => "wrong",
//!       Program::Stop        => "not even close",
//!     };
//!
//!     assert_eq!(message, "right");
//! }
//! ```

/// A `Program` for the cockroach brain
///
/// It is either
/// 1. `Forward` followed by a `Program
/// 2. `Backward` followed by a `Program`
/// 3. nothing, this is signified by `Stop`
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Program {
    /// forward alternative of the grammar
    Forward(Box<Program>),
    /// backward alternative of the grammar
    Backward(Box<Program>),
    /// empty alternative of the grammar
    Stop,
}
