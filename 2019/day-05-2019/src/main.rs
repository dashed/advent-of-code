// https://adventofcode.com/2019/day/5
#![allow(dead_code)]

enum ParameterMode {
    // position mode, which causes the parameter to be interpreted as a position
    Position,
    // In immediate mode, a parameter is interpreted as a value
    Immediate,
}

struct Computer {
    current_instruction_pointer: usize,
    tape: Vec<i32>,
    mode: ParameterMode,
}

impl Computer {
    fn new(input_string: String) -> Self {
        let inputs: Vec<i32> = input_string
            .trim()
            .split(',')
            .map(|value| -> i32 { value.parse().unwrap() })
            .collect();

        Computer {
            current_instruction_pointer: 0,
            tape: inputs,
            mode: ParameterMode::Position,
        }
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    println!("{}", input_string);
}
