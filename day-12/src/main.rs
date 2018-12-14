use std::collections::HashMap;
use std::collections::BTreeMap;

// helpers and types

type PotIndex = i32;
type HasPlant = bool;
type State = BTreeMap<PotIndex, HasPlant>;
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

fn generate_next_state(state: State, rules: &Rules) -> State {

    let next_state = state.clone();

    return next_state;
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
            state.insert(pot_index as i32, has_plant(has_plant_char));
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

    let num_of_generations = 20;

    for _generation in 1..=num_of_generations {
        state = generate_next_state(state, &rules);
    }
}
