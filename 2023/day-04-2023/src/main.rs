// https://adventofcode.com/2023/day/4
use std::collections::HashSet;

struct Card {
    #[allow(dead_code)]
    card_id: i32,
    winning_numbers: HashSet<i32>,
    numbers: HashSet<i32>,
}

impl Card {
    fn get_points(self) -> i32 {
        let intersection: HashSet<&i32> =
            self.winning_numbers.intersection(&self.numbers).collect();
        if intersection.is_empty() {
            return 0;
        }
        let base: i32 = 2;
        base.pow(intersection.len() as u32 - 1)
    }
}

fn part_1(input_string: &str) -> i32 {
    let inputs: Vec<&str> = input_string.trim().lines().collect();

    let mut cards: Vec<Card> = Vec::new();

    for raw_input in inputs {
        let input: Vec<&str> = raw_input.trim().split(':').collect();
        assert!(input.len() == 2);

        let card_id: i32 = {
            let card_id = input[0].trim();
            card_id
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<i32>()
                .unwrap()
        };

        let list_of_numbers: Vec<&str> = input[1].trim().split('|').collect();
        let winning_numbers: HashSet<i32> = list_of_numbers[0]
            .trim()
            .split(' ')
            .filter(|x| !x.trim().is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let numbers: HashSet<i32> = list_of_numbers[1]
            .trim()
            .split(' ')
            .filter(|x| !x.trim().is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let card = Card {
            card_id,
            winning_numbers,
            numbers,
        };
        cards.push(card);
    }

    cards.into_iter().map(|x| x.get_points()).sum::<i32>()
}

fn main() {
    let input_string = include_str!("input.txt");

    // Part 1

    let answer = part_1(input_string);
    println!("Part 1: {}", answer);
    // assert_eq!(answer, 539637);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle() {
        let input_string = r###"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"###;

        assert_eq!(part_1(input_string), 13);
    }
}
