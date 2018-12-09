// https://adventofcode.com/2018/day/3

// imports

use std::str::Lines;

// part 1

fn part_1(inputs: Lines) {
    for input in inputs {
        println!("{}", input);
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    let inputs = input_string.lines();

    part_1(inputs);
}
