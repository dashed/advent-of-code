// https://adventofcode.com/2019/day/2

fn main() {
    let input_string = include_str!("input.txt");

    let mut inputs: Vec<u32> = input_string
        .trim()
        .split(',')
        .map(|opcode| -> u32 {
            return opcode.parse().unwrap();
        })
        .collect();

    // restore the gravity assist program (your puzzle input) to the "1202 program alarm" state
    // it had just before the last computer caught fire. To do this, before running the program,
    // replace position 1 with the value 12 and replace position 2 with the value 2.
    inputs[1] = 12;
    inputs[2] = 2;

    // Part 1

    let mut current_instruction_pointer: usize = 0;

    loop {
        let opcode = inputs[current_instruction_pointer];

        match opcode {
            1 => {
                // add opcode
                let arg_1 = inputs[current_instruction_pointer + 1];
                let arg_2 = inputs[current_instruction_pointer + 2];
                let position_result = inputs[current_instruction_pointer + 3];
                inputs[position_result as usize] = arg_1 + arg_2;
            }
            2 => {
                // multiply opcode
                let arg_1 = inputs[current_instruction_pointer + 1];
                let arg_2 = inputs[current_instruction_pointer + 2];
                let position_result = inputs[current_instruction_pointer + 3];
                inputs[position_result as usize] = arg_1 * arg_2;
            }
            99 => {
                // halt
                break;
            }
            _ => {
                panic!(
                    "Unknown opcode at pos {}: {}",
                    current_instruction_pointer, opcode
                );
            }
        }

        current_instruction_pointer = current_instruction_pointer + 4;
    }

    // not 116

    println!("Part 1: {}", inputs[0]);
}
