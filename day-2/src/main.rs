// https://adventofcode.com/2018/day/2

// stdlib imports

use std::collections::HashMap;

// types

enum Parsed {
    // input string only contain letter(s) that appear exactly two times
    HasTwo,
    // input string only contain letter(s) that appear exactly three times
    HasThree,
    // input string only contain letter(s) that either appear exactly two times,
    // or appear exactly thre times
    HasBoth,
    // input string does not satisfy the above conditions
    None,
}

fn parse_input(input: &str) {
    println!("{:?}", input);

    let letter_counter = input.chars().fold(
        HashMap::new(),
        |mut letter_counter: HashMap<char, i32>, letter| {
            // count the number of occurrences of the letters within the given input

            let result = letter_counter.get(&letter);

            match result {
                None => {
                    letter_counter.insert(letter, 1);
                }
                Some(num_of_occurrences) => {
                    letter_counter.insert(letter, num_of_occurrences + 1);
                }
            }

            return letter_counter;
        },
    );

    println!("{:?}", letter_counter);
}

fn main() {
    let input_string = include_str!("input.txt");

    let mut inputs: Vec<&str> = input_string.split('\n').collect();

    // inputs.into_iter().map(parse_input);

    let raw_input = inputs.pop().unwrap();

    parse_input(raw_input);

    // println!("{}", raw_input);

    // while let Some(raw_input) = inputs.pop() {
    //     let input = raw_input.trim();

    //     if input.is_empty() {
    //         // skip empty input
    //         continue;
    //     }

    // }

    // let mut num_of_ids_containing

    // let mut inputs: Vec<&str> = input_string.split('\n').collect();
    // println!("{}", input_string);
}
