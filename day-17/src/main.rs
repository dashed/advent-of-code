// https://adventofcode.com/2018/day/17

// imports

use std::collections::HashMap;

// code

type Coordinate = (i32, i32);

trait Transitions {
    fn down(&self) -> Coordinate;
    fn left(&self) -> Coordinate;
    fn right(&self) -> Coordinate;
}

impl Transitions for Coordinate {
    fn down(&self) -> Coordinate {
        let (x, y) = self;
        return (*x, y + 1);
    }

    fn left(&self) -> Coordinate {
        let (x, y) = self;
        return (x - 1, *y);
    }

    fn right(&self) -> Coordinate {
        let (x, y) = self;
        return (x + 1, *y);
    }
}

enum Water {
    AtRest,
    Reachable,
}

enum MapState {
    Clay,
    Sand,
    Water(Water),
}

type Terrain = HashMap<Coordinate, MapState>;

fn main() {
    let input_string = include_str!("input.txt");

    println!("{}", input_string);
}
