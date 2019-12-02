// https://adventofcode.com/2015/day/3

// imports

use std::collections::HashMap;

// code

type Coordinate = (i32, i32);

trait Transitions {
    fn up(&self) -> Coordinate;
    fn down(&self) -> Coordinate;
    fn left(&self) -> Coordinate;
    fn right(&self) -> Coordinate;
}

impl Transitions for Coordinate {
    fn up(&self) -> Coordinate {
        let (x, y) = self;
        return (*x, y + 1);
    }

    fn down(&self) -> Coordinate {
        let (x, y) = self;
        return (*x, y - 1);
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

type Visits = u32;

struct Santa {
    visited_houses: HashMap<Coordinate, Visits>,
    current_position: Coordinate,
}

impl Santa {
    fn new(initial_position: Coordinate) -> Self {
        let mut visited_houses = HashMap::new();
        visited_houses.insert(initial_position, 1);

        Santa {
            visited_houses,
            current_position: initial_position,
        }
    }

    fn update_next_coord(&mut self, next_coord: Coordinate) {
        let counter = self.visited_houses.entry(next_coord).or_insert(0);
        *counter += 1;

        self.current_position = next_coord;
    }

    fn up(&mut self) {
        let next_coord = self.current_position.up();
        self.update_next_coord(next_coord);
    }

    fn down(&mut self) {
        let next_coord = self.current_position.down();
        self.update_next_coord(next_coord);
    }

    fn left(&mut self) {
        let next_coord = self.current_position.left();
        self.update_next_coord(next_coord);
    }

    fn right(&mut self) {
        let next_coord = self.current_position.right();
        self.update_next_coord(next_coord);
    }

    fn num_of_visited_houses(&self) -> usize {
        self.visited_houses.keys().len()
    }

    fn visited_coords(&self) -> Vec<Coordinate> {
        self.visited_houses.keys().cloned().collect()
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    println!("Part 1: {}", part_1(input_string));

    println!("Part 2: {}", part_2(input_string));
}

fn parse_input(input_string: &str) -> Santa {
    let mut santa = Santa::new((0, 0));

    for instruction in input_string.trim().chars() {
        match instruction {
            '^' => {
                santa.up();
            }
            'v' => {
                santa.down();
            }
            '>' => {
                santa.right();
            }
            '<' => {
                santa.left();
            }

            _ => {
                unreachable!();
            }
        }
    }

    santa
}

fn part_1(input_string: &str) -> usize {
    let santa = parse_input(input_string);

    santa.num_of_visited_houses()
}

fn part_2(input_string: &str) -> usize {
    enum Turn {
        Santa,
        RoboSanta,
    }

    let mut current_turn = Turn::Santa;
    let mut santa_instructions = String::from("");
    let mut robo_santa_instructions = String::from("");

    for instruction in input_string.chars() {
        match current_turn {
            Turn::Santa => {
                santa_instructions.push(instruction);
                current_turn = Turn::RoboSanta;
            }
            Turn::RoboSanta => {
                robo_santa_instructions.push(instruction);
                current_turn = Turn::Santa;
            }
        }
    }

    let santa = parse_input(&santa_instructions);
    let robo_santa = parse_input(&robo_santa_instructions);

    let mut visited_coords = santa.visited_coords();
    visited_coords.append(&mut robo_santa.visited_coords());

    visited_coords.sort();
    visited_coords.dedup();

    visited_coords.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("input.txt")), 2572);

        assert_eq!(part_1(">"), 2);
        assert_eq!(part_1("^>v<"), 4);
        assert_eq!(part_1("^v^v^v^v^v"), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("input.txt")), 2631);

        assert_eq!(part_2("^v"), 3);
        assert_eq!(part_2("^>v<"), 3);
        assert_eq!(part_2("^v^v^v^v^v"), 11);
    }
}
