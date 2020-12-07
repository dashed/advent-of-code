fn main() {
    let input_string = include_str!("input.txt");

    let puzzle_inputs = parse_input(input_string);

    // Part 1

    println!("Part 1: {}", part_1(puzzle_inputs.clone()));

    // Part 2

    println!("Part 2: {}", part_2(puzzle_inputs));
}

#[derive(Clone)]
struct PasswordPolicy {
    min: u32,
    max: u32,
    character: char,
    password: String,
}

impl PasswordPolicy {
    fn is_valid_old_policy(&self) -> bool {
        let counts = self.password.matches(self.character).count() as u32;
        return self.min <= counts && counts <= self.max;
    }

    fn is_valid_new_policy(&self) -> bool {
        let first_position_char = self.password.chars().nth(self.min as usize - 1).unwrap();
        let second_position_char = self.password.chars().nth(self.max as usize - 1).unwrap();

        let is_same = first_position_char == second_position_char;

        if first_position_char == self.character && !is_same {
            return true;
        }

        if second_position_char == self.character && !is_same {
            return true;
        }

        return false;
    }
}

fn parse_input(input_string: &str) -> Vec<PasswordPolicy> {
    let inputs: Vec<&str> = input_string.trim().split('\n').collect();

    let mut policies: Vec<PasswordPolicy> = vec![];

    for input in inputs {
        let input: Vec<&str> = input.trim().split(':').collect();

        let raw_policy: Vec<&str> = input[0].trim().split_whitespace().collect();
        let min_max: Vec<&str> = raw_policy[0].trim().split('-').collect();
        let min: u32 = min_max[0].trim().parse().unwrap();
        let max: u32 = min_max[1].trim().parse().unwrap();
        let character = raw_policy[1].trim().chars().next().unwrap();

        let password = input[1].trim().to_string();

        policies.push(PasswordPolicy {
            min,
            max,
            character,
            password,
        });
    }

    return policies;
}

fn part_1(entries: Vec<PasswordPolicy>) -> i32 {
    let mut num_of_valid_passwords = 0;

    for entry in entries {
        if entry.is_valid_old_policy() {
            num_of_valid_passwords += 1;
        }
    }

    return num_of_valid_passwords;
}

fn part_2(entries: Vec<PasswordPolicy>) -> i32 {
    let mut num_of_valid_passwords = 0;

    for entry in entries {
        if entry.is_valid_new_policy() {
            num_of_valid_passwords += 1;
        }
    }

    return num_of_valid_passwords;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_2() {
        let input_string = include_str!("input.txt");

        let puzzle_inputs = parse_input(input_string);

        assert_eq!(part_1(puzzle_inputs.clone()), 625);
        assert_eq!(part_2(puzzle_inputs), 391);        
    }
}
