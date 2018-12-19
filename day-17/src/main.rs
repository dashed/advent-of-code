// https://adventofcode.com/2018/day/17

// imports

use std::collections::HashMap;
use std::collections::HashSet;

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

    fn num_of_water_tiles(&self) -> i32 {
        let min_y = self.min_y();
        let max_y = self.max_y();
        let min_x = self.min_x();
        let max_x = self.max_x();

        let mut total = 0;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let position = (x, y);

                if self.is_water(&position) {
                    total += 1;
                }
            }
        }

        return total;
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

    fn min_y(&self) -> i32 {
        return self
            .terrain
            .iter()
            .filter(|item| {
                let (coord, _map_state) = item;
                return self.is_clay(coord);
            })
            .map(|item| {
                let (coord, _map_state) = item;
                let (_x, y) = coord;
                return *y;
            })
            .min()
            .unwrap();
    }

    fn max_y(&self) -> i32 {
        return self
            .terrain
            .iter()
            .filter(|item| {
                let (coord, _map_state) = item;
                return self.is_clay(coord);
            })
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
            .filter(|item| {
                let (coord, _map_state) = item;
                return self.is_clay(coord);
            })
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
            .filter(|item| {
                let (coord, _map_state) = item;
                return self.is_clay(coord);
            })
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

    #[allow(dead_code)]
    fn to_string(&self) -> String {
        let max_y = self.max_y();
        // let max_y = 100;
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
                            self.terrain
                                .insert(*position, MapState::Water(Water::AtRest));
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

        'main_loop: while let Some(current) = flowing_water.pop() {
            // use std::thread;
            // use std::time::Duration;
            // thread::sleep(Duration::from_millis(100));
            // println!("{}", self.to_string());
            // println!("============");

            // invariant: current position is not clay
            assert!(!self.is_clay(&current));

            if self.is_dry_sand(&current) {
                self.upgrade_water(&current);
            }

            // can water flow down?
            let next_position_down = current.down();

            if self.is_coord_out_of_bounds(&next_position_down) {
                // invariant: water will flow infinitely into the abyss
                // invariant: there's no clay to hit
                continue;
            }

            if self.is_dry_sand(&next_position_down) {
                flowing_water.push(current);
                flowing_water.push(next_position_down);
                continue;
            }

            if self.is_clay(&next_position_down) || self.is_water_at_rest(&next_position_down) {
                let left_position = current.left();
                let left_condition = self.is_dry_sand(&left_position)
                    && !self.is_coord_out_of_bounds(&left_position);

                let right_position = current.right();
                let right_condition = self.is_dry_sand(&right_position)
                    && !self.is_coord_out_of_bounds(&right_position);

                if left_condition || right_condition {
                    flowing_water.push(current);
                }

                if left_condition {
                    flowing_water.push(left_position);
                }

                if right_condition {
                    flowing_water.push(right_position);
                }

                if left_condition || right_condition {
                    continue;
                }

                // no dry sand on either side of current

                assert!(self.is_water_flowing(&current));

                // sweep left until dry sand is found

                let mut sweeped_positions = vec![];

                let mut current_sweep = left_position;

                while !self.is_coord_out_of_bounds(&current_sweep) {
                    if self.is_dry_sand(&current_sweep) {
                        flowing_water.push(left_position);
                        continue 'main_loop;
                    }

                    if self.is_clay(&current_sweep) {
                        // hit a wall
                        break;
                    }

                    let below_sweep = current_sweep.down();

                    if self.is_clay(&below_sweep) || self.is_water_at_rest(&below_sweep) {
                        sweeped_positions.push(current_sweep);
                    }

                    current_sweep = current_sweep.left();
                }

                println!("sweep: {:?}", current);
            }
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

    map.run_water();

    println!("{}", map.to_string());
    // not: 2339
    println!("Part 1: {}", map.num_of_water_tiles());
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

        let mut map = generate_map(input_string);

        assert_eq!(map.to_string(), expected);

        map.run_water();

        assert_eq!(map.num_of_water_tiles(), 57);
    }
}
