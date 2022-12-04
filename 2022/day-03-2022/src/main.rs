// https://adventofcode.com/2022/day/3
use std::collections::HashSet;

struct Rucksack {
    first_compartment: Vec<char>,
    second_compartment: Vec<char>,
}

impl Rucksack {
    fn new(input: String) -> Self {
        assert!(input.len() % 2 == 0);
        let input_len = input.len();
        let chars: Vec<char> = input.chars().collect();

        let first_compartment: Vec<char> = chars.iter().take(input_len / 2).copied().collect();

        let second_compartment: Vec<char> = chars.iter().skip(input_len / 2).copied().collect();

        {
            let first_string: String = first_compartment.iter().collect();
            let second_string: String = second_compartment.iter().collect();
            assert!(input == format!("{}{}", first_string, second_string));
        };

        Rucksack {
            first_compartment,
            second_compartment,
        }
    }

    fn get_common_item(&self) -> char {
        let first: HashSet<&char> = HashSet::from_iter(self.first_compartment.iter());
        let second: HashSet<&char> = HashSet::from_iter(self.second_compartment.iter());
        let common: HashSet<_> = first.intersection(&second).collect();
        assert!(common.len() == 1);
        **common.into_iter().next().unwrap()
    }
}

fn get_sum_of_priorities(inputs: Vec<String>) -> u64 {
    inputs
        .iter()
        .map(|input| -> Rucksack { Rucksack::new(input.trim().to_string()) })
        .map(|x: Rucksack| -> u64 {
            let common_item: char = x.get_common_item();

            if common_item.is_ascii_lowercase() {
                (common_item as u64) - 96
            } else {
                (common_item as u64) - 38
            }
        })
        .sum()
}

fn main() {
    let input_string = include_str!("input.txt");

    let inputs: Vec<String> = input_string
        .split_whitespace()
        .map(|x| -> String { x.to_string() })
        .collect();

    let sum_of_priorities = get_sum_of_priorities(inputs);

    println!("Part 1: {}", sum_of_priorities);
    // guesses:
    // 28059
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input_string = r###"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
    "###
        .trim();

        let inputs: Vec<String> = input_string
            .split_whitespace()
            .map(|x| -> String { x.to_string() })
            .collect();

        let sum_of_priorities = get_sum_of_priorities(inputs);

        assert_eq!(sum_of_priorities, 157);
    }
}
