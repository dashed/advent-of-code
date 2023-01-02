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
    head: Coordinate,
    tail: Coordinate,

    tail_visits: HashSet<Coordinate>,
}

impl Rope {
    fn new() -> Self {
        let starting_point: Coordinate = (0, 0);
        let tail_visits = {
            let mut tail_visits = HashSet::new();
            tail_visits.insert(starting_point);
            tail_visits
        };

        Rope {
            head: starting_point,
            tail: starting_point,

            tail_visits,
        }
    }

    fn parse_move(&mut self, parsed_move_step: Move) {
        match parsed_move_step {
            Move::Up(direction_length) => {
                for _ in 1..=direction_length {
                    self.head = self.head.up();
                    self.update_tail_position();
                }
            }
            Move::Down(direction_length) => {
                for _ in 1..=direction_length {
                    self.head = self.head.down();
                    self.update_tail_position();
                }
            }
            Move::Left(direction_length) => {
                for _ in 1..=direction_length {
                    self.head = self.head.left();
                    self.update_tail_position();
                }
            }
            Move::Right(direction_length) => {
                for _ in 1..=direction_length {
                    self.head = self.head.right();
                    self.update_tail_position();
                }
            }
        }
    }

    fn update_tail_position(&mut self) {
        let (head_x, head_y) = self.head;
        let (tail_x, tail_y) = self.tail;

        // Tail must always be within two steps directly up, down, left, or right away from head.
        let x_distance = (head_x - tail_x).abs();
        let y_distance = (head_y - tail_y).abs();
        assert!(x_distance <= 2);
        assert!(y_distance <= 2);
        let same_row = head_y == tail_y;
        let same_col = head_x == tail_x;
        let head_and_tail_touching = x_distance == 1 && y_distance == 1;

        // If the head is ever two steps directly up, down, left, or right from the tail, the tail must also move one
        // step in that direction so it remains close enough
        if x_distance == 2 && same_row {
            if head_x > tail_x {
                self.tail = self.tail.right();
            } else {
                self.tail = self.tail.left();
            }
        } else if y_distance == 2 && same_col {
            if head_y > tail_y {
                self.tail = self.tail.up();
            } else {
                self.tail = self.tail.down();
            }

        // if the head and tail aren't touching and aren't in the same row or column, the tail always moves one step
        // diagonally to keep up
        } else if !head_and_tail_touching && !same_row && !same_col {
            if head_x > tail_x {
                self.tail = self.tail.right();
            } else {
                self.tail = self.tail.left();
            }

            if head_y > tail_y {
                self.tail = self.tail.up();
            } else {
                self.tail = self.tail.down();
            }
        }
        self.tail_visits.insert(self.tail);
    }
}

fn part_1(input_string: String) -> usize {
    let mut rope = Rope::new();

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

    let tail_visits = part_1(input_string);

    println!("Part 1: {}", tail_visits);
    assert_eq!(tail_visits, 6357);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
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
        // assert_eq!(part_2(input_string.to_string()), 8);
    }
}
