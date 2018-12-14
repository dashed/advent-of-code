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

#[derive(Debug)]
enum PotState {
    NoPlant,
    HasPlant,
}

impl PotState {
    fn from_char(x: char) -> PotState {
        assert!(is_valid_plant_state(x));

        if has_plant(x) {
            return PotState::HasPlant;
        }

        return PotState::NoPlant;
    }
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
        let mut iter = input.trim().split("=>").into_iter();

        let initial_rule: &str = iter.next().unwrap().trim();

        let next_state: PotState = iter
            .next()
            .unwrap()
            .trim()
            .chars()
            .map(PotState::from_char)
            .next()
            .unwrap();

        println!("{} => {:?}", initial_rule, next_state);
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

        let inputs: Vec<String> = iter.map(|x| x.trim().to_string()).collect();

        for input in inputs {
            Rule::from_string(&input);
        }
    };

    // println!("{:?}", inputs);
}
