// https://adventofcode.com/2018/day/9

struct GameState(Vec<i32>, /* current marble */ usize);

impl GameState {
    fn new() -> GameState {
        GameState(vec![0], 0)
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    // init marble game with the first marble in the circle
    let game_state: GameState = GameState::new();

    println!("{}", input_string);
}
