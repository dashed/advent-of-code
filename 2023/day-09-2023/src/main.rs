struct History {
    sequence: Vec<i64>,
    differences: Box<Option<History>>,
}

impl History {
    fn new(sequence: Vec<i64>) -> Self {
        let differences: Vec<i64> = sequence.windows(2).map(|x| x[1] - x[0]).collect();
        assert!(!differences.is_empty());

        let mut differences_clone = differences.clone();
        // check if differences are all the same
        differences_clone.dedup();

        let history_difference = if differences_clone.len() == 1 {
            if differences_clone[0] == 0 {
                None
            } else {
                Some(Self::new(differences))
            }
        } else {
            Some(Self::new(differences))
        };

        Self {
            sequence,
            differences: Box::new(history_difference),
        }
    }

    fn find_next_value(&self) -> i64 {
        let last_value = *self.sequence.last().unwrap();

        if let Some(history_difference) = &*self.differences {
            
            last_value + history_difference.find_next_value()
        } else {
            last_value
        }
    }
}

fn part_1(input_string: &str) -> i64 {
    let inputs: Vec<&str> = input_string.trim().lines().collect();

    let mut histories = Vec::new();

    for input in inputs.into_iter() {
        let input = input.trim();

        let history = input
            .split(' ')
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let history = History::new(history);
        histories.push(history);
    }

    histories.into_iter().map(|x| x.find_next_value()).sum()
}

fn main() {
    let input_string = include_str!("input.txt");

    // Part 1

    let answer = part_1(input_string);
    println!("Part 1: {}", answer);
    assert_eq!(answer, 1930746032);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle() {
        let input_string = r###"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"###;

        assert_eq!(part_1(input_string), 114);
    }
}
