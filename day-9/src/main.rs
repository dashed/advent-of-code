// https://adventofcode.com/2018/day/9

type Score = usize;

struct GameState {
    state: Vec<usize>,
    // number of marbles added
    num_of_marbles_added: usize,
    pos_of_current_marble: usize,
    players: Vec<Score>,
}

impl GameState {
    fn new(num_of_players: usize) -> GameState {
        let mut players = Vec::with_capacity(num_of_players);

        for _ in 1..=num_of_players {
            // each player has an initial score of 0
            players.push(0);
        }

        GameState {
            // initially, the marble numbered 0 is placed within the circle
            state: vec![0],
            num_of_marbles_added: 1,
            pos_of_current_marble: 0,
            players,
        }
    }

    fn get_position_clockwise(&self, pos_to_move: usize) -> usize {
        // get vector position that is pos_to_move to the right (i.e. clockwise) of pos_of_current_marble
        return get_position_clockwise(self.pos_of_current_marble, pos_to_move, self.state.len());
    }

    fn get_position_counter_clockwise(&self, pos_to_move: usize) -> usize {
        // get vector position that is pos_to_move to the left (i.e. clockwise) of pos_of_current_marble
        return get_position_counter_clockwise(
            self.pos_of_current_marble,
            pos_to_move,
            self.state.len(),
        );
    }

    fn max_score(&self) -> Score {
        return *self.players.iter().max().unwrap();
    }

    fn add_marble(&mut self) {
        let value_of_next_marble = self.num_of_marbles_added;

        // zero-based index
        let current_player = (value_of_next_marble - 1) % self.players.len();

        if (value_of_next_marble % 23) == 0 {
            // add new marble to current player's score
            self.players[current_player] += value_of_next_marble;

            // the marble 7 marbles counter-clockwise from the current marble is removed from the circle
            // and also added to the current player's score.
            let pos_to_remove = self.get_position_counter_clockwise(7);
            let removed_marble = self.state.remove(pos_to_remove);
            self.players[current_player] += removed_marble;

            // The marble located immediately clockwise of the marble that was removed becomes the new current marble.
            self.pos_of_current_marble = pos_to_remove % self.state.len();

            self.num_of_marbles_added += 1;
            return;
        }

        // get position to add new marble into
        let new_position = (self.get_position_clockwise(1) + 1) % (self.state.len() + 1);

        // update game state

        self.pos_of_current_marble = new_position;
        self.num_of_marbles_added += 1;
        self.state.insert(new_position, value_of_next_marble);
    }
}

// ref: http://yourdailygeekery.com/2011/06/28/modulo-of-negative-numbers.html
fn modulo(a: i32, b: usize) -> usize {
    let b = b as i32;
    (((a % b) + b) % b) as usize
}

fn get_position_clockwise(
    current_position: usize,
    pos_to_move: usize,
    num_of_marbles: usize,
) -> usize {
    assert!(current_position < num_of_marbles);
    let new_position: i32 = (current_position as i32) + (pos_to_move as i32);
    return modulo(new_position, num_of_marbles);
}

fn get_position_counter_clockwise(
    current_position: usize,
    pos_to_move: usize,
    num_of_marbles: usize,
) -> usize {
    assert!(current_position < num_of_marbles);
    let new_position: i32 = (current_position as i32) - (pos_to_move as i32);
    return modulo(new_position, num_of_marbles);
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

fn main() {
    let input_string = include_str!("input.txt");

    let max_score = part_1(input_string);

    println!("Part 1: {:?}", max_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_position_clockwise() {
        assert_eq!(get_position_clockwise(0, 1, 1), 0);
        assert_eq!(get_position_clockwise(1, 1, 8), 2);
        assert_eq!(get_position_clockwise(7, 1, 8), 0);
        assert_eq!(get_position_clockwise(12, 1, 13), 0);
    }

    #[test]
    fn test_get_position_counter_clockwise() {
        assert_eq!(get_position_counter_clockwise(12, 1, 13), 11);
        assert_eq!(get_position_counter_clockwise(0, 1, 13), 12);
    }

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
