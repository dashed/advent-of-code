// https://adventofcode.com/2023/day/4
use std::collections::HashMap;
use std::{cmp, collections::HashSet};

#[derive(Debug, Clone)]
struct Card {
    #[allow(dead_code)]
    card_id: i32,
    winning_numbers: HashSet<i32>,
    numbers: HashSet<i32>,
}

impl Card {
    fn get_points(&self) -> i32 {
        let intersection: HashSet<&i32> =
            self.winning_numbers.intersection(&self.numbers).collect();
        if intersection.is_empty() {
            return 0;
        }
        let base: i32 = 2;
        base.pow(intersection.len() as u32 - 1)
    }

    fn get_num_of_winning_cards(&self) -> i32 {
        let intersection: HashSet<&i32> =
            self.winning_numbers.intersection(&self.numbers).collect();
        intersection.len() as i32
    }
}

fn part_1(cards: Vec<Card>) -> i32 {
    cards.into_iter().map(|x| x.get_points()).sum::<i32>()
}

fn part_2(cards: Vec<Card>) -> i32 {
    let cards_len = cards.len();
    let mut total_num_of_cards = 0;
    let mut copies_map: HashMap<i32, i32> = HashMap::new();

    for (current_card_index, current_card) in cards.into_iter().enumerate() {
        // count original card
        total_num_of_cards += 1;

        // count copies
        let num_of_copies = *copies_map.get(&(current_card_index as i32)).unwrap_or(&0);

        total_num_of_cards += num_of_copies;

        let num_of_winning_cards = current_card.get_num_of_winning_cards();

        if num_of_winning_cards == 0 {
            continue;
        }

        let start_range = current_card_index + 1;
        let max_range = current_card_index + num_of_winning_cards as usize;
        let max_range = cmp::min(max_range, cards_len - 1);

        if start_range > max_range {
            break;
        }

        let created_num_of_copies = num_of_copies + 1;

        // do this for the original card and for each copy
        for card_copy_index in start_range..=max_range {
            copies_map
                .entry(card_copy_index as i32)
                .and_modify(|counter| *counter += created_num_of_copies)
                .or_insert(created_num_of_copies);
        }
    }

    total_num_of_cards
}

#[allow(dead_code)]
fn part_2_naive(cards: Vec<Card>) -> i32 {
    let mut total_num_of_cards = cards.len() as i32;

    let mut card_buffer: Vec<(usize, Card)> = cards.clone().into_iter().enumerate().collect();

    loop {
        if card_buffer.is_empty() {
            break;
        }
        let (current_card_index, current_card) = card_buffer.remove(0);
        let num_of_winning_cards = current_card.get_num_of_winning_cards();

        if num_of_winning_cards == 0 {
            continue;
        }

        let start_range = current_card_index + 1;

        let max_range = current_card_index + num_of_winning_cards as usize;
        let max_range = cmp::min(max_range, cards.len() - 1);

        if start_range > max_range {
            break;
        }

        let num_of_winning_cards = max_range - start_range + 1;

        total_num_of_cards += num_of_winning_cards as i32;

        let range = start_range..=max_range;
        for (card_copy_index, card_copy) in cards[range].iter().enumerate() {
            let card_copy = card_copy.clone();
            card_buffer.push((start_range + card_copy_index, card_copy));
        }
    }
    total_num_of_cards
}

fn process_input(input_string: &str) -> Vec<Card> {
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
    cards
}

fn main() {
    let input_string = include_str!("input.txt");
    let cards = process_input(input_string);

    // Part 1

    let answer = part_1(cards.clone());
    println!("Part 1: {}", answer);
    assert_eq!(answer, 26914);

    // Part 2

    let answer = part_2(cards);
    println!("Part 2: {}", answer);
    assert_eq!(answer, 13080971);
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

        let cards = process_input(input_string);

        assert_eq!(part_1(cards.clone()), 13);
        assert_eq!(part_2(cards), 30);
    }
}
