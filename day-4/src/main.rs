// https://adventofcode.com/2018/day/4

// imports

use std::collections::HashMap;

// helpers

fn substring(this: &str, start: usize, len: usize) -> String {
    this.chars().skip(start).take(len).collect()
}

// types

type GuardID = String;
type Minute = i32;
type Occurences = i32;

#[derive(Debug)]
enum GuardState {
    BeginsShift(GuardID),
    Sleeps(Minute),
    Wakes(Minute),
}

struct Record {
    state: GuardState,
}

struct Guard {
    id: GuardID,

    minutes_slept: i32,

    // which minute portion on midnight was the guard sleeping at, and how many times?
    occurences_counter: HashMap<Minute, Occurences>,
}

fn parse_record(input: &str) {
    println!("input: {:?}", input);

    let (date_string, state_string) = {
        let mut date_string = input.to_string();
        let state_string = date_string.split_off(19);

        (date_string, state_string)
    };

    // parse date

    // parse state_string

    let state: GuardState = if state_string.starts_with("wakes up") {
        GuardState::Wakes(0)
    } else if state_string.starts_with("falls asleep") {
        GuardState::Sleeps(0)
    } else if state_string.starts_with("Guard") {
        GuardState::BeginsShift("0".to_string())
    } else {
        unreachable!();
    };

    println!("{}", date_string);
    println!("{:?}", state);
}

fn main() {
    let input_string = include_str!("input.txt");

    let mut inputs = input_string.lines();

    parse_record(inputs.next().unwrap());

    // for input in inputs {
    //     println!("{:?}", input);
    // }
}
