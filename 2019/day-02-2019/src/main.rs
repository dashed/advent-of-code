// https://adventofcode.com/2019/day/2

use rayon::prelude::*;

fn run_computer(input_string: String, noun: u32, verb: u32) -> u32 {
    let mut inputs: Vec<u32> = input_string
        .trim()
        .split(',')
        .map(|opcode| -> u32 { opcode.parse().unwrap() })
        .collect();

    inputs[1] = noun;
    inputs[2] = verb;

    let mut current_instruction_pointer: usize = 0;

    loop {
        let opcode = inputs[current_instruction_pointer];

        match opcode {
            1 => {
                // add opcode
                let pos_arg_1 = inputs[current_instruction_pointer + 1];
                let arg_1 = inputs[pos_arg_1 as usize];
                let pos_arg_2 = inputs[current_instruction_pointer + 2];
                let arg_2 = inputs[pos_arg_2 as usize];
                let position_result = inputs[current_instruction_pointer + 3];
                inputs[position_result as usize] = arg_1 + arg_2;
            }
            2 => {
                // multiply opcode
                let pos_arg_1 = inputs[current_instruction_pointer + 1];
                let arg_1 = inputs[pos_arg_1 as usize];
                let pos_arg_2 = inputs[current_instruction_pointer + 2];
                let arg_2 = inputs[pos_arg_2 as usize];
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

        current_instruction_pointer += 4;
    }

    inputs[0]
}

fn part_2(input_string: String) {
    let noun_range: Vec<u32> = (0..=99).collect();
    let verb_range: Vec<u32> = (0..=99).collect();

    let result: Option<(u32, u32)> = noun_range
        .into_par_iter()
        .map(|noun| -> Option<(u32, u32)> {
            let result = verb_range
                .par_iter()
                .map(|verb| -> Option<(u32, u32)> {
                    let result = run_computer(input_string.clone(), noun, *verb);

                    if result == 19690720 {
                        return Some((noun, *verb));
                    }

                    None
                })
                .find_first(|result: &Option<(u32, u32)>| result.is_some());

            if let Some(result) = result {
                return result;
            }

            None
        })
        .find_first(|result: &Option<(u32, u32)>| result.is_some())
        .unwrap();

    match result {
        Some((noun, verb)) => {
            println!("Part 2");
            println!("Noun: {} Verb: {}", noun, verb);
            println!("100 * noun + verb = {}", 100 * noun + verb);
        }
        None => {
            println!("Part 2. Unable to find the noun and verb combination.");
        }
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    // Part 1

    // restore the gravity assist program (your puzzle input) to the "1202 program alarm" state
    // it had just before the last computer caught fire. To do this, before running the program,
    // replace position 1 with the value 12 and replace position 2 with the value 2.
    println!("Part 1: {}", run_computer(input_string.to_string(), 12, 2));

    // Part 2

    part_2(input_string.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_computer() {
        assert_eq!(
            run_computer("1,9,10,3,2,3,11,0,99,30,40,50".to_string(), 9, 10),
            3500
        );
        assert_eq!(run_computer("1,0,0,0,99".to_string(), 0, 0), 2);
        assert_eq!(run_computer("2,4,4,5,99,0".to_string(), 4, 4), 2);
        assert_eq!(run_computer("1,1,1,4,99,5,6,0,99".to_string(), 1, 1), 30);
    }
}
