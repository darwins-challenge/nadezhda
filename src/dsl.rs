//! A Domain Specific Language (DSL) for Abstract Syntax Trees (AST).
//!
//! A [DSL](https://en.wikipedia.org/wiki/Domain-specific_language) is a
//!
//! > is a computer language specialized to a particular application domain.
//! > This is in contrast to a general-purpose language (GPL), which is broadly
//! > applicable across domains.
//!
//! The domain we are interested in here are AST for our grammar.

#[macro_export]
macro_rules! forward {
    ($program: expr) => (nadezhda::grammar::Program::Forward(Box::new($program)))
}

#[macro_export]
macro_rules! backward {
    ($program: expr) => (nadezhda::grammar::Program::Backward(Box::new($program)))
}

#[macro_export]
macro_rules! stop {
    () => (nadezhda::grammar::Program::Stop)
}
