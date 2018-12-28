// https://adventofcode.com/2018/day/24

// imports

extern crate combine;

use combine::combinator::token;
use combine::parser::char::{char, digit, letter, spaces};
use combine::stream::easy;
use combine::{between, choice, many1, optional, sep_by, sep_by1, tokens, Parser};

use core::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

// code

fn parse_input(input_string: &str) -> Battle {
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
        sep_by1::<HashSet<String>, _, _>(many1(letter()), spaces().skip(char(',')).skip(spaces()))
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
                        id: 0,

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
            return Battle::new(groups);
        });

    let result: Result<(Battle, &str), easy::ParseError<&str>> = parser.easy_parse(input_string);

    match result {
        Ok((value, remaining_input)) => {
            assert!(remaining_input.trim().len() <= 0);
            return value;
        }
        Err(err) => {
            panic!("{}", err);
        }
    }
}

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

type GroupID = i32;
type Damage = i32;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Group {
    id: GroupID,

    race: Race,

    num_of_units: i32,
    hit_points: i32,

    attack_damage: Damage,
    attack_type: String,
    initiative: i32,

    immunities: HashSet<String>,
    weaknesses: HashSet<String>,
}

impl Group {
    #[allow(dead_code)]
    fn to_string(&self) -> String {
        format!(
            "Group {} ({:?}) with {} units (HP: {})",
            self.id, self.race, self.num_of_units, self.hit_points
        )
    }

    fn effective_power(&self) -> i32 {
        return self.attack_damage * self.num_of_units;
    }

    fn immune_to(&self, attack_type: &String) -> bool {
        return self.immunities.contains(attack_type);
    }

    fn weak_to(&self, attack_type: &String) -> bool {
        return self.weaknesses.contains(attack_type);
    }

    fn calculate_damage_to_group(&self, other_group: &Self) -> Damage {
        if other_group.immune_to(&self.attack_type) {
            return 0;
        }

        if other_group.weak_to(&self.attack_type) {
            return 2 * self.effective_power();
        }

        return self.effective_power();
    }

    fn take_damage(&mut self, other_group: &Self) {
        let damage_taken = other_group.calculate_damage_to_group(&self);

        let mut num_of_units_dead: i32 = damage_taken / self.hit_points;

        assert!(num_of_units_dead >= 0);

        num_of_units_dead = if num_of_units_dead >= self.num_of_units {
            self.num_of_units
        } else {
            num_of_units_dead
        };

        self.num_of_units = self.num_of_units - num_of_units_dead;

        // println!(
        //     "Group {} ({:?}) attacking Group {} ({:?}): {} units died",
        //     other_group.id, other_group.race, self.id, self.race, num_of_units_dead
        // );
    }

