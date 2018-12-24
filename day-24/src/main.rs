// https://adventofcode.com/2018/day/24

// imports

use std::collections::HashSet;

// code

struct Group {
    num_of_units: i32,
    hit_points: i32,

    attack_damage: i32,
    attack_type: String,
    initiative: i32,

    immunity: HashSet<String>,
    weaknesses: HashSet<String>,
}

fn main() {
    let input_string = include_str!("input.txt");

    println!("{}", input_string);
}
