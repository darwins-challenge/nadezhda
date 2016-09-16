extern crate rand;
extern crate nadezhda;

use nadezhda::grammar::Program;

trait Length {
    fn length(&self) -> i32;
}

impl Length for Program {
    fn length(&self) -> i32 {
        1 + match *self {
            Program::Forward(ref contained_program) => {
                contained_program.length()
            },
            Program::Backward(ref contained_program) => {
                contained_program.length()
            },
            Program::Stop => 0

        }
    }
}

fn main() {
    let program: Program = rand::random();

    println!("length of program is {}", program.length());
}

