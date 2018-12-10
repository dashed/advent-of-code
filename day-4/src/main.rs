// https://adventofcode.com/2018/day/4

// imports

use std::collections::HashMap;

// types

type Minute = i32;
type Occurences = i32;

struct Guard {
    minutes_slept: i32,

    // which minute portion on midnight was the guard sleeping at, and how many times?
    occurences_counter: HashMap<Minute, Occurences>,
}

fn main() {
    let input_string = include_str!("input.txt");

    let inputs = input_string.lines();

    for input in inputs {
        println!("{:?}", input);
    }
}
