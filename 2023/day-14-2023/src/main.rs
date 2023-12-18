use std::collections::{HashMap, HashSet};

enum Direction {
    North,
    South,
    West,
    East,
}

type Coordinate = (i32, i32);

trait BoundsCheck {
    fn within_bounds(&self, max_x: i32, max_y: i32) -> bool;
}

impl BoundsCheck for Coordinate {
    fn within_bounds(&self, max_x: i32, max_y: i32) -> bool {
        let (x, y) = self;
        let x_bounds = 0 <= *x && *x <= max_x;
        let y_bounds = 0 <= *y && *y <= max_y;
        x_bounds && y_bounds
    }
}

trait Transitions {
    fn north(&self) -> Coordinate;
    fn south(&self) -> Coordinate;
    fn west(&self) -> Coordinate;
    fn east(&self) -> Coordinate;
}

impl Transitions for Coordinate {
    fn north(&self) -> Coordinate {
        let (x, y) = self;
        (*x, y - 1)
    }

    fn south(&self) -> Coordinate {
        let (x, y) = self;
        (*x, y + 1)
    }

    fn west(&self) -> Coordinate {
        let (x, y) = self;
        (x - 1, *y)
    }

    fn east(&self) -> Coordinate {
        let (x, y) = self;
        (x + 1, *y)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Rocks {
    Round,
    Cube,
}

type Map = HashMap<Coordinate, Rocks>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Platform {
    map: Map,
    // rocks that can move
    round_rocks: HashSet<Coordinate>,
    // rocks that can't move
    cube_rocks: HashSet<Coordinate>,
    max_y_coord: i32,
    max_x_coord: i32,
}

impl Platform {
    #[allow(dead_code)]
    fn to_str(&self) -> String {
        let mut string = String::new();
        for y_coord in 0..=self.max_y_coord {
            for x_coord in 0..=self.max_x_coord {
                let coord = (x_coord, y_coord);
                match self.map.get(&coord) {
                    None => string.push('.'),
                    Some(Rocks::Round) => string.push('O'),
                    Some(Rocks::Cube) => string.push('#'),
                }
            }
            string.push('\n');
        }

        string.trim().to_string()
    }

    fn cache_platform(&mut self, cache: &mut HashSet<String>) {
        let rocks = self.to_str();

        if cache.contains(&rocks) {
            return;
        }

        cache.insert(rocks);
    }

    fn move_rock(&mut self, coord: Coordinate, direction: &Direction) -> Option<Coordinate> {
        // Returns the new coordinate of the rock if it moved, or None if it didn't move north.

        assert!(self.round_rocks.contains(&coord), "Rock is not round");
        assert!(!self.cube_rocks.contains(&coord), "Wrong rock type");
        assert!(
            self.map.get(&coord).unwrap() == &Rocks::Round,
            "Rock is not round"
        );

        let new_coord = match direction {
            Direction::North => coord.north(),
            Direction::South => coord.south(),
            Direction::West => coord.west(),
            Direction::East => coord.east(),
        };

        if !new_coord.within_bounds(self.max_x_coord, self.max_y_coord) {
            return None;
        }

        if self.map.get(&new_coord).is_some() {
            return None;
        }

        assert!(
            !self.round_rocks.contains(&new_coord),
            "Round rock in the way"
        );
        assert!(
            !self.cube_rocks.contains(&new_coord),
            "Cube rock in the way"
        );

        self.round_rocks.remove(&coord);
        self.round_rocks.insert(new_coord);

        self.map.remove(&coord);
        self.map.insert(new_coord, Rocks::Round);

        Some(new_coord)
    }

    fn prepare_rocks(&self) -> Vec<Coordinate> {
        let mut rocks_to_move: Vec<Coordinate> = self.round_rocks.clone().into_iter().collect();

        // order rocks row-wise from left to right, top to bottom
        // (0, 0), (1, 0), (2, 0), (3, 0), (0, 1), (1, 1), ...
        rocks_to_move.sort_by(|a, b| {
            if a.1 < b.1 {
                return std::cmp::Ordering::Less;
            }
            if a.0 <= b.0 && a.1 <= b.1 {
                return std::cmp::Ordering::Less;
            }

            return std::cmp::Ordering::Greater;
        });

        // for rocks in rocks_to_move.windows(2) {
        //     let rock = rocks[0];
        //     let next_rock = rocks[1];

        //     assert!(
        //         rock.1 <= next_rock.1,
        //         "Rocks are not ordered correctly: {:?} {:?}",
        //         rock,
        //         next_rock
        //     );

        //     if rock.1 == next_rock.1 {
        //         assert!(
        //             rock.0 < next_rock.0,
        //             "Rocks are not ordered correctly: {:?} {:?}",
        //             rock,
        //             next_rock
        //         );
        //     }
        // }

        rocks_to_move
    }

    fn move_rocks(&mut self, direction: Direction) {
        let mut rocks_to_move: Vec<Coordinate> = self.prepare_rocks();

        match direction {
            Direction::North | Direction::West => {}
            Direction::South | Direction::East => {
                rocks_to_move.reverse();
            }
        }

        loop {
            if rocks_to_move.is_empty() {
                break;
            }

            let rock = rocks_to_move.remove(0);
            match self.move_rock(rock, &direction) {
                None => {}
                Some(coord) => {
                    rocks_to_move.push(coord);
                }
            }
        }
    }

    fn get_load(&self) -> i32 {
        let mut load = 0;
        for rock in self.round_rocks.iter() {
            let (_, y) = rock;
            load += (self.max_y_coord - y).abs() + 1;
        }

        load
    }

    fn perform_cycle(&mut self) {
        self.move_rocks(Direction::North);
        self.move_rocks(Direction::West);
        self.move_rocks(Direction::South);
        self.move_rocks(Direction::East);
    }
}

fn generate_platform(input_string: &str) -> Platform {
    let inputs: Vec<&str> = input_string.trim().lines().collect();

    let max_y_coord = inputs.len() as i32 - 1;
    let max_x_coord = inputs[0].len() as i32 - 1;

    let mut map = Map::new();
    let mut round_rocks = HashSet::new();
    let mut cube_rocks = HashSet::new();

    for (y_coord, line) in inputs.iter().enumerate() {
        for (x_coord, character) in line.trim().chars().enumerate() {
            let coordinate = (x_coord as i32, y_coord as i32);

            match character {
                'O' => {
                    map.insert(coordinate, Rocks::Round);
                    round_rocks.insert(coordinate);
                }
                '#' => {
                    map.insert(coordinate, Rocks::Cube);
                    cube_rocks.insert(coordinate);
                }
                '.' => {
                    // Empty space
                }
                _ => panic!("Unknown character: {}", character),
            }
        }
    }

    Platform {
        map,
        round_rocks,
        cube_rocks,
        max_y_coord,
        max_x_coord,
    }
}

fn part_1(input_string: &str) -> i32 {
    let mut platform = generate_platform(input_string);
    platform.move_rocks(Direction::North);
    platform.get_load()
}

fn part_2(input_string: &str) -> i32 {
    // map Platform to load
    let mut cache: HashSet<String> = HashSet::new();
    let mut map_to_cycle: HashMap<String, i32> = HashMap::new();
    // let mut cycle_to_platform: HashMap<i32, Platform> = HashMap::new();
    let mut cycle_to_load: HashMap<i32, i32> = HashMap::new();

    let mut platform = generate_platform(input_string);

    for current_cycle in 1..=1_000_000_000 {
        platform.perform_cycle();

        // println!("Cycle: {}", current_cycle);
        // println!("Load: {}", platform.get_load());

        let rocks = platform.to_str();
        if cache.contains(&rocks) {
            println!("Cycle detected at {}", current_cycle);

            let cycle_start = map_to_cycle.get(&rocks).unwrap();
            let cycle_end = current_cycle - 1;
            let cycle_length = cycle_end - cycle_start + 1;

            // println!("---------------------");
            // for cycle in *cycle_start..=cycle_end {
            //     println!("Cycle: {}", cycle);
            //     println!("Load: {}", cycle_to_load.get(&cycle).unwrap());
            // }
            // println!("---------------------");

            // println!("Cycle start: {}", cycle_start);
            // println!("Cycle end: {}", cycle_end);
            // println!("Cycle length: {}", cycle_length);

            let cycle_target = ((1_000_000_000 - cycle_start) % cycle_length) + cycle_start;

            // println!("Cycle target: {}", cycle_target);

            // return cycle_to_platform.get(&cycle_target).unwrap().get_load();
            return cycle_to_load.get(&cycle_target).unwrap().clone();
        }

        platform.cache_platform(&mut cache);
        map_to_cycle.insert(rocks, current_cycle);

        // cycle_to_platform.insert(current_cycle, platform.clone());
        cycle_to_load.insert(current_cycle, platform.get_load());
    }

    platform.get_load()
}

fn main() {
    let input_string = include_str!("input.txt");

    // Part 1

    let answer = part_1(input_string);
    println!("Part 1: {}", answer);
    assert_eq!(answer, 111979);

    // Part 2

    let answer = part_2(input_string);
    println!("Part 2: {}", answer);
    // 102193 too high
    assert_eq!(answer, 102055);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perform_cycle() {
        let input_string = r###"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"###
        .trim();

        let mut platform = generate_platform(input_string);

        assert_eq!(platform.to_str(), input_string);

        platform.perform_cycle();

        let expected_string = r###"
.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....
"###
        .trim();

        assert_eq!(platform.to_str(), expected_string);

        platform.perform_cycle();

        let expected_string = r###"
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O
"###
        .trim();

        assert_eq!(platform.to_str(), expected_string);

        platform.perform_cycle();

        let expected_string = r###"
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O
"###
        .trim();

        assert_eq!(platform.to_str(), expected_string);
    }

    #[test]
    fn test_move_rocks() {
        let input_string = r###"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"###
        .trim();

        let mut platform = generate_platform(input_string);

        assert_eq!(platform.to_str(), input_string);

        platform.move_rocks(Direction::North);

        let expected_string = r###"
OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....
"###
        .trim();

        assert_eq!(platform.to_str(), expected_string);

        platform.move_rocks(Direction::West);

        let expected_string = r###"
OOOO.#O...
OO..#....#
OOO..##O..
O..#OO....
........#.
..#....#.#
O....#OO..
O.........
#....###..
#....#....
"###
        .trim();

        assert_eq!(platform.to_str(), expected_string);

        platform.move_rocks(Direction::South);

        let expected_string = r###"
.....#....
....#.O..#
O..O.##...
O.O#......
O.O....O#.
O.#..O.#.#
O....#....
OO....OO..
#O...###..
#O..O#....
"###
        .trim();

        assert_eq!(platform.to_str(), expected_string);

        platform.move_rocks(Direction::East);

        let expected_string = r###"
.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....
"###
        .trim();

        assert_eq!(platform.to_str(), expected_string);
    }

    #[test]
    fn test_puzzle() {
        let input_string = r###"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"###;

        assert_eq!(part_1(input_string), 136);
        assert_eq!(part_2(input_string), 64);
    }
}
