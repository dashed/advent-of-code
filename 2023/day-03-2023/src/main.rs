// https://adventofcode.com/2023/day/3

use std::collections::HashMap;

#[derive(Debug, Clone)]
enum MapState {
    Empty,
    Symbol(char),
    Digit(i32),
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

type Terrain = HashMap<Coordinate, MapState>;

struct Schematic {
    terrain: Terrain,
    max_y_coord: i32,
    max_x_coord: i32,
}

impl Schematic {
    fn new(input_string: &str) -> Self {
        let inputs: Vec<&str> = input_string.trim().lines().collect();

        let max_x_coord = inputs[0].len() as i32;
        let mut terrain = Terrain::new();

        for (y_coord, line) in inputs.iter().enumerate() {
            for (x_coord, character) in line.trim().chars().enumerate() {
                let coordinate = (x_coord as i32, y_coord as i32);
                match character {
                    '.' => {
                        terrain.insert(coordinate, MapState::Empty);
                    }
                    _ => {
                        if let Some(digit) = character.to_digit(10) {
                            terrain.insert(coordinate, MapState::Digit(digit as i32));
                        } else {
                            terrain.insert(coordinate, MapState::Symbol(character));
                        }
                    }
                }
            }
        }

        Schematic {
            terrain,
            max_y_coord: inputs.len() as i32,
            max_x_coord,
        }
    }

    fn get(&self, coordinate: &Coordinate) -> MapState {
        match self.terrain.get(coordinate) {
            None => MapState::Empty,
            Some(state) => state.clone(),
        }
    }

    fn is_adjacent_to_symbol(&self, coordinate: &Coordinate) -> bool {
        let coordinates_to_check = [
            coordinate.north(),
            coordinate.south(),
            coordinate.west(),
            coordinate.east(),
            coordinate.north().west(),
            coordinate.north().east(),
            coordinate.south().west(),
            coordinate.south().east(),
        ];

        let coordinates_to_check = coordinates_to_check
            .iter()
            .filter(|c| c.within_bounds(self.max_x_coord, self.max_y_coord));

        for coordinate_to_check in coordinates_to_check {
            if let MapState::Symbol(_) = self.get(coordinate_to_check) {
                return true;
            }
        }
        false
    }

    fn is_valid_coordinate(&self, coordinate: &Coordinate) -> bool {
        coordinate.within_bounds(self.max_x_coord, self.max_y_coord)
    }
}

fn part_1(input_string: &str) -> i32 {
    let map = Schematic::new(input_string);

    let mut valid_numbers: Vec<i32> = vec![];

    for y_coord in 0..map.max_y_coord {
        let mut x_coord = 0;
        loop {
            let coordinate = (x_coord, y_coord);
            if !map.is_valid_coordinate(&coordinate) {
                break;
            }

            let mut is_adjacent_to_symbol = false;
            let mut digits_buffer: Vec<i32> = vec![];

            match map.get(&coordinate) {
                MapState::Empty => {
                    x_coord += 1;
                    continue;
                }
                MapState::Symbol(_) => {
                    x_coord += 1;
                    continue;
                }
                MapState::Digit(digit) => {
                    if map.is_adjacent_to_symbol(&coordinate) {
                        is_adjacent_to_symbol = true;
                    }
                    digits_buffer.push(digit);
                    x_coord += 1;
                    let mut next_coord = coordinate.east();
                    loop {
                        if !map.is_valid_coordinate(&next_coord) {
                            break;
                        }

                        match map.get(&next_coord) {
                            MapState::Digit(digit) => {
                                if map.is_adjacent_to_symbol(&next_coord) {
                                    is_adjacent_to_symbol = true;
                                }
                                digits_buffer.push(digit);
                                x_coord += 1;
                                next_coord = next_coord.east();
                            }
                            _ => break,
                        }
                    }

                    if is_adjacent_to_symbol {
                        let digits = digits_buffer
                            .into_iter()
                            .map(|d| d.to_string())
                            .collect::<Vec<String>>()
                            .join("");
                        let number = digits.parse::<i32>().unwrap();
                        valid_numbers.push(number);
                    }

                    continue;
                }
            }
        }
    }

    valid_numbers.iter().sum()
}

fn main() {
    let input_string = include_str!("input.txt");

    // Part 1

    let answer = part_1(input_string);
    println!("Part 1: {}", answer);
    assert_eq!(answer, 539637);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_2() {
        let input_string = r###"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"###;

        assert_eq!(part_1(input_string), 4361);

        // assert_eq!(part_2(input_string), 2286);
    }
}
