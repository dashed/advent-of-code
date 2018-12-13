// https://adventofcode.com/2018/day/9

type GameState = Vec<i32>;

fn main() {
    let input_string = include_str!("input.txt");

    // init marble game with the first marble in the circle
    let game_state: GameState = vec![0];

    println!("{}", input_string);
}
