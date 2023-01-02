// https://adventofcode.com/2022/day/9

use std::collections::HashSet;

type Coordinate = (i32, i32);

#[derive(Debug, Clone)]
enum Move {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

trait Transitions {
    fn up(&self) -> Coordinate;
    fn down(&self) -> Coordinate;
    fn left(&self) -> Coordinate;
    fn right(&self) -> Coordinate;
}

impl Transitions for Coordinate {
    fn up(&self) -> Coordinate {
        let (x, y) = self;
        (*x, y + 1)
    }

    fn down(&self) -> Coordinate {
        let (x, y) = self;
        (*x, y - 1)
    }

    fn left(&self) -> Coordinate {
        let (x, y) = self;
        (x - 1, *y)
    }

    fn right(&self) -> Coordinate {
        let (x, y) = self;
        (x + 1, *y)
    }
}

#[derive(Debug, Clone)]
struct Rope {
    knots: Vec<Coordinate>,
    tail_visits: HashSet<Coordinate>,
}

impl Rope {
    fn new(num_of_body_knots: u32) -> Self {
        let starting_point: Coordinate = (0, 0);
        let tail_visits = {
            let mut tail_visits = HashSet::new();
            tail_visits.insert(starting_point);
            tail_visits
        };

        let mut knots: Vec<Coordinate> = vec![(0, 0), (0, 0)];

        for _ in 1..=num_of_body_knots {
            knots.push(starting_point);
        }

        Rope { knots, tail_visits }
    }

    fn parse_move(&mut self, parsed_move_step: Move) {
        match parsed_move_step {
            Move::Up(direction_length) => {
                for _ in 1..=direction_length {
                    self.knots[0] = self.knots[0].up();
                    self.update_body_positions();
                }
            }
            Move::Down(direction_length) => {
                for _ in 1..=direction_length {
                    self.knots[0] = self.knots[0].down();
                    self.update_body_positions();
                }
            }
            Move::Left(direction_length) => {
                for _ in 1..=direction_length {
                    self.knots[0] = self.knots[0].left();
                    self.update_body_positions();
                }
            }
            Move::Right(direction_length) => {
                for _ in 1..=direction_length {
                    self.knots[0] = self.knots[0].right();
                    self.update_body_positions();
                }
            }
        }
    }

    fn update_body_positions(&mut self) {
        let num_of_knots = self.knots.len();
        for index in 1..=(num_of_knots - 1) {
            self.update_knot_position(index);
        }
    }

    fn update_knot_position(&mut self, index_of_knot: usize) {
        assert!(index_of_knot > 0);

        let is_last_knot = index_of_knot == (self.knots.len() - 1);

        let prev_knot = self.knots[index_of_knot - 1];
        let current_knot = &mut self.knots[index_of_knot];

        let (prev_knot_x, prev_knot_y) = prev_knot;
        let (current_knot_x, current_knot_y) = *current_knot;

        // Tail must always be within two steps directly up, down, left, or right away from head.
        let x_distance = (prev_knot_x - current_knot_x).abs();
        let y_distance = (prev_knot_y - current_knot_y).abs();
        assert!(x_distance <= 2);
        assert!(y_distance <= 2);
        let same_row = prev_knot_y == current_knot_y;
        let same_col = prev_knot_x == current_knot_x;
        let head_and_tail_touching = x_distance == 1 && y_distance == 1;

        // If the head is ever two steps directly up, down, left, or right from the tail, the tail must also move one
        // step in that direction so it remains close enough
        if x_distance == 2 && same_row {
            if prev_knot_x > current_knot_x {
                *current_knot = current_knot.right();
            } else {
                *current_knot = current_knot.left();
            }
        } else if y_distance == 2 && same_col {
            if prev_knot_y > current_knot_y {
                *current_knot = current_knot.up();
            } else {
                *current_knot = current_knot.down();
            }

        // if the head and tail aren't touching and aren't in the same row or column, the tail always moves one step
        // diagonally to keep up
        } else if !head_and_tail_touching && !same_row && !same_col {
            if prev_knot_x > current_knot_x {
                *current_knot = current_knot.right();
            } else {
                *current_knot = current_knot.left();
            }

            if prev_knot_y > current_knot_y {
                *current_knot = current_knot.up();
            } else {
                *current_knot = current_knot.down();
            }
        }

        if is_last_knot {
            self.tail_visits.insert(*current_knot);
        }
    }
}

fn part_1(input_string: String) -> usize {
    let mut rope = Rope::new(0);

    for line in input_string.trim().lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parsed_move_step = {
            let parsed: Vec<&str> = line.split_whitespace().collect();
            assert_eq!(parsed.len(), 2);
            let direction = parsed[0];
            let direction_length: u32 = parsed[1].parse().unwrap();
            assert!(direction_length > 0);

            match direction {
                "U" => Move::Up(direction_length),
                "D" => Move::Down(direction_length),
                "L" => Move::Left(direction_length),
                "R" => Move::Right(direction_length),
                _ => {
                    unreachable!();
                }
            }
        };

        rope.parse_move(parsed_move_step);
    }

    rope.tail_visits.len()
}

fn part_2(input_string: String) -> usize {
    let mut rope = Rope::new(8);

    for line in input_string.trim().lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parsed_move_step = {
            let parsed: Vec<&str> = line.split_whitespace().collect();
            assert_eq!(parsed.len(), 2);
            let direction = parsed[0];
            let direction_length: u32 = parsed[1].parse().unwrap();
            assert!(direction_length > 0);

            match direction {
                "U" => Move::Up(direction_length),
                "D" => Move::Down(direction_length),
                "L" => Move::Left(direction_length),
                "R" => Move::Right(direction_length),
                _ => {
                    unreachable!();
                }
            }
        };

        rope.parse_move(parsed_move_step);
    }

    rope.tail_visits.len()
}

fn main() {
    let input_string = include_str!("input.txt").to_string();

    let tail_visits = part_1(input_string.clone());
    println!("Part 1: {}", tail_visits);
    assert_eq!(tail_visits, 6357);

    let tail_visits = part_2(input_string);
    println!("Part 2: {}", tail_visits);
    assert_eq!(tail_visits, 2627);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input_string = r###"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"###
        .to_string();

        assert_eq!(part_1(input_string.to_string()), 13);
    }

    #[test]
    fn test_part_2() {
        let input_string = r###"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"###
        .to_string();

        assert_eq!(part_2(input_string.to_string()), 1);

        let input_string = r###"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"###
        .to_string();

        assert_eq!(part_2(input_string.to_string()), 36);
    }
}
