// https://adventofcode.com/2018/day/1

// stdlib imports

use std::collections::HashSet;

// Part 1
fn sum_frequencies(mut current_frequency: i32, mut inputs: Vec<&str>) -> i32 {
    while let Some(raw_input) = inputs.pop() {
        let input = raw_input.trim();

        if input.is_empty() {
            // skip empty input
            continue;
        }

        let parsed_frequency: i32 = input.parse().unwrap();

        current_frequency += parsed_frequency;
    }

    return current_frequency;
}

// Part 2
fn find_second_frequency(
    mut current_frequency: i32,
    seen_frequencies: &mut HashSet<i32>,
    mut inputs: Vec<&str>,
) -> (bool, i32) {
    while let Some(raw_input) = inputs.pop() {
        let input = raw_input.trim();

        if input.is_empty() {
            // skip empty input
            continue;
        }

        let parsed_frequency: i32 = input.parse().unwrap();

        current_frequency += parsed_frequency;

        if seen_frequencies.contains(&current_frequency) {
            println!("Frequency first seen twice: {}", current_frequency);
            return (true, current_frequency);
        }

        seen_frequencies.insert(current_frequency);
    }

    return (false, current_frequency);
}

fn main() {
    let input_string = include_str!("input.txt");

    let inputs: Vec<&str> = {
        let mut inputs: Vec<&str> = input_string.split('\n').collect();
        inputs.reverse();
        inputs
    };

    // according to the puzzle, the starting frequency is 0
    let mut current_frequency = 0;

    current_frequency = sum_frequencies(current_frequency, inputs.clone());

    // part 1 of the Day 1 puzzle

    println!("Resulting frequency: {}", current_frequency);

    // part 2 of the Day 1 puzzle

    let mut seen_frequencies: HashSet<i32> = HashSet::new();
    current_frequency = 0;
    seen_frequencies.insert(current_frequency);

    loop {
        let (should_break, next_frequency) =
            find_second_frequency(current_frequency, &mut seen_frequencies, inputs.clone());

        current_frequency = next_frequency;

        if should_break {
            break;
        }
    }
}
