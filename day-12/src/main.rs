use std::collections::BTreeMap;

// helpers and types

type PotIndex = i32;
type HasPlant = bool;
type State = BTreeMap<PotIndex, HasPlant>;

fn has_plant(x: char) -> bool {
    return x == '#';
}

fn is_valid_plant_state(x: char) -> bool {
    return x == '.' || x == '#';
}

enum PotState {
    NoPlant,
    HasPlant,
}

struct Rule {
    // Representation of LLCRR => N
    // where L are pots to the left,
    // C is the current pot being considered,
    // R are the pots to the right,
    // and N is whether the current pot will have a plant in the next generation.
    left: (PotState, PotState),
    current: PotState,
    right: (PotState, PotState),

    next: PotState,
}

impl Rule {
    fn from_string(input: &str) {
        input.trim().split("=>");
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    let inputs: Vec<&str> = input_string.trim().lines().collect();

    let mut state: State = BTreeMap::new();

    {
        let mut iter = inputs.iter();

        // add initial state

        let initial_state = iter.next().unwrap();

        let initial_state_iter = initial_state
            .chars()
            .filter(|x| {
                return is_valid_plant_state(*x);
            })
            .enumerate();

        for (pot_index, has_plant_char) in initial_state_iter {
            state.insert(pot_index as i32, has_plant(has_plant_char));
        }

        // skip empty line
        iter.next();

        let inputs: Vec<String> = iter.map(|x| x.to_string()).collect();
    };

    // println!("{:?}", inputs);
}
