use num::integer::lcm;
use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Instruction {
    Right,
    Left,
}

#[derive(Clone, Debug)]
struct Node {
    label: String,

    // vertices connecting to this node
    left: String,
    right: String,
}

impl Node {
    fn new(label: String, left: String, right: String) -> Self {
        Self { label, left, right }
    }
}

type Nodes = HashMap<String, Node>;

fn parse_input(input_string: &str) -> (Vec<Instruction>, Nodes) {
    let mut inputs: Vec<&str> = input_string.trim().lines().collect();

    let instructions = inputs.remove(0);

    // parse instructions
    let instructions: Vec<Instruction> = instructions
        .chars()
        .map(|c| match c {
            'R' => Instruction::Right,
            'L' => Instruction::Left,
            _ => panic!("Invalid instruction"),
        })
        .collect();

    // parse nodes
    let mut nodes: Nodes = HashMap::new();

    for input in inputs.into_iter() {
        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let input: Vec<String> = input.split('=').map(|x| x.trim().to_string()).collect();
        assert!(input.len() == 2);

        let label = input[0].clone();

        let node = {
            let choices: Vec<String> = input[1]
                .split(',')
                .map(|x| x.trim().to_string())
                .collect::<Vec<String>>();
            assert!(choices.len() == 2);

            let left = choices[0].trim_start_matches('(').to_string();
            let right = choices[1].trim_end_matches(')').to_string();

            Node::new(label, left, right)
        };

        nodes.insert(node.label.clone(), node);
    }

    (instructions, nodes)
}

fn part_1(input_string: &str) -> i64 {
    let (instructions, nodes) = parse_input(input_string);

    let mut current_node = nodes.get("AAA").unwrap();
    let mut num_of_steps = 0;
    let mut instruction_step = 0;

    loop {
        if current_node.label == "ZZZ" {
            break;
        }

        num_of_steps += 1;

        let current_instruction = &instructions[instruction_step];

        match current_instruction {
            Instruction::Right => {
                current_node = nodes.get(&current_node.right).unwrap();
            }
            Instruction::Left => {
                current_node = nodes.get(&current_node.left).unwrap();
            }
        }

        instruction_step += 1;
        instruction_step %= instructions.len();
    }

    num_of_steps
}

fn part_2(input_string: &str) -> i64 {
    let (instructions, nodes) = parse_input(input_string);

    // start at every node that ends with A
    let mut current_nodes: Vec<Node> = nodes
        .values()
        .filter(|x| x.label.ends_with('A'))
        .cloned()
        .collect();

    let mut num_of_steps = 0;
    let mut instruction_step = 0;
    let current_nodes_len = current_nodes.len();
    let mut cycle_lengths = vec![0; current_nodes_len];

    // After examining the input, we will know that each node that ends with A will cycle.
    //
    // For example:
    // xxA -> xxB --> ... -> xxZ --> xxB --> ...
    // We measure the cycle length (or number of steps) from xxB to xxZ.
    //
    // Knowing this, we take the LCM (least common multiple) of all cycle lengths to get the number of steps for all
    // nodes to reach a node that ends with Z.

    // find cycle lengths
    loop {
        if cycle_lengths.par_iter().all(|x| *x != 0) {
            break;
        }

        {
            for (index, current_node) in current_nodes.iter().enumerate() {
                if current_node.label.ends_with('Z') && cycle_lengths[index] == 0 {
                    cycle_lengths[index] = num_of_steps;
                }
            }
        }

        // check if every node in current_nodes ends with Z
        // if current_nodes.par_iter().all(|x| x.label.ends_with("Z")) {
        //     break;
        // }

        num_of_steps += 1;
        let current_instruction = &instructions[instruction_step];

        current_nodes = current_nodes
            .into_par_iter()
            .map(|current_node| {
                match current_instruction {
                    Instruction::Right => {
                        // next node
                        nodes.get(&current_node.right).unwrap()
                    }
                    Instruction::Left => nodes.get(&current_node.left).unwrap(),
                }
            })
            .cloned()
            .collect();

        assert!(current_nodes.len() == current_nodes_len);

        instruction_step += 1;
        instruction_step %= instructions.len();
    }

    cycle_lengths.into_iter().reduce(lcm).unwrap()
}

fn main() {
    let input_string = include_str!("input.txt");

    // Part 1

    let answer = part_1(input_string);
    println!("Part 1: {}", answer);
    assert_eq!(answer, 21883);

    // Part 2

    let answer = part_2(input_string);
    println!("Part 2: {}", answer);
    assert_eq!(answer, 12833235391111);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle() {
        let input_string = r###"
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"###;

        assert_eq!(part_1(input_string), 2);

        let input_string = r###"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"###;

        assert_eq!(part_1(input_string), 6);

        let input_string = r###"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"###;

        assert_eq!(part_2(input_string), 6);
    }
}
