// https://adventofcode.com/2022/day/2

use std::cmp::Ordering;
use std::vec;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn get_moves() -> Vec<Move> {
        vec![Move::Rock, Move::Paper, Move::Scissors]
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Move::Rock, Move::Rock) => Ordering::Equal,
            (Move::Rock, Move::Paper) => Ordering::Less,
            (Move::Rock, Move::Scissors) => Ordering::Greater,

            (Move::Paper, Move::Rock) => Ordering::Greater,
            (Move::Paper, Move::Paper) => Ordering::Equal,
            (Move::Paper, Move::Scissors) => Ordering::Less,

            (Move::Scissors, Move::Rock) => Ordering::Less,
            (Move::Scissors, Move::Paper) => Ordering::Greater,
            (Move::Scissors, Move::Scissors) => Ordering::Equal,
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Game {
    opponent_move: Move,
    my_move: Move,
}

impl Game {
    fn get_my_move_score(&self) -> i32 {
        // 1 for Rock, 2 for Paper, and 3 for Scissors
        match self.my_move {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn get_score_part_1(&self) -> i32 {
        let my_move_score = self.get_my_move_score();

        // 0 if you lost, 3 if the round was a draw, and 6 if you won
        let outcome = match self.my_move.cmp(&self.opponent_move) {
            Ordering::Equal => 3,
            Ordering::Less => 0,
            Ordering::Greater => 6,
        };

        my_move_score + outcome
    }
}

struct Tournament {
    rounds: Vec<Game>,
}

impl Tournament {
    fn get_total_scores(&self) -> i32 {
        self.rounds
            .iter()
            .map(|round| -> i32 { round.get_score_part_1() })
            .sum()
    }
}

impl Tournament {
    fn parse_opponent_move(input: &str) -> Move {
        // A for Rock, B for Paper, and C for Scissors
        match input {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => {
                unreachable!()
            }
        }
    }

    fn parse_round_input_part_1(&mut self, input: String) {
        let input: Vec<&str> = input.split_whitespace().collect();
        assert!(input.len() == 2);

        // A for Rock, B for Paper, and C for Scissors
        let opponent_move = Tournament::parse_opponent_move(input[0]);

        // X for Rock, Y for Paper, and Z for Scissors
        let my_move = match input[1] {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => {
                unreachable!()
            }
        };

        let round = Game {
            opponent_move,
            my_move,
        };

        self.rounds.push(round);
    }

    fn parse_round_input_part_2(&mut self, input: String) {
        let input: Vec<&str> = input.split_whitespace().collect();
        assert!(input.len() == 2);

        // A for Rock, B for Paper, and C for Scissors
        let opponent_move = Tournament::parse_opponent_move(input[0]);

        // X means you need to lose,
        // Y means you need to end the round in a draw,
        // and Z means you need to win
        let my_move = match input[1] {
            "X" => Move::get_moves()
                .into_iter()
                .find(|possible_move| possible_move.cmp(&opponent_move) == Ordering::Less)
                .unwrap(),
            "Y" => opponent_move.clone(),
            "Z" => Move::get_moves()
                .into_iter()
                .find(|possible_move| possible_move.cmp(&opponent_move) == Ordering::Greater)
                .unwrap(),
            _ => {
                unreachable!()
            }
        };

        let round = Game {
            opponent_move,
            my_move,
        };

        self.rounds.push(round);
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    let inputs: Vec<&str> = input_string.trim().split('\n').collect();

    {
        // Part 1
        let mut tournament = Tournament { rounds: vec![] };

        for input in inputs.clone() {
            let input = input.trim();
            if input.is_empty() {
                continue;
            }

            tournament.parse_round_input_part_1(input.to_string());
        }

        let tournament = tournament;

        println!("Part 1: {}", tournament.get_total_scores());
        assert!(tournament.get_total_scores() == 14163);
    };

    {
        // Part 2
        let mut tournament = Tournament { rounds: vec![] };

        for input in inputs {
            let input = input.trim();
            if input.is_empty() {
                continue;
            }

            tournament.parse_round_input_part_2(input.to_string());
        }

        let tournament = tournament;

        println!("Part 2: {}", tournament.get_total_scores());
        assert!(tournament.get_total_scores() == 12091);
    };
}
