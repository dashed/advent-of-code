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
    Flowing,
}

enum MapState {
    Clay,
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

    fn is_coord_out_of_bounds(&self, position: &Coordinate) -> bool {
        let (x, y) = position;

        let max_x = self.max_x();
        let min_x = self.min_x();
        let max_y = self.max_y();

        if x > &max_x {
            return true;
        }

        if x < &min_x {
            return true;
        }

        if y > &max_y {
            return true;
        }

        if y < &0 {
            return true;
        }

        return false;
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
                            MapState::Water(water) => match water {
                                Water::AtRest => {
                                    row_string.push_str("~");
                                }
                                Water::Flowing => {
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

    fn is_clay(&self, position: &Coordinate) -> bool {
        match self.terrain.get(&position) {
            None => {
                return false;
            }
            Some(map_state) => {
                match map_state {
                    MapState::Clay => {
                        return true;
                    }
                    _ => {
                        return false;
                    }
                };
            }
        }
    }

    fn is_dry_sand(&self, position: &Coordinate) -> bool {
        return self.terrain.get(&position).is_none();
    }

    fn upgrade_water(&mut self, position: &Coordinate) {
        match self.terrain.get(&position) {
            None => {
                self.terrain
                    .insert(*position, MapState::Water(Water::Flowing));
            }
            Some(map_state) => {
                match map_state {
                    MapState::Water(water_state) => match water_state {
                        Water::Flowing => {
                            let left_position = position.left();
                            let left_condition =
                                self.is_clay(&left_position) || self.is_water(&left_position);

                            let right_position = position.right();
                            let right_condition =
                                self.is_clay(&right_position) || self.is_water(&right_position);

                            if left_condition && right_condition {
                                self.terrain
                                    .insert(*position, MapState::Water(Water::AtRest));
                            }
                        }
                        Water::AtRest => {}
                    },
                    MapState::Clay => {
                        unreachable!();
                    }
                };
            }
        }
    }

    fn can_flow_into(&self, position: &Coordinate) -> bool {
        return !self.is_clay(position) && self.is_dry_sand(position);
    }

    fn is_water_at_rest(&self, position: &Coordinate) -> bool {
        match self.terrain.get(&position) {
            None => {
                return false;
            }
            Some(map_state) => match map_state {
                MapState::Clay => {
                    return false;
                }
                MapState::Water(water_state) => match water_state {
                    Water::Flowing => false,
                    Water::AtRest => true,
                },
            },
        }
    }

    fn is_water_flowing(&self, position: &Coordinate) -> bool {
        match self.terrain.get(&position) {
            None => {
                return false;
            }
            Some(map_state) => match map_state {
                MapState::Clay => {
                    return false;
                }
                MapState::Water(water_state) => match water_state {
                    Water::Flowing => true,
                    Water::AtRest => false,
                },
            },
        }
    }

    fn is_water(&self, position: &Coordinate) -> bool {
        match self.terrain.get(&position) {
            None => {
                return false;
            }
            Some(map_state) => match map_state {
                MapState::Clay => {
                    return false;
                }
                MapState::Water(_water_state) => {
                    return true;
                }
            },
        }
    }

    fn run_water(&mut self) {
        let mut flowing_water: Vec<Coordinate> = vec![WATER_SPRING.down()];

        let mut index = 1;

        while let Some(current) = flowing_water.pop() {
            if index >= 85 {
                break;
            }
            index += 1;

            println!("{:?}", current);
            println!("{}", self.to_string());
            println!("============");

            // invariant: current position is not clay
            assert!(!self.is_clay(&current));
            // invariant: current position is dry sand
            // assert!(self.is_dry_sand(&current));

            if self.is_dry_sand(&current) {
                self.upgrade_water(&current);
            }

            // can water flow down?
            let next_position_down = current.down();
            // invariant: water cannot go down only if the next position is:
            // - water
            // - or clay
            let should_flow_sideways =
                self.is_clay(&next_position_down) || self.is_water_at_rest(&next_position_down);

            if should_flow_sideways {
                // TODO:
                continue;
            }

            if self.is_coord_out_of_bounds(&next_position_down) {
                // invariant: water will flow infinitely into the abyss
                continue;
            }

            if self.is_water(&next_position_down) {
                // invariant: no new areas of dry sand to flow into
                continue;
            }

            // at this point, water can flow down
            if self.is_water_flowing(&current) {
                flowing_water.push(current);
            }

            flowing_water.push(next_position_down);
        }
    }
}

fn generate_map(input_string: &str) -> Map {
    // parse positions of clay

    let clay_coordinates: Vec<Coordinate> =
        input_string.trim().lines().fold(vec![], |mut acc, line| {
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
                let _axis_str = parsed_range[0];
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
        map.insert_clay(&coordinate);
    }

    return map;
}

fn main() {
    let input_string = include_str!("input.txt");

    let mut map = generate_map(input_string);

    println!("max_y: {}", map.max_y());
    println!("min_x: {}", map.min_x());
    println!("max_x: {}", map.max_x());

    println!("{}", map.to_string());
    println!("============");

    map.run_water();

    println!("{}", map.to_string());
    println!("============");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let input_string = r###"
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504
        "###
        .trim();

        let expected = r###"
.....+......
...........#
#..#.......#
#..#..#.....
#..#..#.....
#.....#.....
#.....#.....
#######.....
............
............
...#.....#..
...#.....#..
...#.....#..
...#######..
        "###
        .trim();

        let map = generate_map(input_string);

        assert_eq!(map.to_string(), expected);
    }
}
