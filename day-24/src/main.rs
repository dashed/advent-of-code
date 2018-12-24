// https://adventofcode.com/2018/day/24

// imports

extern crate combine;
use combine::combinator::token;
use combine::parser::char::{char, digit, letter, spaces};
use combine::stream::easy;
use combine::{any, tokens};
use combine::{between, choice, many1, sep_by, Parser};

use core::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

// code

fn target_order(first_group: &Group, second_group: &Group) -> Ordering {
    // group with higher effective power is first
    if first_group.effective_power() != second_group.effective_power() {
        return first_group
            .effective_power()
            .cmp(&second_group.effective_power());
    }

    // in a tie, the group with the higher initiative is first.
    return first_group.initiative.cmp(&second_group.initiative);
}

#[derive(Debug, Eq, PartialEq)]
enum Race {
    Immunity,
    Infection,
}

#[derive(Debug, Eq, PartialEq)]
struct Group {
    race: Race,

    num_of_units: i32,
    hit_points: i32,

    attack_damage: i32,
    attack_type: String,
    initiative: i32,

    immunities: HashSet<String>,
    weaknesses: HashSet<String>,
}

impl Group {
    fn effective_power(&self) -> i32 {
        return self.attack_damage * self.num_of_units;
    }

    fn immune_to(&self, attack_type: &String) -> bool {
        return self.immunities.contains(attack_type);
    }

    fn weak_to(&self, attack_type: &String) -> bool {
        return self.weaknesses.contains(attack_type);
    }

    fn calculate_damage_to_group(&self, other_group: &Self) -> i32 {
        if other_group.immune_to(&self.attack_type) {
            return 0;
        }

        if other_group.weak_to(&self.attack_type) {
            return 2 * self.attack_damage;
        }

        return self.attack_damage;
    }

    // take damage from other_group
    fn take_damage(&mut self, other_group: &Self) {
        let damage_taken = other_group.calculate_damage_to_group(self);

        let num_of_units_dead: i32 = damage_taken / self.num_of_units;

        self.num_of_units = self.num_of_units - num_of_units_dead;
    }
}

impl Ord for Group {
    fn cmp(&self, other: &Self) -> Ordering {
        return target_order(self, other);
    }
}

impl PartialOrd for Group {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

struct Battle {
    groups: BinaryHeap<Group>,
}

fn get_attackable_target() {}

fn parse_input(input_string: &str) {
    let input_string = input_string.trim();

    let skip_spaces = spaces().silent();

    let constant = |needle: String| {
        let chars: Vec<char> = needle.chars().into_iter().collect();
        return tokens(|l, r| l.eq_ignore_ascii_case(&r), "error".into(), chars).map(move |_| {
            return needle.clone();
        });
    };

    let integer = many1(digit()).map(|string: String| -> i32 {
        return string.parse::<i32>().unwrap();
    });

    let immunity_start = (constant("Immune System:".to_string()), skip_spaces);

    // let infection_start = many1(any()).and_then(|word: String| {
    //     if word == "Infection:" {
    //         Ok(word)
    //     } else {
    //         Err(easy::Error::Expected(easy::Info::Borrowed("Infection:")))
    //     }
    // });

    let mut parser = (immunity_start).map(|(_)| {
        return ();
    });

    let result: Result<((), &str), easy::ParseError<&str>> = parser.easy_parse(input_string);

    match result {
        Ok((value, _remaining_input)) => {
            println!("{:?}", value);
            println!("{}", _remaining_input);
        }
        Err(err) => println!("{}", err),
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    parse_input(input_string);
}
