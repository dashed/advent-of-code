// https://adventofcode.com/2018/day/24

// imports

extern crate combine;
use combine::combinator::token;
use combine::parser::char::{char, digit, letter, spaces};
use combine::stream::easy;
use combine::{between, choice, many1, optional, sep_by, sep_by1, tokens, Parser};

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

#[derive(Debug, Eq, PartialEq, Clone)]
enum Race {
    Immunity,
    Infection,
}

#[derive(Debug, Clone)]
enum Trait {
    Weaknesses(HashSet<String>),
    Immunities(HashSet<String>),
}

impl Trait {
    fn unwrap(self) -> HashSet<String> {
        match self {
            Trait::Weaknesses(set) => set,
            Trait::Immunities(set) => set,
        }
    }
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

#[derive(Debug)]
struct Battle {
    groups: BinaryHeap<Group>,
}

fn parse_input(input_string: &str) {
    let input_string = input_string.trim();

    let skip_spaces = || spaces().silent();

    let constant = |needle: String| {
        let chars: Vec<char> = needle.chars().into_iter().collect();
        return tokens(|l, r| l.eq_ignore_ascii_case(&r), "error".into(), chars).map(move |_| {
            return needle.clone();
        });
    };

    let integer = || {
        many1(digit()).map(|string: String| -> i32 {
            return string.parse::<i32>().unwrap();
        })
    };

    let immunity_start = (constant("Immune System:".to_string()), skip_spaces());
    let infection_start = (constant("Infection:".to_string()), skip_spaces());

    let list_of_words = || {
        sep_by::<HashSet<String>, _, _>(many1(letter()), spaces().skip(char(',')).skip(spaces()))
    };

    let parse_immunities = (
        constant("immune to".to_string()).with(skip_spaces()),
        list_of_words(),
    )
        .map(|(_, words)| {
            return Trait::Immunities(words);
        });

    let parse_weaknesses = (
        constant("weak to".to_string()).with(skip_spaces()),
        list_of_words(),
    )
        .map(|(_, words)| {
            return Trait::Weaknesses(words);
        });

    let traits_list = sep_by::<Vec<Trait>, _, _>(
        choice((parse_immunities, parse_weaknesses)),
        spaces().skip(char(';')).skip(spaces()),
    );

    let parse_traits_group = between(token('('), token(')'), traits_list);

    let parse_group = |race: Race| {
        (
            integer(), // num of units
            skip_spaces()
                .with(constant("units each with".to_string()))
                .with(skip_spaces()),
            integer(), // hit points
            skip_spaces()
                .with(constant("hit points".to_string()))
                .with(skip_spaces()),
            optional(parse_traits_group),
            skip_spaces()
                .with(constant("with an attack that does".to_string()))
                .with(skip_spaces()),
            integer(), /* attack damage */
            skip_spaces(),
            many1::<String, _>(letter()), /* attack type */
            skip_spaces()
                .with(constant("damage at initiative".to_string()))
                .with(skip_spaces()),
            integer(), /* initiative */
            skip_spaces(),
        )
            .map(
                move |(
                    num_of_units,
                    _,
                    hit_points,
                    _,
                    traits,
                    _,
                    attack_damage,
                    _,
                    attack_type,
                    _,
                    initiative,
                    _,
                )|
                      -> Group {
                    let (immunities, weaknesses) = traits
                        .map(|traits| {
                            let immunities = traits
                                .iter()
                                .find(|item| match item {
                                    Trait::Immunities(_) => {
                                        return true;
                                    }
                                    _ => false,
                                })
                                .map(|x| {
                                    return (*x).clone().unwrap();
                                })
                                .unwrap_or(HashSet::new());

                            let weaknesses = traits
                                .iter()
                                .find(|item| match item {
                                    Trait::Weaknesses(_) => {
                                        return true;
                                    }
                                    _ => false,
                                })
                                .map(|x| {
                                    return (*x).clone().unwrap();
                                })
                                .unwrap_or(HashSet::new());

                            return (immunities, weaknesses);
                        })
                        .unwrap_or((HashSet::new(), HashSet::new()));

                    return Group {
                        race: race.clone(),

                        num_of_units,
                        hit_points,

                        attack_damage,
                        attack_type,
                        initiative,

                        immunities,
                        weaknesses,
                    };
                },
            )
    };

    let mut parser = (
        immunity_start,
        many1::<Vec<Group>, _>(parse_group.clone()(Race::Immunity)),
        skip_spaces(),
        infection_start,
        many1::<Vec<Group>, _>(parse_group(Race::Infection)),
    )
        .map(|(_, immunities, _, _, infections)| {
            let mut groups = BinaryHeap::new();

            groups.extend(immunities);
            groups.extend(infections);
            return Battle { groups };
        });

    let result: Result<(Battle, &str), easy::ParseError<&str>> = parser.easy_parse(input_string);

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
