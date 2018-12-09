// https://adventofcode.com/2018/day/1

fn main() {
    let input_string = include_str!("input.txt");

    let mut inputs: Vec<&str> = input_string.split('\n').collect();

    // according to the puzzle, the starting frequency is 0
    let mut current_frequency = 0;

    while let Some(raw_input) = inputs.pop() {
        let input = raw_input.trim();

        if input.is_empty() {
            // skip empty input
            continue;
        }

        let parsed_frequency: i32 = input.parse().unwrap();

        current_frequency += parsed_frequency;
    }

    println!("Resulting frequency: {}", current_frequency);
}
