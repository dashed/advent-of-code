use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Card {
    label: char,
}

impl Card {
    fn get_strength(&self) -> i64 {
        // A hand consists of five cards labeled one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2.
        // The relative strength of each card follows this order, where A is the highest and 2 is the lowest.
        let strength = [
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ];
        strength.iter().position(|&x| x == self.label).unwrap() as i64 + 1
    }

    fn part_2_get_strength(&self) -> i64 {
        let strength = [
            'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
        ];
        strength.iter().position(|&x| x == self.label).unwrap() as i64 + 1
    }
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<Card>,
    bid_amount: i64,
}

impl Hand {
    fn from_string(input: &str) -> Hand {
        assert!(input.len() == 5);
        let cards = input
            .chars()
            .map(|x| Card { label: x })
            .collect::<Vec<Card>>();

        assert!(cards.len() == 5);
        let bid_amount = 0;

        Hand { cards, bid_amount }
    }

    #[allow(dead_code)]
    fn to_display(&self) -> String {
        let cards: String = self.cards.iter().map(|x| x.label).collect::<String>();

        format!("{} {}", cards, self.bid_amount)
    }

    fn to_hand_string(&self) -> String {
        self.cards.iter().map(|x| x.label).collect::<String>()
    }

    fn get_strength(&self) -> i64 {
        if self.is_five_of_a_kind() {
            return 7;
        }
        if self.is_four_of_a_kind() {
            return 6;
        }
        if self.is_full_house() {
            return 5;
        }
        if self.is_three_of_a_kind() {
            return 4;
        }
        if self.is_two_pair() {
            return 3;
        }
        if self.is_one_pair() {
            return 2;
        }

        // High card, where all cards' labels are distinct: 23456

        let label_counts = self.get_label_counts();
        assert!(label_counts.len() == 5);
        1
    }

    fn get_label_counts(&self) -> HashMap<char, i64> {
        let mut label_counts: HashMap<char, i64> = HashMap::new();
        for card in self.cards.iter() {
            let label_count = label_counts.entry(card.label).or_insert(0);
            *label_count += 1;
        }
        label_counts
    }

    fn is_five_of_a_kind(&self) -> bool {
        // Five of a kind, where all five cards have the same label: AAAAA
        let label_counts = self.get_label_counts();
        if label_counts.len() != 1 {
            return false;
        }
        let (_, count) = label_counts.iter().next().unwrap();
        *count == 5
    }

    fn is_four_of_a_kind(&self) -> bool {
        // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
        let label_counts = self.get_label_counts();
        if label_counts.len() != 2 {
            return false;
        }
        for (_, count) in label_counts.iter() {
            let valid = *count == 4 || *count == 1;
            if !valid {
                return false;
            }
        }
        true
    }

    fn is_full_house(&self) -> bool {
        // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
        let label_counts = self.get_label_counts();
        if label_counts.len() != 2 {
            return false;
        }
        for (_, count) in label_counts.iter() {
            let valid = *count == 3 || *count == 2;
            if !valid {
                return false;
            }
        }
        true
    }

    fn is_three_of_a_kind(&self) -> bool {
        // Three of a kind, where three cards have the same label, and the remaining two cards are each different from
        // any other card in the hand: TTT98
        let label_counts = self.get_label_counts();
        if label_counts.len() != 3 {
            return false;
        }
        for (_, count) in label_counts.iter() {
            let valid = *count == 3 || *count == 1;
            if !valid {
                return false;
            }
        }
        true
    }

    fn is_two_pair(&self) -> bool {
        // Two pair, where two cards share one label, two other cards share a second label, and the remaining card
        // has a third label: 23432
        let label_counts = self.get_label_counts();
        if label_counts.len() != 3 {
            return false;
        }
        for (_, count) in label_counts.iter() {
            let valid = *count == 2 || *count == 1;
            if !valid {
                return false;
            }
        }
        true
    }

    fn is_one_pair(&self) -> bool {
        // One pair, where two cards share one label, and the other three cards have a different label from the pair
        // and each other: A23A4
        let label_counts = self.get_label_counts();
        if label_counts.len() != 4 {
            return false;
        }
        for (_, count) in label_counts.iter() {
            let valid = *count == 2 || *count == 1;
            if !valid {
                return false;
            }
        }
        true
    }

    fn is_stronger_than(&self, other_hand: &Hand) -> bool {
        let self_strength = self.get_strength();
        let other_strength = other_hand.get_strength();
        if self_strength != other_strength {
            return self_strength > other_strength;
        }

        // If two hands have the same type, a second ordering rule takes effect.
        // Start by comparing the first card in each hand. If these cards are different, the hand with the stronger
        // first card is considered stronger. If the first card in each hand have the same label, however, then move on
        // to considering the second card in each hand. If they differ, the hand with the higher second card wins;
        // otherwise, continue with the third card in each hand, then the fourth, then the fifth.

        for (self_card, other_card) in self.cards.iter().zip(other_hand.cards.iter()) {
            if self_card.label == other_card.label {
                continue;
            }
            let self_card_strength = self_card.get_strength();
            let other_card_strength = other_card.get_strength();

            return self_card_strength > other_card_strength;
        }

        false
    }

