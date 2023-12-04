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
        let chars: Vec<char> = needle.chars().collect();
        tokens(|l, r| l.eq_ignore_ascii_case(&r), "error".into(), chars)
            .map(move |_| needle.clone())
    };

    let integer = || many1(digit()).map(|string: String| -> i32 { string.parse::<i32>().unwrap() });

    let immunity_start = (constant("Immune System:".to_string()), skip_spaces());
    let infection_start = (constant("Infection:".to_string()), skip_spaces());

    let list_of_words = || {
        sep_by1::<HashSet<String>, _, _>(many1(letter()), spaces().skip(char(',')).skip(spaces()))
    };

    let parse_immunities = (
        constant("immune to".to_string()).with(skip_spaces()),
        list_of_words(),
    )
        .map(|(_, words)| Trait::Immunities(words));

    let parse_weaknesses = (
        constant("weak to".to_string()).with(skip_spaces()),
        list_of_words(),
    )
        .map(|(_, words)| Trait::Weaknesses(words));

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
                                    Trait::Immunities(_) => true,
                                    _ => false,
                                })
                                .map(|x| (*x).clone().unwrap())
                                .unwrap_or_default();

                            let weaknesses = traits
                                .iter()
                                .find(|item| match item {
                                    Trait::Weaknesses(_) => true,
                                    _ => false,
                                })
                                .map(|x| (*x).clone().unwrap())
                                .unwrap_or_default();

                            (immunities, weaknesses)
                        })
                        .unwrap_or((HashSet::new(), HashSet::new()));

                    Group {
                        id: 0,

                        race: race.clone(),

                        num_of_units,
                        hit_points,

                        attack_damage,
                        attack_type,
                        initiative,

                        immunities,
                        weaknesses,
                    }
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
            Battle::new(groups)
        });

    let result: Result<(Battle, &str), easy::ParseError<&str>> = parser.easy_parse(input_string);

    match result {
        Ok((value, remaining_input)) => {
            assert!(remaining_input.trim().is_empty());
            value
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
    first_group.initiative.cmp(&second_group.initiative)
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
        self.attack_damage * self.num_of_units
    }

    fn immune_to(&self, attack_type: &String) -> bool {
        self.immunities.contains(attack_type)
    }

    fn weak_to(&self, attack_type: &String) -> bool {
        self.weaknesses.contains(attack_type)
    }

    fn calculate_damage_to_group(&self, other_group: &Self) -> Damage {
        if other_group.immune_to(&self.attack_type) {
            return 0;
        }

        if other_group.weak_to(&self.attack_type) {
            return 2 * self.effective_power();
        }

        self.effective_power()
    }

    fn take_damage(&mut self, other_group: &Self) -> bool {
        let damage_taken = other_group.calculate_damage_to_group(self);

        let mut num_of_units_dead: i32 = damage_taken / self.hit_points;

        assert!(num_of_units_dead >= 0);

        num_of_units_dead = if num_of_units_dead >= self.num_of_units {
            self.num_of_units
        } else {
            num_of_units_dead
        };

        self.num_of_units -= num_of_units_dead;

        // println!(
        //     "Group {} ({:?}) attacking Group {} ({:?}): {} units died",
        //     other_group.id, other_group.race, self.id, self.race, num_of_units_dead
        // );

        num_of_units_dead > 0
    }

    fn is_alive(&self) -> bool {
        self.num_of_units > 0
    }
}

impl Ord for Group {
    fn cmp(&self, other: &Self) -> Ordering {
        target_order(self, other)
    }
}

impl PartialOrd for Group {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq)]
enum WarStatus {
    Over,
    NotOver,
    StaleMate,
}

#[derive(Debug, Clone)]
struct Battle {
    groups: BinaryHeap<Group>,
}

impl Battle {
    fn new(groups: BinaryHeap<Group>) -> Self {
        let mut new_group = BinaryHeap::new();

        for (current_id, mut group) in groups.into_iter().enumerate() {
            group.id = current_id as i32;
            new_group.push(group);
        }

        Battle { groups: new_group }
    }

