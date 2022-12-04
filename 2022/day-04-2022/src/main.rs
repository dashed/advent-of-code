// https://adventofcode.com/2022/day/4

type Assignments = (i32, i32);

fn parse_assignment(input: String) -> Assignments {
    let assignments: Vec<i32> = input
        .trim()
        .split('-')
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();

    assert!(assignments.len() == 2);

    let start = assignments[0];
    let end = assignments[1];

    assert!(start <= end);

    (start, end)
}

struct Pairs {
    assignments: (Assignments, Assignments),
}

impl Pairs {
    fn new(input: String) -> Self {
        let input: Vec<String> = input
            .trim()
            .split(',')
            .map(|x| -> String { x.to_string() })
            .collect();

        assert!(input.len() == 2);

        let first = parse_assignment(input[0].clone());
        let second = parse_assignment(input[1].clone());

        Pairs {
            assignments: (first, second),
        }
    }

    fn part_1(&self) -> bool {
        let (first, second) = self.assignments;
        let (start_1, end_1) = first;
        let (start_2, end_2) = second;

        if start_2 <= start_1 && end_1 <= end_2 {
            return true;
        }

        if start_1 <= start_2 && end_2 <= end_1 {
            return true;
        }

        false
    }

    fn part_2(&self) -> bool {
        let (first, second) = self.assignments;
        let (start_1, end_1) = first;
        let (start_2, end_2) = second;

        if end_2 < start_1 {
            return false;
        }

        if end_1 < start_2 {
            return false;
        }

        true
    }
}

fn part1(input_string: String) -> usize {
    input_string
        .split_whitespace()
        .map(|x| -> String { x.to_string() })
        .map(|input| -> Pairs { Pairs::new(input) })
        .filter(|pairs| pairs.part_1())
        .count()
}

fn part2(input_string: String) -> usize {
    input_string
        .split_whitespace()
        .map(|x| -> String { x.to_string() })
        .map(|input| -> Pairs { Pairs::new(input) })
        .filter(|pairs| pairs.part_2())
        .count()
}

fn main() {
    let input_string = include_str!("input.txt");

    let part_1 = part1(input_string.to_string());
    println!("Part 1: {}", part_1);
    assert_eq!(part_1, 582);

    let part_2 = part2(input_string.to_string());
    println!("Part 2: {}", part_2);
    assert_eq!(part_2, 893);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input_string = r###"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
    "###
        .trim();

        assert_eq!(part1(input_string.to_string()), 2);
        assert_eq!(part2(input_string.to_string()), 4);
    }
}
