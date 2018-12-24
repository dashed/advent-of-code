// https://adventofcode.com/2018/day/24

// imports

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
struct Group {
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

    fn calculate_damage_to_group(&self, other_group: &Self) -> i32 {
        return 0;
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

fn get_attackable_target() {}

fn main() {
    let input_string = include_str!("input.txt");

    println!("{}", input_string);
}
