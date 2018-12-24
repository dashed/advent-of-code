// https://adventofcode.com/2018/day/24

// imports

use core::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

// code

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
        // In decreasing order of effective power, groups choose their targets;
        if self.effective_power() != other.effective_power() {
            return self.effective_power().cmp(&self.effective_power());
        }

        // in a tie, the group with the higher initiative chooses first.
        return self.initiative.cmp(&other.initiative);
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
