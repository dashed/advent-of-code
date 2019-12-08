// https://adventofcode.com/2019/day/5

struct Computer {
    current_instruction_pointer: usize,
    tape: Vec<i32>,
}

impl Computer {
    fn new(input_string: String) -> Self {
        let inputs: Vec<i32> = input_string
            .trim()
            .split(',')
            .map(|value| -> i32 {
                return value.parse().unwrap();
            })
            .collect();

        Computer {
            current_instruction_pointer: 0,
            tape: inputs,
        }
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    println!("{}", input_string);
}
