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

#[derive(Debug)]
struct Record {
    date_time: String,
    state: GuardState,
}

struct Guard {
    id: GuardID,

    minutes_slept: i32,

    // which minute portion on midnight was the guard sleeping at, and how many times?
    occurences_counter: HashMap<Minute, Occurences>,
}

fn parse_record(input: &str) -> Record {
    let (date_string, state_string) = {
        let mut date_string = input.to_string();
        let state_string = date_string.split_off(19);

        (date_string, state_string)
    };

    // parse date

    let minute: Minute = substring(&date_string, 15, 2).parse().unwrap();

    // ensure this ordering invariant holds
    assert!("1518-09-24" < "1518-10-24");

    // parse state_string

    let state: GuardState = if state_string.starts_with("wakes up") {
        GuardState::Wakes(minute)
    } else if state_string.starts_with("falls asleep") {
        GuardState::Sleeps(minute)
    } else if state_string.starts_with("Guard") {
        let inputs: Vec<&str> = state_string.split_whitespace().collect();
        let guard_id: GuardID = inputs.get(1).unwrap().to_string();
        GuardState::BeginsShift(guard_id)
    } else {
        unreachable!();
    };

    Record {
        date_time: date_string,
        state: state,
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    let inputs = input_string.lines();

    for input in inputs {
        let record = parse_record(input);

        println!("{:?}", record);
    }
}
