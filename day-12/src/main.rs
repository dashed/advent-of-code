use std::collections::BTreeMap;
use std::collections::HashMap;

// helpers and types

type PotIndex = i32;
type State = BTreeMap<PotIndex, PotState>;
type Rules = HashMap<InitialRule, Rule>;

fn has_plant(x: char) -> bool {
    return x == '#';
}

fn is_valid_plant_state(x: char) -> bool {
    return x == '.' || x == '#';
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
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

    fn has_plant(&self) -> bool {
        match self {
            PotState::HasPlant => true,
            PotState::NoPlant => false,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct InitialRule {
    left: (PotState, PotState),
    current: PotState,
    right: (PotState, PotState),
}

struct Rule {
    // Representation of LLCRR => N
    // where L are pots to the left,
    // C is the current pot being considered,
    // R are the pots to the right,
    // and N is whether the current pot will have a plant in the next generation.
    initial_rule: InitialRule,
    next: PotState,
}

impl Rule {
    fn from_string(input: &str) -> Rule {
        let mut iter = input.trim().split("=>").into_iter();

        let initial_rule: Vec<PotState> = iter
            .next()
            .unwrap()
            .trim()
            .chars()
            .map(PotState::from_char)
            .collect();

        assert!(initial_rule.len() == 5);

        let next_state: PotState = iter
            .next()
            .unwrap()
            .trim()
            .chars()
            .map(PotState::from_char)
            .next()
            .unwrap();

        let mut initial_rule_iter = initial_rule.into_iter();

        let initial_rule = InitialRule {
            left: (
                initial_rule_iter.next().unwrap(),
                initial_rule_iter.next().unwrap(),
            ),
            current: initial_rule_iter.next().unwrap(),
            right: (
                initial_rule_iter.next().unwrap(),
                initial_rule_iter.next().unwrap(),
            ),
        };

        Rule {
            initial_rule: initial_rule,

            next: next_state,
        }
    }
}

fn get_pot_from_state(state: &State, index: PotIndex) -> PotState {
    match state.get(&index) {
        None => PotState::NoPlant,
        Some(plant_state) => plant_state.clone(),
    }
}

fn generate_next_state(mut state: State, rules: &Rules) -> State {
    {
        // extend state by considering two pots on both ends;
        // only if there are pots with plants on either ends
        let keys: Vec<PotIndex> = state.keys().cloned().collect();
        let highest: PotIndex = *keys.iter().max().unwrap();
        let lowest: PotIndex = *keys.iter().min().unwrap();

        if state.get(&highest).unwrap().has_plant() {
            state.insert(highest + 1, PotState::NoPlant);
            state.insert(highest + 2, PotState::NoPlant);
        }

        if state.get(&lowest).unwrap().has_plant() {
            state.insert(lowest - 1, PotState::NoPlant);
            state.insert(lowest - 2, PotState::NoPlant);
        }
    };

    let mut next_state = state.clone();

    for (pot_index, plant_state) in state.iter() {
        let left_plant_2: PotState = get_pot_from_state(&state, pot_index - 2);
        let left_plant_1: PotState = get_pot_from_state(&state, pot_index - 1);
        let right_plant_1: PotState = get_pot_from_state(&state, pot_index + 1);
        let right_plant_2: PotState = get_pot_from_state(&state, pot_index + 2);

        let rule_needle = InitialRule {
            left: (left_plant_2, left_plant_1),
            current: plant_state.clone(),
            right: (right_plant_1, right_plant_2),
        };

        if rules.contains_key(&rule_needle) {
            // assert!(rules.contains_key(&rule_needle));

            let next_plant_state = &rules.get(&rule_needle).unwrap().next;

            next_state.insert(*pot_index, next_plant_state.clone());
        } else {
            next_state.insert(*pot_index, PotState::NoPlant);
        }
    }

    return next_state;
}

fn state_to_string(state: &State) -> String {
    let state_string: String = state
        .iter()
        .map(|(_key, plant_state)| match plant_state {
            PotState::HasPlant => "#",
            PotState::NoPlant => ".",
        })
        .collect();
    return state_string;
}

fn main() {
    let input_string = include_str!("input.txt");

    let inputs: Vec<&str> = input_string.trim().lines().collect();

    let mut state: State = BTreeMap::new();

    let rules: Rules = {
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
            state.insert(pot_index as i32, PotState::from_char(has_plant_char));
        }

        // skip empty line
        iter.next();

        let inputs: Vec<String> = iter.map(|x| x.trim().to_string()).collect();

        let mut rules: Rules = HashMap::new();

        for input in inputs {
            let rule = Rule::from_string(&input);
            let initial_rule = rule.initial_rule.clone();

            assert!(!rules.contains_key(&initial_rule));

            rules.insert(initial_rule, rule);
        }

        rules
    };

    println!("Initial state: {}", state_to_string(&state));

    let num_of_generations = 20;

    for _generation in 1..=num_of_generations {
        state = generate_next_state(state, &rules);

        // Debug
        // println!("{}", state_to_string(&state));
    }

    let part_1: i32 = state
        .iter()
        .map(|(pot_index, plant_state)| -> i32 {
            match plant_state {
                PotState::HasPlant => *pot_index,
                PotState::NoPlant => 0,
            }
        })
        .sum();

    println!("Part 1: {}", part_1);
}
