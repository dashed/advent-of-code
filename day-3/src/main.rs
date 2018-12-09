// https://adventofcode.com/2018/day/3

// imports

use std::str::Lines;

// part 1

struct Fabric {
    id: String,

    // starting coordinates
    left: i32,
    top: i32,

    // size
    height: i32,
    width: i32,
}

fn parse_to_fabric(input: &str) -> Fabric {
    // TODO: parsing magic

    Fabric {
        id: "123".to_string(),
        left: 123,
        top: 123,
        height: 123,
        width: 123,
    }
}

fn part_1(inputs: Lines) {
    for input in inputs {
        let fabric = parse_to_fabric(input);
        println!("{}", input);
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    let inputs = input_string.lines();

    part_1(inputs);
}