    fn is_alive(&self) -> bool {
        return self.num_of_units > 0;
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

#[derive(Debug, Eq, PartialEq)]
enum WarStatus {
    Over,
    NotOver,
}

#[derive(Debug)]
struct Battle {
    groups: BinaryHeap<Group>,
}

impl Battle {
    fn new(groups: BinaryHeap<Group>) -> Self {
        let mut new_group = BinaryHeap::new();

        let mut current_id = 0;
        for mut group in groups {
            group.id = current_id;
            current_id += 1;

            new_group.push(group);
        }

        Battle { groups: new_group }
    }

    fn has_targets(&self, current_group: &Group) -> bool {
        return self
            .groups
            .iter()
            .filter(|group| {
                // cannot attack its own race
                return current_group.race != group.race;
            })
            .count()
            > 0;
    }

    fn execute_fight_round(&mut self) -> WarStatus {
        // target selection phase

        let targets = self.groups.clone();
        let mut queue = self.groups.clone();

        let mut unavailable_targets: HashSet<i32> = HashSet::new();

        let mut groups_lookup: HashMap<GroupID, Group> = HashMap::new();
        let mut target_selection: Vec<(GroupID, GroupID)> = vec![];

        while let Some(current_group) = queue.pop() {
            groups_lookup.insert(current_group.id, current_group.clone());

            // TODO: remove
            // println!(
            //     "{} effective_power -- {} initiative",
            //     current_group.effective_power(),
            //     current_group.initiative
            // );

            if !self.has_targets(&current_group) {
                return WarStatus::Over;
            }

            let mut potential_targets: Vec<(&Group, i32)> = targets
                .iter()
                .filter(|target| {
                    // cannot attack itself
                    return current_group.id != target.id;
                })
                .filter(|target| {
                    // cannot attack its own race
                    return current_group.race != target.race;
                })
                .map(|target| {
                    let potential_damage = current_group.calculate_damage_to_group(target);
                    return (target, potential_damage);
                })
                .filter(|(_target, potential_damage)| {
                    // only consider targets for which damage can be dealt
                    return potential_damage > &0;
                })
                .filter(|(target, _potential_damage)| {
                    // only consider targets for which are not chosen
                    return !unavailable_targets.contains(&target.id);
                })
                .collect();

            potential_targets.sort_by(|this, other| {
                let (this_target, this_potential_damage) = this;
                let (other_target, other_potential_damage) = other;

                if this_potential_damage != other_potential_damage {
                    // The attacking group chooses to target the group in the enemy army to which it would deal the most damage
                    // (after accounting for weaknesses and immunities, but not accounting for whether the defending group
                    // has enough units to actually receive all of that damage).
                    return other_potential_damage.cmp(this_potential_damage);
                }

                // If an attacking group is considering two defending groups to which it would deal equal damage,
                // it chooses to target the defending group with the largest effective power;
                // if there is still a tie, it chooses the defending group with the highest initiative.
                return target_order(other_target, this_target);
            });

            if potential_targets.len() > 0 {
                let (target, _damage) = potential_targets.get(0).unwrap();

                assert!(!unavailable_targets.contains(&target.id));

                unavailable_targets.insert(target.id);
                target_selection.push((current_group.id, target.id));
            }

            // TODO: remove
            // for (target, damage) in potential_targets {
            //     println!(
            //         "Group {} ({:?}) to Group {} ({:?}): {} damage to take -- {} effective_power -- {} initiative -- {} dead units",
            //         current_group.id,
            //         current_group.race,
            //         target.id,
            //         target.race,
            //         damage,
            //         target.effective_power(),
            //         target.initiative,
            //         damage / target.hit_points
            //     );
            // }

            // println!("-----");
        }

        // attack phase

        target_selection.sort_by(|(this, _), (other, _)| {
            let this_initiative = groups_lookup.get(&this).unwrap().initiative;
            let other_initiative = groups_lookup.get(&other).unwrap().initiative;

            return other_initiative.cmp(&this_initiative);
        });

        for (attacking_group_id, defending_group_id) in target_selection.into_iter() {
            let attacking_group = groups_lookup.get(&attacking_group_id).unwrap().clone();

            // println!("{}", attacking_group.to_string());

            if !attacking_group.is_alive() {
                continue;
            }

            match groups_lookup.get_mut(&defending_group_id) {
                None => {
                    unreachable!();
                }
                Some(defending_group) => {
                    if !defending_group.is_alive() {
                        continue;
                    }
                    defending_group.take_damage(&attacking_group);
                }
            }
        }

        let mut new_groups: BinaryHeap<Group> = BinaryHeap::new();

        let remaining_groups = groups_lookup
            .values()
            .into_iter()
            .map(|g| g.clone())
            .filter(|g| g.is_alive());

        new_groups.extend(remaining_groups);

        self.groups = new_groups;

        if self.groups.len() > 0 {
            return WarStatus::NotOver;
        }

        return WarStatus::Over;
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    let mut battle = parse_input(input_string);

    loop {
        let status = battle.execute_fight_round();
        // println!("--------");

        if status == WarStatus::Over {
            break;
        }
    }

    let remaining_units = battle.groups.iter().fold(0, |acc, group| {
        return acc + group.num_of_units;
    });

    println!("Part 1: {}", remaining_units);
}
