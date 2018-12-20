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

// invariant: the routes will take you through every door in the facility at least once

impl Map {
    fn new() -> Self {
        Map {
            map: HashMap::new(),
        }
    }

    fn parse_input(&self, input_string: &str) {
        println!("{}", input_string);
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    let map = Map::new();

    map.parse_input(input_string);
}