    fn to_strongest_card(&self) -> Hand {
        let label_counts = self.get_label_counts();
        let self_has_joker = label_counts.contains_key(&'J');

        let self_hand = self.clone();

        if !self_has_joker {
            return self_hand;
        }

        if label_counts.len() == 1 {
            let mut card_form = Hand::from_string("AAAAA");
            card_form.bid_amount = self_hand.bid_amount;
            return card_form;
        }

        let mut card_forms = vec![];

        for (label, _count) in label_counts.iter() {
            if label == &'J' {
                continue;
            }
            let hand_string = self_hand.to_hand_string().replace('J', &label.to_string());

            let mut card_form = Hand::from_string(&hand_string);
            card_form.bid_amount = self_hand.bid_amount;

            card_forms.push(card_form);
        }

        if card_forms.is_empty() {
            panic!("No card forms found: {:?}", self_hand);
        }

        card_forms
            .into_iter()
            .max_by(|a, b| {
                if b.is_stronger_than(a) {
                    return std::cmp::Ordering::Less;
                }
                if a.is_stronger_than(b) {
                    return std::cmp::Ordering::Greater;
                }
                std::cmp::Ordering::Equal
            })
            .unwrap()
    }

    fn is_stronger_than_part_2(&self, other_hand: &Hand) -> bool {
        let self_hand = self.to_strongest_card();
        let strong_other_hand = other_hand.to_strongest_card();

        let self_strength = self_hand.get_strength();
        let other_strength = strong_other_hand.get_strength();
        if self_strength != other_strength {
            return self_strength > other_strength;
        }

        // If two hands have the same type, a second ordering rule takes effect.
        // Start by comparing the first card in each hand. If these cards are different, the hand with the stronger
        // first card is considered stronger. If the first card in each hand have the same label, however, then move on
        // to considering the second card in each hand. If they differ, the hand with the higher second card wins;
        // otherwise, continue with the third card in each hand, then the fourth, then the fifth.

        for (self_card, other_card) in self.cards.iter().zip(other_hand.cards.iter()) {
            if self_card.label == other_card.label {
                continue;
            }
            let self_card_strength = self_card.part_2_get_strength();
            let other_card_strength = other_card.part_2_get_strength();

            return self_card_strength > other_card_strength;
        }

        false
    }
}

fn part_1(input_string: &str) -> i64 {
    let inputs: Vec<&str> = input_string.trim().lines().collect();

    let mut hands: Vec<Hand> = Vec::new();

    for input in inputs {
        let input = input.trim();
        let inputs = input
            .split(' ')
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();
        assert!(inputs.len() == 2);

        let cards = inputs[0]
            .chars()
            .map(|x| Card { label: x })
            .collect::<Vec<Card>>();
        let bid_amount = inputs[1].parse::<i64>().unwrap();

        assert!(cards.len() == 5);

        let hand = Hand { cards, bid_amount };

        hands.push(hand);
    }

    hands.sort_by(|a, b| {
        if b.is_stronger_than(a) {
            return std::cmp::Ordering::Less;
        }
        if a.is_stronger_than(b) {
            return std::cmp::Ordering::Greater;
        }
        panic!("Hands are equal: {:?} {:?}", a, b);
        // std::cmp::Ordering::Equal
    });

    // for hand in &hands {
    //     println!("{}", hand.to_str());
    // }

    hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| {
            let rank = i + 1;
            hand.bid_amount * rank as i64
        })
        .sum::<i64>()
}

fn part_2(input_string: &str) -> i64 {
    let inputs: Vec<&str> = input_string.trim().lines().collect();

    let mut hands: Vec<Hand> = Vec::new();

    for input in inputs {
        let input = input.trim();
        let inputs = input
            .split(' ')
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();
        assert!(inputs.len() == 2);

        let cards = inputs[0]
            .chars()
            .map(|x| Card { label: x })
            .collect::<Vec<Card>>();
        let bid_amount = inputs[1].parse::<i64>().unwrap();

        assert!(cards.len() == 5);

        let hand = Hand { cards, bid_amount };

        hands.push(hand);
    }

    hands.sort_by(|a, b| {
        if b.is_stronger_than_part_2(a) {
            return std::cmp::Ordering::Less;
        }
        if a.is_stronger_than_part_2(b) {
            return std::cmp::Ordering::Greater;
        }
        std::cmp::Ordering::Equal
    });

    hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| {
            let rank = i + 1;
            hand.bid_amount * rank as i64
        })
        .sum::<i64>()
}

fn main() {
    let input_string = include_str!("input.txt");

    // Part 1

    let answer = part_1(input_string);
    println!("Part 1: {}", answer);
    assert_eq!(answer, 250474325);

    // Part 2

    let answer = part_2(input_string);
    println!("Part 2: {}", answer);
    assert_eq!(answer, 248909434);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand() {
        assert!(Hand::from_string("KK677").is_stronger_than(&Hand::from_string("KTJJT")));
        assert!(Hand::from_string("QQQJA").is_stronger_than(&Hand::from_string("T55J5")));
        assert!(Hand::from_string("33332").is_stronger_than(&Hand::from_string("2AAAA")));
        assert!(Hand::from_string("54321").is_stronger_than(&Hand::from_string("52346")));
        assert!(Hand::from_string("6K854").is_stronger_than(&Hand::from_string("43825")));
    }

    #[test]
    fn test_puzzle() {
        let input_string = r###"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"###;

        assert_eq!(part_1(input_string), 6440);
        assert_eq!(part_2(input_string), 5905);

        let input_string = r###"
AAAAA 2
22222 3
AAAAK 5
22223 7
AAAKK 11
22233 13
AAAKQ 17
22234 19
AAKKQ 23
22334 29
AAKQJ 31
22345 37
AKQJT 41
23456 43
"###;

        assert_eq!(part_1(input_string), 1343);
        assert_eq!(part_2(input_string), 1369);

        let input_string = r###"
6K854 1
43825 1
"###;

        assert_eq!(part_1(input_string), 3);

        let input_string = r###"
43825 2
6K854 3
"###;

        assert_eq!(part_1(input_string), 8);
    }
}
