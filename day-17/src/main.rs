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

    fn max_y(&self) -> i32 {
        return self
            .terrain
            .iter()
            .map(|item| {
                let (coord, _map_state) = item;
                let (_x, y) = coord;
                return *y;
            })
            .max()
            .unwrap();
    }

    fn min_x(&self) -> i32 {
        return self
            .terrain
            .iter()
            .map(|item| {
                let (coord, _map_state) = item;
                let (x, _y) = coord;
                return *x;
            })
            .min()
            .unwrap();
    }

    fn max_x(&self) -> i32 {
        return self
            .terrain
            .iter()
            .map(|item| {
                let (coord, _map_state) = item;
                let (x, _y) = coord;
                return *x;
            })
            .max()
            .unwrap();
    }

    fn insert_clay(&mut self, clay_coordinate: &Coordinate) {
        self.terrain.insert(*clay_coordinate, MapState::Clay);
    }

    fn to_string(&self) -> String {
        let max_y = self.max_y();
        let min_x = self.min_x();
        let max_x = self.max_x();

        let mut map_string: Vec<String> = vec![];

        for y in 0..=max_y {
            let mut row_string = String::from("");

            for x in min_x..=max_x {
                let position = (x, y);

                match self.terrain.get(&position) {
                    None => {
                        row_string.push_str(".");
                    }
                    Some(map_state) => match map_state {
                        MapState::Clay => {
                            row_string.push_str("#");
                        }
                        MapState::Sand => {
                            row_string.push_str(".");
                        }
                        MapState::Water(water) => match water {
                            Water::AtRest => {
                                row_string.push_str("~");
                            }
                            Water::Reachable => {
                                row_string.push_str("|");
                            }
                        },
                    },
                }
            }

            map_string.push(row_string);
        }

        return map_string.join("\n");
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    // parse positions of clay

    let clay_coordinates: Vec<_> = input_string
        .trim()
        .lines()
        .map(|line| {
            // println!("{}", line);

            let tokens: Vec<&str> = line.split(",").map(|s| s.trim()).collect();

            assert!(tokens.len() == 2);

            let axis: i32 = tokens[0].parse().unwrap();
            let range: Vec<i32> = {
                let range = tokens[1];
                range
                    .split("..")
                    .map(|s| s.trim())
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect()
            };

            println!("{:?}", tokens);

            return line;

            // let target = line.split("..").next().unwrap().trim();

            // let (x, y) = target
            //     .split(",")
            //     .map(|s| s.trim())
            //     .fold((None, None), |acc, s| {
            //         let (x, y) = acc;

            //         let mut s_iter = s.trim().split("=").map(|s| s.trim());

            //         let identifier = s_iter.next().unwrap().to_lowercase();
            //         let value: i32 = s_iter.next().map(|s| s.parse::<i32>().unwrap()).unwrap();

            //         match identifier.as_ref() {
            //             "x" => {
            //                 return (Some(value), y);
            //             }
            //             "y" => {
            //                 return (x, Some(value));
            //             }
            //             _ => {
            //                 unreachable!();
            //             }
            //         }
            //     });

            // let coord: Coordinate = (x.unwrap(), y.unwrap());

            // return coord;
        })
        .collect();

    // add clay to terrain

    // let mut map = Map::new();

    // let terrain = HashMap::new();

    // for coordinate in clay_coordinates {
    //     // println!("{:?}", coordinate);
    //     map.insert_clay(&coordinate);
    // }

    // println!("max_y: {}", map.max_y());
    // println!("min_x: {}", map.min_x());
    // println!("max_x: {}", map.max_x());

    // println!("{}", map.to_string());
}
