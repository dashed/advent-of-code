// https://adventofcode.com/2018/day/20

// imports

use std::collections::HashMap;
use std::collections::HashSet;

// code

/*

Based on https://adriann.github.io/rust_parser.html

Grammar:

branches
    routes | routes
    routes |            (with an empty option)

branch_group
    ( branches )

routes
    branch_group routes
    route routes
    route
    branch_group

route
    direction route
    direction

direction:
    N, S, W, E

start := ^
end := $
input -> start routes end

*/

#[derive(Debug)]
enum GrammarItem {
    Start,
    End,
    OpenDirection(OpenDirections),
    ParenOpen,
    ParenClose,
    BranchOr
}

type Distance = i32;

#[derive(Debug)]
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

    // the fewest number of doors to pass through to reach room defined by Coordinate
    room_distance: HashMap<Coordinate, Distance>,
}

// invariant: the routes will take you through every door in the facility at least once

impl Map {
    fn new() -> Self {
        Map {
            map: HashMap::new(),

            room_distance: HashMap::new(),
        }
    }

    fn parse_input(&self, input_string: &str) {
        let input_string = input_string.trim();

        if input_string.len() <= 2 {
            return;
        }

        let mut iter = input_string.trim().chars();

        // start
        assert!(iter.next().unwrap() == '^');

        for direction in input_string.trim().chars() {
            println!("{}", direction);

            if direction == '$' {
                break;
            }
        }
    }

    // parse route starting from the current position
    fn parse_route(&self, current_position: Coordinate) {}
}

fn main() {
    let input_string = include_str!("input.txt");

    let map = Map::new();

    map.parse_input(input_string);
}
