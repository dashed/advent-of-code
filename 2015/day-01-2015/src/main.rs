// https://adventofcode.com/2015/day/1

// code

fn main() {
    let input_string = include_str!("input.txt");

    // part 1

    let final_floor =
        input_string.chars().fold(
            0,
            |current_floor: i32, instruction: char| match instruction {
                '(' => {
                    return current_floor + 1;
                }
                ')' => {
                    return current_floor - 1;
                }
                _ => {
                    unreachable!();
                }
            },
        );

    println!("Part 1: {}", final_floor);

    // part 2

    let mut current_floor = 0;
    let mut pos_of_instruction = 0;
    for instruction in input_string.chars() {
        pos_of_instruction += 1;

        match instruction {
            '(' => {
                current_floor += 1;
            }
            ')' => {
                current_floor -= 1;
            }
            _ => {
                unreachable!();
            }
        }

        if current_floor < 0 {
            println!("Part 2: {}", pos_of_instruction);
            break;
        }
    }
}
