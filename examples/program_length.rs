extern crate rand;
extern crate nadezhda;

use std::collections::HashMap;
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
    let mut length_frequency = HashMap::new();
    let number_of_loops = 10_000;
    for _ in 0..number_of_loops {
        let program: Program = rand::random();
        let length = program.length();

        let count = match length_frequency.get(&length) {
            Some(last_value) => last_value + 1,
            None => 1
        };
        length_frequency.insert(length, count);
    }

    let mut lengths: Vec<&i32> = length_frequency.keys().collect();
    lengths.sort();
    let mut weighted_sum = 0;
    for length in lengths {
        let frequency = length_frequency.get(length).unwrap();
        weighted_sum += length * frequency;
        println!("{}: {}", length, frequency );
    }
    println!("averge {}", (weighted_sum as f32) / (number_of_loops as f32));
}

