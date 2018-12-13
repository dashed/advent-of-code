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

    println!("{}", input_string);
}
