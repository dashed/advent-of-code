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

struct Map {
    terrain: Terrain,
}

impl Map {
    fn new() -> Self {
        Map {
            terrain: HashMap::new(),
        }
    }

    fn insert_clay(&mut self, clay_coordinate: &Coordinate) {
        self.terrain.insert(*clay_coordinate, MapState::Clay);
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    // parse positions of clay

    let clay_coordinates: Vec<Coordinate> = input_string
        .trim()
        .lines()
        .map(|line| {
            let target = line.split("..").next().unwrap().trim();
            let coord: Vec<i32> = target
                .split(",")
                .map(|s| {
                    return s.trim().split("=").skip(1).next().unwrap().trim();
                })
                .map(|s| {
                    return s.parse::<i32>().unwrap();
                })
                .collect();

            assert!(coord.len() == 2);

            let coord: Coordinate = (coord[0], coord[1]);

            return coord;
        })
        .collect();

    // add clay to terrain

    let mut map = Map::new();

    // let terrain = HashMap::new();

    for coordinate in clay_coordinates {
        // println!("{:?}", coordinate);
        map.insert_clay(&coordinate);
    }

    // println!("{}", input_string);
}
