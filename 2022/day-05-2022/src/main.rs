// https://adventofcode.com/2022/day/5

struct Move {
    amount: u32,
    from_stack: u32,
    to_stack: u32,
}

impl Move {
    fn new(input: String) -> Self {
        let inputs: Vec<String> = input
            .trim()
            .split("from")
            .map(|x| -> String { x.trim().to_string() })
            .collect();

        assert!(inputs.len() == 2);

        let amount: u32 = {
            let parsed: Vec<String> = inputs[0]
                .split("move")
                .map(|x| -> String { x.trim().to_string() })
                .filter(|x| !x.is_empty())
                .collect();
            assert!(parsed.len() == 1);
            parsed[0].parse().unwrap()
        };

        let (from_stack, to_stack) = {
            let parsed: Vec<String> = inputs[1]
                .split("to")
                .map(|x| -> String { x.trim().to_string() })
                .filter(|x| !x.is_empty())
                .collect();
            let from_stack: u32 = parsed[0].parse().unwrap();
            assert!(from_stack >= 1);
            let to_stack: u32 = parsed[1].parse().unwrap();
            assert!(to_stack >= 1);
            (from_stack - 1, to_stack - 1)
        };

        Move {
            amount,
            from_stack,
            to_stack,
        }
    }
}

fn rotate_diagram(diagram: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_diagram: Vec<Vec<char>> = {
        // Allocate size for the transposed matrix.
        let num_of_rows = diagram.len();
        let num_of_columns = diagram
            .iter()
            .map(|row| -> usize { row.len() })
            .max()
            .unwrap();
        vec![vec![' '; num_of_rows]; num_of_columns]
    };
    // diagram[y][x]
    // diagram[i][j]
    // diagram[row][column]

    // transpose the diagram
    for (j, row) in diagram.iter().enumerate() {
        for (i, entry) in row.iter().enumerate() {
            // the i-th row, j-th column element of new_diagram is the j-th row, i-th column element of diagram
            new_diagram[i][j] = *entry;
        }
    }

    // reverse the order of the columns
    new_diagram
        .into_iter()
        .map(|mut row| {
            row.reverse();
            row
        })
        .collect()
}

struct Crane {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Move>,
}

impl Crane {
    fn new(input: String) -> Self {
        let inputs: Vec<String> = input
            .split('\n')
            .map(|x| -> String { x.to_string() })
            .collect();

        let mut stack_diagram: Vec<Vec<char>> = vec![];
        let mut should_push_to_stack_diagram = true;
        let mut instructions = vec![];

        for line in inputs {
            if line.is_empty() {
                continue;
            }

            if line.contains("move") {
                should_push_to_stack_diagram = false;
            }

            if should_push_to_stack_diagram {
                stack_diagram.push(line.chars().collect());
            } else {
                instructions.push(Move::new(line));
            }
        }

        // rotate stack_diagram
        let stack_diagram = rotate_diagram(stack_diagram);
        let stack_diagram: Vec<String> = stack_diagram
            .into_iter()
            .map(|x| -> String { x.into_iter().collect() })
            .collect();

        // parse stack diagram
        let mut stacks = vec![];
        for line in stack_diagram {
            let line = line.trim();
            if line.is_empty() || line.contains('[') || line.contains(']') {
                continue;
            }

            let mut line: Vec<char> = line.chars().collect();
            // Remove first char that indicates the stack number.
            let actual_index = line.remove(0).to_digit(10).unwrap();
            let expected_index = stacks.len();

            assert_eq!(expected_index as u32, actual_index - 1);
            stacks.push(line);
        }
        Crane {
            stacks,
            instructions,
        }
    }

    fn part_1(&self) -> String {
        let mut stacks = self.stacks.clone();
        for instruction in &self.instructions {
            assert!(instruction.from_stack != instruction.to_stack);
            let mut from_stack = stacks[instruction.from_stack as usize].clone();
            let mut to_stack = stacks[instruction.to_stack as usize].clone();
            for _ in 1..=instruction.amount {
                let item = from_stack.pop().unwrap();
                to_stack.push(item);
            }
            stacks[instruction.from_stack as usize] = from_stack;
            stacks[instruction.to_stack as usize] = to_stack;
        }

        // message
        stacks.iter().map(|stack| stack.last().unwrap()).collect()
    }

    fn part_2(&self) -> String {
        let mut stacks = self.stacks.clone();
        for instruction in &self.instructions {
            assert!(instruction.from_stack != instruction.to_stack);
            let mut from_stack = stacks[instruction.from_stack as usize].clone();
            let mut to_stack = stacks[instruction.to_stack as usize].clone();

            let mut items: Vec<char> = from_stack
                .iter()
                .rev()
                .take(instruction.amount as usize)
                .rev()
                .copied()
                .collect();

            from_stack = from_stack
                .iter()
                .rev()
                .skip(instruction.amount as usize)
                .rev()
                .copied()
                .collect();

            to_stack.append(&mut items);

            stacks[instruction.from_stack as usize] = from_stack;
            stacks[instruction.to_stack as usize] = to_stack;
        }

        // message
        stacks.iter().map(|stack| stack.last().unwrap()).collect()
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    let crane = Crane::new(input_string.to_string());
    let message = crane.part_1();
    println!("Part 1: {}", message);
    assert_eq!(message, "TBVFVDZPN".to_string());

    let message = crane.part_2();
    println!("Part 2: {}", message);
    assert_eq!(message, "VLCWHTDSZ".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // whitespace is important here
        let input_string = r###"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"###
        .to_string();

        let crane = Crane::new(input_string.to_string());
        assert_eq!(crane.part_1(), "CMZ".to_string());
        assert_eq!(crane.part_2(), "MCD".to_string());
    }
}
