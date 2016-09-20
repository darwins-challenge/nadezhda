extern crate rand;
#[macro_use]
extern crate nadezhda;

use std::collections::HashSet;
use nadezhda::grammar::Program;
use nadezhda::darwin::crossover::crossover;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    let mut rng = rand::thread_rng();
    let left: Program = forward!(forward!(forward!(stop!())));
    let right: Program = backward!(backward!(backward!(stop!())));

    let mut count = 0;
    loop {
        let (left_child, right_child) = crossover(&mut rng, &left, &right);
        let left_finger_print = left_child.finger_print();
        let right_finger_print = right_child.finger_print();
        finger_prints.insert(left_finger_print);
        finger_prints.insert(right_finger_print);
        count += 1;

        if finger_prints.len() == 16 { break; }
    }

    let mut prints: Vec<&FingerPrint> = finger_prints.iter().collect();
    prints.sort();
    println!("{}", count);
    for print in prints {
        println!("{:?}", print);
    }
}
