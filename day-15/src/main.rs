// https://adventofcode.com/2018/day/15

// imports

use std::collections::HashMap;

// code

type Coordinate = (i32, i32);

enum MapState {
    Wall,
    Cavern,
}

type Map = HashMap<Coordinate, MapState>;

fn main() {

    // ensures reading order is satisfied
    assert!((0, 0) < (1, 0));
    assert!((0, 0) < (0, 1));
    assert!((0, 0) < (1, 1));
    assert!((1, 0) < (1, 1));
    assert!((0, 0) < (1, 1));

    let input_string = include_str!("input.txt");

    for (y, line) in input_string.lines().enumerate() {
        for (x, map_state_as_char) in line.chars().enumerate() {
            print!("{}", map_state_as_char);
        }
        println!("");
    }

    // println!("{:?}", input_string);
}
