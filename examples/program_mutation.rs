extern crate rand;
#[macro_use]
extern crate nadezhda;

use std::collections::HashSet;
use nadezhda::grammar::Program;
use nadezhda::darwin::mutate::{Mutatable, RandomDecision};

#[derive(Debug, PartialEq, Eq, Hash)]
struct FingerPrint {
    f: i32,
    b: i32,
}

impl FingerPrint {
    fn new() -> FingerPrint {
        FingerPrint {
            f: 0,
            b: 0,
        }
    }

    fn f(&self) -> FingerPrint {
        FingerPrint { f: self.f + 1, ..*self }
    }

    fn b(&self) -> FingerPrint {
        FingerPrint { b: self.b + 1, ..*self }
    }
}

trait FingerPrintable {
    fn finger_print(&self) -> FingerPrint;
}

impl FingerPrintable for Program {
    fn finger_print(&self) -> FingerPrint {
        match *self {
            Program::Forward(ref contained_program) => contained_program.finger_print().f(),
            Program::Backward(ref contained_program) => contained_program.finger_print().b(),
            Program::Stop => FingerPrint::new(),
        }
    }
}

fn main() {
    let mut finger_prints: HashSet<FingerPrint> = HashSet::new();
    let program: Program = forward!(forward!(forward!(stop!())));
    let mut random_decision: RandomDecision = RandomDecision::new(0.1);

    let mut count = 0;
    loop {
        let mutation = program.mutate(&mut random_decision);
        let finger_print = mutation.finger_print();
        finger_prints.insert(finger_print);
        count += 1;

        if finger_prints.len() == 4 { break; }
    }

    println!("{} {:?}", count, finger_prints);
}
