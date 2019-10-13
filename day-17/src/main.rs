// https://adventofcode.com/2018/day/17

// imports

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

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

    min_y: Option<i32>,
    max_y: Option<i32>,
}

impl Map {
    fn new() -> Self {
        Map {
            terrain: HashMap::new(),
            min_y: None,
            max_y: None,
        }
    }

    fn num_of_rested_water_tiles(&mut self) -> i32 {
        let min_y = self.min_y();
        let max_y = self.max_y();

        let mut total = 0;

        for (position, _tile) in self.terrain.iter() {
            let (_x, y) = position;

            if y < &min_y {
                continue;
            }

            if y > &max_y {
                continue;
            }

            if self.is_water_at_rest(&position) {
                total += 1;
            }
        }

        return total;
    }

    fn num_of_water_tiles(&mut self) -> i32 {
        let min_y = self.min_y();
        let max_y = self.max_y();

        let mut total = 0;

        for (position, _tile) in self.terrain.iter() {
            let (_x, y) = position;

            if y < &min_y {
                continue;
            }

            if y > &max_y {
                continue;
            }

            if self.is_water(&position) {
                total += 1;
            }
        }

        return total;
    }

    fn is_coord_out_of_bounds(&mut self, position: &Coordinate) -> bool {
        let (_x, y) = position;

        let max_y = self.max_y();

        if y > &max_y {
            return true;
        }

        if y < &0 {
            return true;
        }

        return false;
    }

    fn min_y(&mut self) -> i32 {
        if self.min_y.is_some() {
            return self.min_y.unwrap();
        }

        let value = self
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

        self.min_y = Some(value);

        return value;
    }

    fn max_y(&mut self) -> i32 {
        if self.max_y.is_some() {
            return self.max_y.unwrap();
        }

        let value = self
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

        self.max_y = Some(value);

        return value;
    }

    fn min_x(&mut self) -> i32 {
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

    fn max_x(&mut self) -> i32 {
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

    #[allow(dead_code)]
    fn to_string(&mut self) -> String {
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

    fn flood(&mut self, position: &Coordinate) {
        if self.is_coord_out_of_bounds(position) {
            return;
        }

        if self.is_water_flowing(position) {
            return;
        }

        if self.is_clay(position) || self.is_water_at_rest(position) {
            return;
        }

        if self.is_dry_sand(position) {
            self.upgrade_water(position);
        }

        // flood downward
        let down_position = position.down();

        self.flood(&down_position);

        if !(self.is_clay(&down_position) || self.is_water_at_rest(&down_position)) {
            return;
        }

        // flood left
        let mut water_at_rest = vec![];
        water_at_rest.push(position.clone());

        let mut current = position.left();
        let has_left_wall;

        loop {
            if self.is_clay(&current) || self.is_water_at_rest(&current) {
                has_left_wall = true;
                break;
            }

            if self.is_dry_sand(&current) {
                self.upgrade_water(&current);
            }

            let down = current.down();
            self.flood(&down);

            if !(self.is_clay(&down) || self.is_water_at_rest(&down)) {
                has_left_wall = false;
                break;
            }

            water_at_rest.push(current);

            current = current.left();
        }

        // flood right
        let mut current = position.right();
        let has_right_wall;

        loop {
            if self.is_clay(&current) || self.is_water_at_rest(&current) {
                has_right_wall = true;
                break;
            }

            if self.is_dry_sand(&current) {
                self.upgrade_water(&current);
            }

            let down = current.down();
            self.flood(&down);

            if !(self.is_clay(&down) || self.is_water_at_rest(&down)) {
                has_right_wall = false;
                break;
            }

            water_at_rest.push(current);

            current = current.right();
        }

        if has_left_wall && has_right_wall {
            for current in water_at_rest {
                self.upgrade_water(&current);
            }
        }
    }

    fn run_flood(&mut self) {
        self.flood(&WATER_SPRING.down());
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

    map.run_flood();

    println!("Part 1: {}", map.num_of_water_tiles());

    let mut output = File::create("day-17/part_1_result.txt").unwrap();
    write!(output, "{}", map.to_string()).unwrap();

    println!("Part 2: {}", map.num_of_rested_water_tiles());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_and_2() {
        let input_string = include_str!("input.txt");
        let mut map = generate_map(input_string);

        map.run_flood();

        // Part 1
        assert_eq!(map.num_of_water_tiles(), 33004);

        // Part 2
        assert_eq!(map.num_of_rested_water_tiles(), 23294);
    }

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

        map.run_flood();

        assert_eq!(map.num_of_water_tiles(), 57);
    }
}
