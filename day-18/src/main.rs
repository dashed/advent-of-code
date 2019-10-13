// https://adventofcode.com/2018/day/18

// imports

use rayon::prelude::*;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

// code

type Coordinate = (i32, i32);

type CollectionArea = HashMap<Coordinate, Acre>;

trait Transitions {
    fn up(&self) -> Coordinate;
    fn down(&self) -> Coordinate;
    fn left(&self) -> Coordinate;
    fn right(&self) -> Coordinate;
}

impl Transitions for Coordinate {
    fn up(&self) -> Coordinate {
        let (x, y) = self;
        return (*x, y - 1);
    }

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

#[derive(Debug, Clone, PartialEq, Eq)]
enum Acre {
    Ground,
    Tree,
    Lumberyard,
}

impl Acre {
    fn next(&self, adjacent_acres: Vec<Acre>) -> Self {
        match self {
            Acre::Ground => {
                // An open acre will become filled with trees if three or more
                // adjacent acres contained trees. Otherwise, nothing happens.
                let num_of_adjacent_trees = adjacent_acres
                    .par_iter()
                    .filter(|s| **s == Acre::Tree)
                    .count();

                if num_of_adjacent_trees >= 3 {
                    return Acre::Tree;
                }

                return self.clone();
            }
            Acre::Tree => {
                // An acre filled with trees will become a lumberyard if three or more
                // adjacent acres were lumberyards. Otherwise, nothing happens.
                let num_of_adjacent_lumberyards = adjacent_acres
                    .par_iter()
                    .filter(|s| **s == Acre::Lumberyard)
                    .count();

                if num_of_adjacent_lumberyards >= 3 {
                    return Acre::Lumberyard;
                }
                return self.clone();
            }
            Acre::Lumberyard => {
                // An acre containing a lumberyard will remain a lumberyard if it was adjacent
                // to at least one other lumberyard and at least one acre containing trees.
                // Otherwise, it becomes open.
                let num_of_adjacent_lumberyards = adjacent_acres
                    .par_iter()
                    .filter(|s| **s == Acre::Lumberyard)
                    .count();

                let num_of_adjacent_trees = adjacent_acres
                    .par_iter()
                    .filter(|s| **s == Acre::Tree)
                    .count();

                if num_of_adjacent_lumberyards >= 1 && num_of_adjacent_trees >= 1 {
                    return self.clone();
                }

                return Acre::Ground;
            }
        }
    }
}

#[derive(Debug, Clone, Eq)]
struct Area {
    area: CollectionArea,
    max_y: i32,
    max_x: i32,
}

impl PartialEq for Area {
    fn eq(&self, other: &Self) -> bool {
        let equal_max_y = self.max_y == other.max_y;
        let equal_max_x = self.max_x == other.max_x;
        let equal_string = self.to_string() == other.to_string();

        equal_max_y && equal_max_x && equal_string
    }
}

impl Hash for Area {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.max_y.hash(state);
        self.max_x.hash(state);
        self.to_string().hash(state);
    }
}

impl Area {
    fn new() -> Self {
        Area {
            area: HashMap::new(),
            max_y: 0,
            max_x: 0,
        }
    }

    fn num_of_trees(&self) -> usize {
        return self.area.values().filter(|s| **s == Acre::Tree).count();
    }

    fn num_of_lumberyards(&self) -> usize {
        return self
            .area
            .values()
            .filter(|s| **s == Acre::Lumberyard)
            .count();
    }

    #[allow(dead_code)]
    fn to_string(&self) -> String {
        let mut map_string: Vec<String> = vec![];

        for y in 0..=self.max_y {
            let mut row_string = String::from("");

            for x in 0..=self.max_x {
                let position = (x, y);

                match self.area.get(&position) {
                    None => {
                        unreachable!();
                    }
                    Some(acre) => match acre {
                        Acre::Ground => {
                            row_string.push_str(".");
                        }
                        Acre::Tree => {
                            row_string.push_str("|");
                        }
                        Acre::Lumberyard => {
                            row_string.push_str("#");
                        }
                    },
                }
            }

            map_string.push(row_string);
        }

        return map_string.join("\n");
    }

    fn insert(&mut self, position: Coordinate, acre: char) {
        let (x, y) = position;

        if x > self.max_x {
            self.max_x = x;
        }

        if y > self.max_y {
            self.max_y = y;
        }

        match acre {
            '.' => {
                self.area.insert(position, Acre::Ground);
            }
            '|' => {
                self.area.insert(position, Acre::Tree);
            }
            '#' => {
                self.area.insert(position, Acre::Lumberyard);
            }
            _ => {
                unreachable!();
            }
        }
    }

