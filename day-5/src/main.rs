// https://adventofcode.com/2018/day/5

// imports

use std::collections::HashSet;
use std::iter::FromIterator;

// helpers

fn is_same_type(x: char, y: char) -> bool {
    return x.to_lowercase().to_string() == y.to_lowercase().to_string();
}

fn is_opposite_polarity(x: char, y: char) -> bool {
    return (x.is_uppercase() && y.is_lowercase()) || (y.is_uppercase() && x.is_lowercase());
}

fn does_react(x: char, y: char) -> bool {
    return is_same_type(x, y) && is_opposite_polarity(x, y);
}

fn part_1(input: &str) -> String {
    let mut units: Vec<char> = input.chars().collect();

    // skip first N units known to not react
    let mut skip_n = 0;

    'outer_loop: loop {
        let mut units_iterable = units.iter().enumerate().skip(skip_n).peekable();

        while let Some((current_index, current_unit)) = units_iterable.next() {
            if units_iterable.peek().is_none() {
                // no further reactions possible
                break 'outer_loop;
            }

            let (_next_index, next_unit) = units_iterable.peek().unwrap();

            if does_react(*current_unit, **next_unit) {
                // remove these items and start from the beginning
                units.drain(current_index..(current_index + 2));

                // Know that first skip_n do not react, so we start again from there.
                skip_n = if current_index == 0 {
                    current_index
                } else {
                    current_index - 1
                };

                break;
            }
        }
    }

    let final_result: String = units.into_iter().collect();
    return final_result;
}

fn part_2(input: &str) -> String {
    let unique_types: HashSet<char> = HashSet::from_iter(input.to_lowercase().chars().into_iter());

    let result = unique_types.iter().fold(
        input.to_string(),
        |shortest_string, character: &char| -> String {
            let units: String = input
                .chars()
                .into_iter()
                .filter(|x| -> bool { return !is_same_type(*x, *character) })
                .collect();

            let reacted = part_1(&units);

            if reacted.len() < shortest_string.len() {
                // found new shortest polymer produced
                return reacted;
            }

            return shortest_string;
        },
    );

    return result;
}

fn main() {
    let input_string = include_str!("input.txt");

    let final_result = part_1(input_string);

    println!("Part 1:");
    println!("Started with {} units.", input_string.len());
    println!(
        "How many units remain after fully reacting the polymer you scanned?: {}",
        final_result.len()
    );

    println!("Part 2:");
    let shortest_polymer = part_2(input_string);

    println!("shortest_polymer length: {:?}", shortest_polymer.len());
    // println!("shortest_polymer: {:?}", shortest_polymer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_same_type() {
        assert_eq!(is_same_type('a', 'a'), true);
        assert_eq!(is_same_type('a', 'A'), true);
        assert_eq!(is_same_type('a', 'b'), false);
    }

    #[test]
    fn test_is_opposite_polarity() {
        assert_eq!(is_opposite_polarity('a', 'a'), false);
        assert_eq!(is_opposite_polarity('B', 'B'), false);
        assert_eq!(is_opposite_polarity('a', 'A'), true);
        assert_eq!(is_opposite_polarity('A', 'a'), true);
        assert_eq!(is_opposite_polarity('a', 'B'), true);
    }

    #[test]
    fn test_does_react() {
        assert_eq!(does_react('a', 'a'), false);
        assert_eq!(does_react('A', 'A'), false);
        assert_eq!(does_react('a', 'A'), true);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("dabAcCaCBAcCcaDA"), "dabCBAcaDA".to_string());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("dabAcCaCBAcCcaDA"), "daDA".to_string());
    }
}
