// https://adventofcode.com/2022/day/3
use std::collections::HashSet;

struct Rucksack {
    all: Vec<char>,
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
            all: chars,
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

fn get_priority(x: char) -> u64 {
    if x.is_ascii_lowercase() {
        return (x as u64) - 96;
    };

    (x as u64) - 38
}

fn get_sum_of_priorities_part_1(inputs: Vec<String>) -> u64 {
    inputs
        .iter()
        .map(|input| -> Rucksack { Rucksack::new(input.trim().to_string()) })
        .map(|x: Rucksack| -> u64 {
            let common_item: char = x.get_common_item();

            get_priority(common_item)
        })
        .sum()
}

fn get_sum_of_priorities_part_2(inputs: Vec<String>) -> u64 {
    let mut iter = inputs
        .iter()
        .map(|input| -> Rucksack { Rucksack::new(input.trim().to_string()) })
        .peekable();

    let mut sum_of_priorities = 0;
    loop {
        if iter.peek().is_none() {
            break;
        }
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();
        let third = iter.next().unwrap();

        let first: HashSet<&char> = HashSet::from_iter(first.all.iter());
        let second: HashSet<&char> = HashSet::from_iter(second.all.iter());
        let third: HashSet<&char> = HashSet::from_iter(third.all.iter());

        let first_and_second: HashSet<&char> = first.intersection(&second).copied().collect();

        let common_item: HashSet<&char> = first_and_second.intersection(&third).copied().collect();

        assert!(common_item.len() == 1);
        let common_item: char = *common_item.into_iter().next().unwrap();

        sum_of_priorities += get_priority(common_item);
    }

    sum_of_priorities
}

fn main() {
    let input_string = include_str!("input.txt");

    let inputs: Vec<String> = input_string
        .split_whitespace()
        .map(|x| -> String { x.to_string() })
        .collect();

    let sum_of_priorities = get_sum_of_priorities_part_1(inputs.clone());

    println!("Part 1: {}", sum_of_priorities);

    println!("Part 1: {}", get_sum_of_priorities_part_2(inputs));
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

        let sum_of_priorities = get_sum_of_priorities_part_1(inputs.clone());

        assert_eq!(sum_of_priorities, 157);

        assert_eq!(get_sum_of_priorities_part_2(inputs), 70);
    }
}
