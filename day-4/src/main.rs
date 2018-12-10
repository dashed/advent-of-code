// https://adventofcode.com/2018/day/4

// imports

use std::collections::BTreeMap;
use std::collections::HashMap;

// helpers

fn substring(this: &str, start: usize, len: usize) -> String {
    this.chars().skip(start).take(len).collect()
}

// types

type GuardID = i32;
type Minute = i32;
type Occurences = i32;
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

#[derive(Debug)]
struct Guard {
    id: GuardID,

    minutes_slept: i32,

    // which minute portion on midnight was the guard sleeping at, and how many times?
    occurences_counter: HashMap<Minute, Occurences>,
}

impl Guard {
    fn new(id: GuardID) -> Guard {
        Guard {
            id: id,
            minutes_slept: 0,
            occurences_counter: HashMap::new(),
        }
    }

    fn slept_at_between(&mut self, slept_at: Minute, woke_up_at: Minute) {
        assert!(slept_at < woke_up_at);

        for minute in slept_at..woke_up_at {
            self.occurences_counter
                .entry(minute)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }

        let minutes_slept = woke_up_at - slept_at;

        self.minutes_slept += minutes_slept;
    }

    fn get_minute_slept_most_at(&self) -> Option<(Minute, Occurences)> {
        // What minute does that guard spend asleep the most?

        let result = self.occurences_counter.iter().fold(
            None,
            |acc: Option<(Minute, Occurences)>, (minute, occurences_count)| match acc {
                None => {
                    return Some((*minute, *occurences_count));
                }
                Some((_prev_minute, prev_occurences_count)) => {
                    if occurences_count > &prev_occurences_count {
                        return Some((*minute, *occurences_count));
                    }
                    return acc;
                }
            },
        );

        return result;
    }
}

// track minutes slept for a guard
type GuardSleepCounter = HashMap<GuardID, Guard>;

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
        let guard_id: GuardID = {
            let x: String = inputs.get(1).unwrap().chars().skip(1).collect();
            x.parse().unwrap()
        };
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
    let mut slept_at: Option<Minute> = None;

    for (_date_time, record) in guard_shifts {
        // println!("{:?}", record.state);

        match record.state {
            GuardState::BeginsShift(guard_id) => {
                current_guard = Some(guard_id.clone());
                guard_sleep_tracker
                    .entry(guard_id.clone())
                    .or_insert(Guard::new(guard_id));
            }
            GuardState::Sleeps(minute) => {
                assert!(current_guard.is_some());
                assert!(slept_at.is_none());
                slept_at = Some(minute);
            }
            GuardState::Wakes(woke_up_at) => {
                assert!(current_guard.is_some());
                assert!(slept_at.is_some());

                // track minutes slept for current guard

                let guard_id = current_guard.clone().unwrap();

                assert!(guard_sleep_tracker.contains_key(&guard_id));

                let guard = guard_sleep_tracker.get_mut(&guard_id).unwrap();

                guard.slept_at_between(slept_at.unwrap(), woke_up_at);

                slept_at = None;
            }
        }
    }

    {
        // Find the guard that has the most minutes asleep.

        let guard_who_sleeps_the_most =
            guard_sleep_tracker
                .iter()
                .fold(None, |acc: Option<&Guard>, (_guard_id, guard)| match acc {
                    None => {
                        return Some(guard);
                    }
                    Some(prev_guard) => {
                        if guard.minutes_slept > prev_guard.minutes_slept {
                            return Some(guard);
                        }

                        return acc;
                    }
                });

        match guard_who_sleeps_the_most {
            None => {
                println!("No guard found");
            }
            Some(guard) => {
                println!(
                    "Guard #{} slept the most with {} minutes",
                    guard.id, guard.minutes_slept
                );

                let (minute_slept_most_at, _occurences) = guard.get_minute_slept_most_at().unwrap();

                println!(
                    "This guard slept the most on the {} minute.",
                    minute_slept_most_at
                );

                let part_1_answer = minute_slept_most_at * guard.id;

                println!(
                    "Part 1 answer: {} * {} = {}",
                    guard.id, minute_slept_most_at, part_1_answer
                );
            }
        }
    };
}
