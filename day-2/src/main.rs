// https://adventofcode.com/2018/day/2

// stdlib imports

use std::collections::HashMap;

// types

#[derive(Debug)]
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

fn parse_input(input: &str) -> Parsed {
    let letter_counter = input.chars().fold(
        HashMap::new(), // accumulator
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

    // from letter_counter, determine if input contains letters that either
    // - occur exactly two times
    // - occur exactly three times
    // - or both

    let result = letter_counter.into_iter().fold(
        Parsed::None, // accumulator
        |current_state, (_letter, num_of_occurrences)| {
            let has_two = num_of_occurrences == 2;
            let has_three = num_of_occurrences == 3;

            if !has_two && !has_three {
                return current_state;
            }

            match current_state {
                Parsed::None => {
                    if has_two {
                        return Parsed::HasTwo;
                    }

                    return Parsed::HasThree;
                }
                Parsed::HasTwo => {
                    if has_two {
                        return Parsed::HasTwo;
                    }

                    return Parsed::HasBoth;
                }
                Parsed::HasThree => {
                    if has_three {
                        return Parsed::HasThree;
                    }

                    return Parsed::HasBoth;
                }
                Parsed::HasBoth => {
                    return Parsed::HasBoth;
                }
            }
        },
    );

    return result;
}

fn part_1(inputs: Vec<&str>) {
    let (num_of_two, num_of_three) = inputs.into_iter().map(parse_input).fold(
        (
            // number of times inputs containing 2 letters that occur at least once
            0, // number of times inputs containing 3 letters that occur at least once
            0,
        ),
        |accumulator, parsed_state: Parsed| {
            let (num_of_two, num_of_three) = accumulator;

            match parsed_state {
                Parsed::None => {
                    return (num_of_two, num_of_three);
                }
                Parsed::HasBoth => {
                    return (num_of_two + 1, num_of_three + 1);
                }
                Parsed::HasTwo => {
                    return (num_of_two + 1, num_of_three);
                }
                Parsed::HasThree => {
                    return (num_of_two, num_of_three + 1);
                }
            }
        },
    );

    println!("Part 1:");
    println!("num_of_two: {}", num_of_two);
    println!("num_of_three: {}", num_of_three);
    let checksum = num_of_two * num_of_three;
    println!("checksum: {} * {} = {}", num_of_two, num_of_three, checksum);
}

fn part_2(inputs: Vec<&str>) {

    println!("Part 2:");
}

fn main() {
    let input_string = include_str!("input.txt");

    let inputs: Vec<&str> = {
        let mut inputs: Vec<&str> = input_string.split('\n').collect();
        inputs.reverse();
        inputs
    };

    part_1(inputs.clone());

    part_2(inputs.clone());
}
