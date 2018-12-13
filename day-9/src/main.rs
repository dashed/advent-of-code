// https://adventofcode.com/2018/day/9

type Score = usize;

struct GameState {
    state: Vec<usize>,
    // number of marbles added to the circle
    num_of_marbles: usize,
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
            num_of_marbles: 1,
            pos_of_current_marble: 0,
            players,
        }
    }

    fn get_position_clockwise(&self, pos_to_move: usize) -> usize {
        // get vector position that is pos_to_move to the right (i.e. clockwise) of pos_of_current_marble
        return get_position_clockwise(self.pos_of_current_marble, pos_to_move, self.num_of_marbles);
    }

    fn max_score(&self) -> Score {
        return *self.players.iter().max().unwrap();
    }

    fn add_marble(&mut self) {
        let value_of_next_marble = self.num_of_marbles;

        // zero-based index
        let current_player = (value_of_next_marble - 1) % self.players.len();

        if value_of_next_marble % 23 == 0 {
            // add new marble to current player's score
            self.players[current_player] += value_of_next_marble;

            // TODO: the marble 7 marbles counter-clockwise from the current marble is removed from the circle
            // and also added to the current player's score.

            // TODO: The marble located immediately clockwise of the marble that was removed becomes the new current marble.
            return;
        }

        let new_num_of_marbles = self.num_of_marbles + 1;

        // get position to add new marble into
        let new_position = (self.get_position_clockwise(1) + 1) % new_num_of_marbles;

        // update game state

        self.num_of_marbles += 1;
        self.pos_of_current_marble = new_position;

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

fn main() {
    let input_string = include_str!("input.txt");

    let (num_of_players, nth_marble): (usize, i32) = {
        let inputs: Vec<&str> = input_string.trim().split_whitespace().collect();

        (
            inputs.get(0).unwrap().parse().unwrap(),
            inputs.get(6).unwrap().parse().unwrap(),
        )
    };

    println!("num_of_players: {}", num_of_players);
    println!("nth_marble: {}", nth_marble);

    // init marble game with the first marble in the circle
    let mut game_state: GameState = GameState::new(num_of_players);

    for _ in 0..9 {
        println!(
            "{:?}",
            game_state
                .state
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("  ")
        );
        game_state.add_marble();
    }

    println!("{}", get_position_clockwise(0, 2, 1));
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

}
