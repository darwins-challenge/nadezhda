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

/// A `Program` is either
///
/// 1. `Forward` followed by a `Program`
/// 2. `Backward` followed by a `Program`
/// 3. nothing, this is signified by `Stop`
pub enum Program {
    /// forward alternative of the grammar
    Forward(Box<Program>),
    /// backward alternative of the grammar
    Backward(Box<Program>),
    /// empty alternative of the grammar
    Stop,
}
