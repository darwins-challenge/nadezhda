//! Crossover two programs
//!
//! Crossover is a way to mix parent DNA into an offspring. The hope is that the
//! offspring inherits the good behavior, allowing it to be better suited to its
//! environment.

use rand;
use rand::distributions::{IndependentSample, Range};
use super::super::grammar::Program;

trait Length {
    fn length(&self) -> i32;
}

impl Length for Program {
    fn length(&self) -> i32 {
        match *self {
            Program::Forward(ref contained_program) => {
                1 + contained_program.length()
            },
            Program::Backward(ref contained_program) => {
                1 + contained_program.length()
            },
            Program::Stop => 1,
        }
    }
}

trait Twin {
    fn duplicate(&self) -> Self;
}

impl Twin for Program {
    fn duplicate(&self) -> Program {
        match *self {
            Program::Forward(ref contained_program) => {
                Program::Forward(Box::new(contained_program.duplicate()))
            },
            Program::Backward(ref contained_program) => {
                Program::Backward(Box::new(contained_program.duplicate()))
            },
            Program::Stop => Program::Stop,
        }
    }
}

trait Split {
    fn take(&self, cut_point: i32, tail: Self) -> Self;
    fn drop(&self, cut_point: i32) -> Self;
}

impl Split for Program {
    fn take(&self, cut_point: i32, tail: Program) -> Program {
        if cut_point > 0 {
            match *self {
                Program::Forward(ref contained_program) => {
                    Program::Forward(Box::new(contained_program.take(cut_point - 1, tail)))
                },
                Program::Backward(ref contained_program) => {
                    Program::Backward(Box::new(contained_program.take(cut_point - 1, tail)))
                },
                Program::Stop => tail.duplicate()
            }
        } else {
            tail.duplicate()
        }
    }

    fn drop(&self, cut_point: i32) -> Program {
        if cut_point > 0 {
            match *self {
                Program::Forward(ref contained_program) => {
                    contained_program.drop(cut_point - 1)
                },
                Program::Backward(ref contained_program) => {
                    contained_program.drop(cut_point - 1)
                },
                Program::Stop => Program::Stop,
            }
        } else {
           self.duplicate()
        }
    }
}

/// Create a child by crossing over two parents.
pub fn crossover(mut rng: &mut rand::Rng, left: &Program, right: &Program) -> Program {
    let left_range = Range::new(0, left.length());
    let right_range = Range::new(0, right.length());
    let left_cut_point = left_range.ind_sample(&mut rng);
    let right_cut_point = right_range.ind_sample(&mut rng);

    left.take(left_cut_point, right.drop(right_cut_point))
}

#[cfg(test)]
mod tests {
    use super::super::super::grammar::Program;
    use super::Split;

    #[test]
    fn take_zero_should_work() {
        let program: Program = Program::Forward(Box::new(Program::Forward(Box::new(Program::Stop))));
        let take: Program = program.take(0, Program::Stop);

        assert_eq!(take, Program::Stop);
    }

    #[test]
    fn take_one_should_work() {
        let program: Program = Program::Forward(Box::new(Program::Forward(Box::new(Program::Stop))));
        let take: Program = program.take(1, Program::Stop);

        assert_eq!(take, Program::Forward(Box::new(Program::Stop)));
    }

    #[test]
    fn drop_zero_should_duplicate_program() {
        let program: Program = Program::Forward(Box::new(Program::Forward(Box::new(Program::Stop))));
        let drop: Program = program.drop(0);

        assert_eq!(drop, Program::Forward(Box::new(Program::Forward(Box::new(Program::Stop)))));
    }

    #[test]
    fn drop_one_should_work() {
        let program: Program = Program::Forward(Box::new(Program::Forward(Box::new(Program::Stop))));
        let drop: Program = program.drop(1);

        assert_eq!(drop, Program::Forward(Box::new(Program::Stop)))
    }

    #[test]
    fn drop_two_should_work() {
        let program: Program = Program::Forward(Box::new(Program::Forward(Box::new(Program::Stop))));
        let drop: Program = program.drop(2);

        assert_eq!(drop, Program::Stop)
    }

    #[test]
    fn drop_more_should_work() {
        let program: Program = Program::Forward(Box::new(Program::Forward(Box::new(Program::Stop))));
        let drop: Program = program.drop(10);

        assert_eq!(drop, Program::Stop)
    }
}
