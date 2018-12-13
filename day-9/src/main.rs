// https://adventofcode.com/2018/day/9

type Score = i32;

struct GameState {
    state: Vec<i32>,
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

    fn add_marble() {

    }
}

fn get_position_clockwise(current_position: usize, pos_to_move: usize, num_of_marbles: usize) -> usize {
    return (current_position + pos_to_move) % num_of_marbles;
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
    let game_state: GameState = GameState::new(num_of_players);

    println!("{}", get_position_clockwise(0, 2, 1));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_position_clockwise() {
        assert_eq!(get_position_clockwise(1, 1, 8), 2);
        assert_eq!(get_position_clockwise(7, 1, 8), 0);
    }

}
