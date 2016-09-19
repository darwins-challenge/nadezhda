//! Mutate a program
//!
//! Mutation is a source of change in a population. Sometimes mutations are
//! beneficial and improve the overall fitness.

use rand;
use rand::distributions::{IndependentSample, Range};
use super::super::grammar::Program;

/// Determines if a mutation should occur
pub trait MutateDecision {
    /// Returns true when a mutation should occur, false otherwise
    fn should_mutate(&mut self) -> bool;
}

/// A MutateDecision that decides randomly
pub struct RandomDecision {
    rng: Box<rand::Rng>,
    range: Range<f32>,
    threshold: f32,
}

impl RandomDecision {
    /// Creates a RandomDecision with a fixed threshold
    pub fn new(threshold: f32) -> RandomDecision {
        RandomDecision {
            rng: Box::new(rand::thread_rng()),
            range: Range::new(0f32, 1.0),
            threshold: threshold,
        }
    }
}

impl MutateDecision for RandomDecision {
    fn should_mutate(&mut self) -> bool {
        self.range.ind_sample(&mut self.rng) < self.threshold
    }
}

/// Somethings can mutate
pub trait Mutatable {
    /// Returns a mutated variant of self
    fn mutate(&self, decide: Box<MutateDecision>) -> Self;
}

impl Mutatable for Program {
    fn mutate(&self, mut decide: Box<MutateDecision>) -> Program {
        match *self {
            Program::Forward(ref contained_program) => {
                if decide.should_mutate() {
                    Program::Backward(Box::new(contained_program.mutate(decide)))
                } else {
                    Program::Forward(Box::new(contained_program.mutate(decide)))
                }
            },
            Program::Backward(ref contained_program) => {
                if decide.should_mutate() {
                    Program::Forward(Box::new(contained_program.mutate(decide)))
                } else {
                    Program::Backward(Box::new(contained_program.mutate(decide)))
                }
            },
            Program::Stop => Program::Stop,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::grammar::Program;
    use super::*;

    enum TestDecision {
        AlwaysMutate,
        NeverMutate,
    }

    impl MutateDecision for TestDecision {
        fn should_mutate(&mut self) -> bool {
            match *self {
                TestDecision::AlwaysMutate => true,
                TestDecision::NeverMutate => false,
            }
        }
    }

    #[test]
    fn should_never_mutate() {
        let program: Program = Program::Forward(Box::new(Program::Stop));
        let decision: TestDecision = TestDecision::NeverMutate;

        let mutation = program.mutate(Box::new(decision));

        assert_eq!(mutation, Program::Forward(Box::new(Program::Stop)));

    }

    #[test]
    fn should_always_mutate() {
        let program: Program = Program::Forward(Box::new(Program::Stop));
        let decision: TestDecision = TestDecision::AlwaysMutate;

        let mutation = program.mutate(Box::new(decision));

        assert_eq!(mutation, Program::Backward(Box::new(Program::Stop)));

    }
}
