// https://adventofcode.com/2018/day/4

// imports

use std::collections::HashMap;
use std::collections::BTreeMap;


// helpers

fn substring(this: &str, start: usize, len: usize) -> String {
    this.chars().skip(start).take(len).collect()
}

// types

type GuardID = String;
type Minute = i32;
// type Occurences = i32;
type Timestamp = String;

// records are added to the b-tree map in the order of their timestamps
type GuardShifts = BTreeMap<Timestamp, Record>;

#[derive(Debug)]
enum GuardState {
    BeginsShift(GuardID),
    Sleeps(Minute),
    Wakes(Minute),
}

#[derive(Debug)]
struct Record {
    date_time: Timestamp,
    state: GuardState,
}

// struct Guard {
//     id: GuardID,

//     minutes_slept: i32,

//     // which minute portion on midnight was the guard sleeping at, and how many times?
//     occurences_counter: HashMap<Minute, Occurences>,
// }

// track minutes slept for a guard
type GuardSleepCounter = HashMap<GuardID, i32>;

fn parse_record(input: &str) -> Record {
    let (date_string, state_string) = {
        let mut date_string = input.to_string();
        let state_string = date_string.split_off(19);

        (date_string, state_string)
    };

    // parse date

    let minute: Minute = substring(&date_string, 15, 2).parse().unwrap();

    // parse state_string

    let state: GuardState = if state_string.starts_with("wakes up") {
        GuardState::Wakes(minute)
    } else if state_string.starts_with("falls asleep") {
        GuardState::Sleeps(minute)
    } else if state_string.starts_with("Guard") {
        let inputs: Vec<&str> = state_string.split_whitespace().collect();
        let guard_id: GuardID = inputs.get(1).unwrap().chars().skip(1).collect();
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

    // for the b-tree map; ensure this ordering invariant holds
    assert!("1518-09-24" < "1518-10-24");

    let input_string = include_str!("input.txt");

    let inputs = input_string.lines();

    let mut guard_shifts: GuardShifts = BTreeMap::new();

    for input in inputs {
        let record = parse_record(input);

        guard_shifts.insert(record.date_time.clone(), record);
    }

    let mut guard_sleep_tracker: GuardSleepCounter = HashMap::new();
    let mut current_guard: Option<GuardID> = None;
    let mut initial_slept: Option<Minute> = None;

    for (_date_time, record) in guard_shifts {

        // println!("{:?}", record.state);

        match record.state {
            GuardState::BeginsShift(guard_id) => {
                current_guard = Some(guard_id.clone());
                guard_sleep_tracker.entry(guard_id).or_insert(0);
            }
            GuardState::Sleeps(minute) => {
                assert!(current_guard.is_some());
                assert!(initial_slept.is_none());
                initial_slept = Some(minute);
            }
            GuardState::Wakes(minute) => {
                assert!(current_guard.is_some());
                assert!(initial_slept.is_some());
                let minutes_slept = minute - initial_slept.unwrap();

                initial_slept = None;

                // track minutes slept for current guard

                guard_sleep_tracker
                    .entry(current_guard.clone().unwrap())
                   .and_modify(|x| { *x += minutes_slept })
                   .or_insert(minutes_slept);
            }
        }
    }

    {

        // Find the guard that has the most minutes asleep.

        let mut guard_who_sleeps_the_most: Option<GuardID> = None;
        let mut guard_minutes_slept = 0;

        for (guard_id, minutes_slept) in guard_sleep_tracker {

            if guard_who_sleeps_the_most.is_none() {
                guard_who_sleeps_the_most = Some(guard_id);
                guard_minutes_slept = minutes_slept;
                continue;
            }

            if minutes_slept > guard_minutes_slept {
                guard_who_sleeps_the_most = Some(guard_id);
                guard_minutes_slept = minutes_slept;
            }

        }

        // TODO: What minute does that guard spend asleep the most?

    };

}
