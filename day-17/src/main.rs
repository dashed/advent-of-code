// https://adventofcode.com/2018/day/17

// imports

use std::collections::HashMap;

// code

type Coordinate = (i32, i32);

// position of the water spring
const WATER_SPRING: Coordinate = (500, 0);

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
        // clay can never be right where the water spring is positioned
        assert!(clay_coordinate != &WATER_SPRING);

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
                        if position == WATER_SPRING {
                            row_string.push_str("+");
                        } else {
                            row_string.push_str(".");
                        }
                    }
                    Some(map_state) => {
                        assert!(position != WATER_SPRING);

                        match map_state {
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
                        };
                    }
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

    let clay_coordinates: Vec<Coordinate> =
        input_string.trim().lines().fold(vec![], |mut acc, line| {
            // println!("{}", line);

            let tokens: Vec<&str> = line.split(",").map(|s| s.trim()).collect();

            assert!(tokens.len() == 2);

            let axis: (Option<i32>, Option<i32>) = {
                let parsed_axis: Vec<&str> = tokens[0].split("=").map(|s| s.trim()).collect();
                let axis_str = parsed_axis[0];
                let value: i32 = parsed_axis[1].parse().unwrap();
                match axis_str.as_ref() {
                    "x" => (Some(value), None),
                    "y" => (None, Some(value)),
                    _ => {
                        unreachable!();
                    }
                }
            };

            let range = {
                let parsed_range: Vec<&str> = tokens[1].split("=").map(|s| s.trim()).collect();
                let axis_str = parsed_range[0];
                let range: Vec<i32> = parsed_range[1]
                    .split("..")
                    .map(|s| s.trim())
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect();

                assert!(range.len() == 2);

                range[0]..=range[1]
            };

            for n in range {
                match axis {
                    (None, None) => {
                        unreachable!();
                    }
                    (Some(_), Some(_)) => {
                        unreachable!();
                    }
                    (Some(x), None) => {
                        acc.push((x, n));
                    }
                    (None, Some(y)) => {
                        acc.push((n, y));
                    }
                }
            }

            return acc;
        });

    // add clay to terrain

    let mut map = Map::new();

    for coordinate in clay_coordinates {
        // println!("{:?}", coordinate);
        map.insert_clay(&coordinate);
    }

    println!("max_y: {}", map.max_y());
    println!("min_x: {}", map.min_x());
    println!("max_x: {}", map.max_x());

    println!("{}", map.to_string());
}