    fn has_immunity(&self) -> bool {
        return self
            .groups
            .iter()
            .filter(|group| group.race == Race::Immunity)
            .count()
            > 0;
    }

    fn has_infection(&self) -> bool {
        return self
            .groups
            .iter()
            .filter(|group| group.race == Race::Infection)
            .count()
            > 0;
    }

    fn boost(&mut self, boost: Damage) {
        self.groups = self
            .groups
            .iter()
            .map(|group| {
                let mut group = group.clone();
                if group.race == Race::Immunity {
                    group.attack_damage += boost;
                }
                group
            })
            .collect();
    }

    fn has_targets(&self, current_group: &Group) -> bool {
        return self
            .groups
            .iter()
            .filter(|group| {
                // cannot attack its own race
                current_group.race != group.race
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
                    current_group.id != target.id
                })
                .filter(|target| {
                    // cannot attack its own race
                    current_group.race != target.race
                })
                .map(|target| {
                    let potential_damage = current_group.calculate_damage_to_group(target);
                    (target, potential_damage)
                })
                .filter(|(_target, potential_damage)| {
                    // only consider targets for which damage can be dealt
                    potential_damage > &0
                })
                .filter(|(target, _potential_damage)| {
                    // only consider targets for which are not chosen
                    !unavailable_targets.contains(&target.id)
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
                target_order(other_target, this_target)
            });

            if !potential_targets.is_empty() {
                let (target, _damage) = potential_targets.get(0).unwrap();

                assert!(!unavailable_targets.contains(&target.id));

                unavailable_targets.insert(target.id);
                target_selection.push((current_group.id, target.id));
            }
        }

        // attack phase

        target_selection.sort_by(|(this, _), (other, _)| {
            let this_initiative = groups_lookup.get(this).unwrap().initiative;
            let other_initiative = groups_lookup.get(other).unwrap().initiative;

            other_initiative.cmp(&this_initiative)
        });

        let mut units_died_in_this_round = false;

        for (attacking_group_id, defending_group_id) in target_selection.into_iter() {
            let attacking_group = groups_lookup.get(&attacking_group_id).unwrap().clone();

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
                    if defending_group.take_damage(&attacking_group) {
                        units_died_in_this_round = true;
                    }
                }
            }
        }

        let mut new_groups: BinaryHeap<Group> = BinaryHeap::new();

        let remaining_groups = groups_lookup.values().cloned().filter(|g| g.is_alive());

        new_groups.extend(remaining_groups);

        self.groups = new_groups;

        if !units_died_in_this_round {
            return WarStatus::StaleMate;
        }

        if !self.groups.is_empty() {
            return WarStatus::NotOver;
        }

        WarStatus::Over
    }
}

fn part_1(input_string: &str) -> i32 {
    let mut battle = parse_input(input_string);

    loop {
        let status = battle.execute_fight_round();
        // println!("--------");

        if status == WarStatus::Over {
            break;
        }
    }

    let remaining_units = battle
        .groups
        .iter()
        .fold(0, |acc, group| acc + group.num_of_units);

    remaining_units
}

fn part_2(input_string: &str) -> i32 {
    let battle = parse_input(input_string);

    for boost in 0.. {
        let mut battle = battle.clone();
        // println!("Boost: {}", boost);
        battle.boost(boost);

        loop {
            let status = battle.execute_fight_round();

            if status == WarStatus::Over || status == WarStatus::StaleMate {
                break;
            }
        }

        if battle.has_immunity() && !battle.has_infection() {
            let remaining_units = battle.groups.iter().fold(0, |acc, group| {
                assert!(group.race == Race::Immunity);
                acc + group.num_of_units
            });

            return remaining_units;
        }
    }

    0
}

fn main() {
    let input_string = include_str!("input.txt");

    println!("Part 1: {}", part_1(input_string));
    println!("Part 2: {}", part_2(input_string));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r###"
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4
        "###;

        assert_eq!(part_1(input), 5216);

        let input_string = include_str!("input.txt");
        assert_eq!(part_1(input_string), 14799);
    }
}
