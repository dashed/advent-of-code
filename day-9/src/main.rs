// https://adventofcode.com/2018/day/9

use std::collections::VecDeque;

type Score = usize;

struct GameState {
    state: VecDeque<usize>,
    // number of marbles added
    num_of_marbles_added: usize,
    players: Vec<Score>,
}

impl GameState {
    fn new(num_of_players: usize) -> GameState {
        let mut players = Vec::with_capacity(num_of_players);

        for _ in 1..=num_of_players {
            // each player has an initial score of 0
            players.push(0);
        }

        // the head of the ring buffer is always the current position
        let mut state = VecDeque::new();
        state.push_front(0);

        GameState {
            // initially, the marble numbered 0 is placed within the circle
            state,
            num_of_marbles_added: 1,
            players,
        }
    }

    fn max_score(&self) -> Score {
        return *self.players.iter().max().unwrap();
    }

    fn move_clockwise(&mut self, amount: usize) {
        for _ in 1..=amount {
            // remove first item and add it to the end of the ring buffer
            let first = self.state.pop_front().unwrap();
            self.state.push_back(first);
        }
    }

    fn move_counter_clockwise(&mut self, amount: usize) {
        for _ in 1..=amount {
            // remove the last item and add it to the front of the ring buffer
            let x = self.state.pop_back().unwrap();
            self.state.push_front(x);
        }
    }

    fn add_marble(&mut self) {
        let value_of_next_marble = self.num_of_marbles_added;

        if (value_of_next_marble % 23) == 0 {
            // zero-based index
            let current_player = (value_of_next_marble - 1) % self.players.len();

            // add new marble to current player's score
            self.players[current_player] += value_of_next_marble;

            // the marble 7 marbles counter-clockwise from the current marble is removed from the circle
            // and also added to the current player's score.
            self.move_counter_clockwise(7);
            let removed_marble = self.state.pop_front().unwrap();
            self.players[current_player] += removed_marble;

            // The marble located immediately clockwise of the marble that was removed becomes the new current marble.

            self.num_of_marbles_added += 1;
            return;
        }

        // update game state

        self.move_clockwise(2);
        self.state.push_front(value_of_next_marble);
        self.num_of_marbles_added += 1;
    }
}

fn part_1(input_string: &str) -> Score {
    let (num_of_players, nth_marble): (usize, i32) = {
        let inputs: Vec<&str> = input_string.trim().split_whitespace().collect();

        (
            inputs.get(0).unwrap().parse().unwrap(),
            inputs.get(6).unwrap().parse().unwrap(),
        )
    };

    // init marble game with the first marble in the circle
    let mut game_state: GameState = GameState::new(num_of_players);

    for _idx in 1..=nth_marble {
        game_state.add_marble();
    }

    return game_state.max_score();
}

fn part_2(input_string: &str) -> Score {
    let (num_of_players, nth_marble): (usize, i32) = {
        let inputs: Vec<&str> = input_string.trim().split_whitespace().collect();

        (
            inputs.get(0).unwrap().parse().unwrap(),
            inputs.get(6).unwrap().parse().unwrap(),
        )
    };

    // init marble game with the first marble in the circle
    let mut game_state: GameState = GameState::new(num_of_players);

    for _idx in 1..=(nth_marble * 100) {
        game_state.add_marble();
    }

    return game_state.max_score();
}

fn main() {
    let input_string = include_str!("input.txt");

    let max_score = part_1(input_string);
    println!("Part 1: {:?}", max_score);

    let max_score = part_2(input_string);
    println!("Part 2: {:?}", max_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("10 players; last marble is worth 1618 points"), 8317);
        assert_eq!(
            part_1("13 players; last marble is worth 7999 points"),
            146373
        );
        assert_eq!(part_1("17 players; last marble is worth 1104 points"), 2764);
        assert_eq!(
            part_1("21 players; last marble is worth 6111 points"),
            54718
        );
        assert_eq!(
            part_1("30 players; last marble is worth 5807 points"),
            37305
        );
    }

}