    fn tick(&mut self) {
        let prev_area = &self.area;

        let next_area: CollectionArea = prev_area
            .par_iter()
            .map(|(coord, acre)| {
                let adjacent = get_adjacent(&prev_area, &coord);

                // Changes happen across all acres simultaneously,
                // each of them using the state of all acres at the beginning of the minute
                // and changing to their new form by the end of that same minute.
                // Changes that happen during the minute don't affect each other.

                // ✨ magic
                let next_acre = acre.next(adjacent);

                return (coord.clone(), next_acre);
            })
            .collect();

        self.area = next_area;
    }
}

fn get_adjacent(area: &CollectionArea, position: &Coordinate) -> Vec<Acre> {
    let adjacent: Vec<Coordinate> = vec![
        // clockwise
        position.up(),
        position.up().right(),
        position.right(),
        position.down().right(),
        position.down(),
        position.down().left(),
        position.left(),
        position.up().left(),
    ];

    let result: Vec<Acre> = adjacent
        .into_iter()
        .map(|coord| area.get(&coord))
        .filter(|s| s.is_some())
        .map(|s| s.unwrap().clone())
        .collect();

    return result;
}

fn generate_area(input_string: &str) -> Area {
    let mut area = Area::new();

    for (y, line) in input_string.trim().lines().enumerate() {
        let line = line.trim();

        for (x, acre) in line.chars().enumerate() {
            let position: Coordinate = (x as i32, y as i32);
            area.insert(position, acre);
        }
    }

    return area;
}

fn part_1(input_string: &str, ticks: i32) -> usize {
    let mut area = generate_area(input_string);

    for _ in 1..=ticks {
        area.tick();
    }

    return area.num_of_lumberyards() * area.num_of_trees();
}

fn part_2(input_string: &str) -> usize {
    let mut area = generate_area(input_string);

    let mut seen: HashMap<Area, i32> = HashMap::new();
    // let mut lookup = HashMap::new();
    let mut area_num = 0;

    let mut ticks = 0;

    loop {
        if seen.contains_key(&area) {
            break;
        }
        seen.insert(area.clone(), area_num);
        // lookup.insert(area_num, area.clone());
        area.tick();

        ticks += 1;
        area_num += 1;
    }

    // invariant: area is the first area of the cycle

    let cycle_length = seen.len() as i32 - seen.get(&area).unwrap() + 1;

    let remaining_ticks = 1_000_000_000 - ticks;

    // additional ticks to perform
    let additional_ticks = remaining_ticks % (cycle_length - 1);

    // additional ticks left to perform are mapped to [0, cycle_length), or
    // [0, cycle_length - 1].
    // with the current area being the first pattern in the cycle (i.e. 0 ticks to perform)

    // let area_num = seen.get(&area).unwrap();
    // let area = lookup.get(&(area_num + additional_ticks)).unwrap();

    for _ in 1..=additional_ticks {
        area.tick();
    }

    return area.num_of_lumberyards() * area.num_of_trees();
}

fn main() {
    let input_string = include_str!("input.txt");

    let part_1_result = part_1(input_string, 10);

    println!("Part 1: {}", part_1_result);

    println!("Part 2: {}", part_2(input_string));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_string() {
        let expected_string = include_str!("input.txt");

        let area = generate_area(expected_string);

        assert_eq!(area.to_string(), expected_string);
    }

    #[test]
    fn test_part_1() {
        let input_string = r###"
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.
    "###
        .trim();

        assert_eq!(part_1(input_string, 10), 1147);
    }

    #[test]
    fn example() {
        let input_string = r###"
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.
    "###
        .trim();

        let mut area = generate_area(input_string);

        // initial
        assert_eq!(area.to_string(), input_string);

        // after 1 minute
        area.tick();

        let result = r###"
.......##.
......|###
.|..|...#.
..|#||...#
..##||.|#|
...#||||..
||...|||..
|||||.||.|
||||||||||
....||..|.
    "###
        .trim();

        assert_eq!(area.to_string(), result);

        // after 2 minutes
        area.tick();

        let result = r###"
.......#..
......|#..
.|.|||....
..##|||..#
..###|||#|
...#|||||.
|||||||||.
||||||||||
||||||||||
.|||||||||
    "###
        .trim();

        assert_eq!(area.to_string(), result);

        // after 3 minutes
        area.tick();

        let result = r###"
.......#..
....|||#..
.|.||||...
..###|||.#
...##|||#|
.||##|||||
||||||||||
||||||||||
||||||||||
||||||||||
    "###
        .trim();

        assert_eq!(area.to_string(), result);

        // after 4 minutes
        area.tick();

        let result = r###"
.....|.#..
...||||#..
.|.#||||..
..###||||#
...###||#|
|||##|||||
||||||||||
||||||||||
||||||||||
||||||||||
    "###
        .trim();

        assert_eq!(area.to_string(), result);

        // after 10 minutes
        for _ in 5..=10 {
            area.tick();
        }

        let result = r###"
.||##.....
||###.....
||##......
|##.....##
|##.....##
|##....##|
||##.####|
||#####|||
||||#|||||
||||||||||
    "###
        .trim();

        assert_eq!(area.to_string(), result);
    }
}
