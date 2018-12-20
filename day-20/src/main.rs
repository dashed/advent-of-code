// https://adventofcode.com/2018/day/20

// imports

use std::collections::HashMap;
use std::collections::HashSet;

// code

enum OpenDirections {
    North,
    South,
    West,
    East,
}

impl OpenDirections {
    fn from_char(d: char) -> Self {
        match d {
            'N' => OpenDirections::North,
            'S' => OpenDirections::South,
            'W' => OpenDirections::West,
            'E' => OpenDirections::East,
            _ => {
                unreachable!();
            }
        }
    }
}

type Coordinate = (i32, i32);

trait Transitions {
    fn north(&self) -> Coordinate;
    fn south(&self) -> Coordinate;
    fn west(&self) -> Coordinate;
    fn east(&self) -> Coordinate;
}

impl Transitions for Coordinate {
    fn north(&self) -> Coordinate {
        let (x, y) = self;
        return (*x, y - 1);
    }

    fn south(&self) -> Coordinate {
        let (x, y) = self;
        return (*x, y + 1);
    }

    fn west(&self) -> Coordinate {
        let (x, y) = self;
        return (x - 1, *y);
    }

    fn east(&self) -> Coordinate {
        let (x, y) = self;
        return (x + 1, *y);
    }
}

struct Map {
    // a coordinate indicates a room;
    // and from this room indicates possible directions to go into
    map: HashMap<Coordinate, HashSet<OpenDirections>>,
}

fn main() {
    let input_string = include_str!("input.txt");

    println!("{}", input_string);
}
