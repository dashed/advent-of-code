// https://adventofcode.com/2022/day/8

use std::collections::{HashMap, HashSet};

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
        (*x, y - 1)
    }

    fn down(&self) -> Coordinate {
        let (x, y) = self;
        (*x, y + 1)
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

type Height = i32;
#[derive(Debug, Clone)]
struct Patch {
    area: HashMap<Coordinate, Height>,
    max_y: i32,
    max_x: i32,
}

impl Patch {
    fn new() -> Self {
        Patch {
            area: HashMap::new(),
            max_y: 0,
            max_x: 0,
        }
    }

    fn insert(&mut self, position: Coordinate, height: i32) {
        let (x, y) = position;

        if x > self.max_x {
            self.max_x = x;
        }

        if y > self.max_y {
            self.max_y = y;
        }

        self.area.insert(position, height);
    }

    fn get_viewing_distance<F>(&self, coord: Coordinate, generate_coord: F) -> usize
    where
        F: Fn(Coordinate) -> Coordinate,
    {
        let mut viewing_distance = 0;
        let mut current_viewing_coord = generate_coord(coord);
        let current_height = *self.area.get(&coord).unwrap();
        loop {
            let tree = self.area.get(&current_viewing_coord);
            if tree.is_none() {
                break;
            }
            viewing_distance += 1;
            let tree_height = *tree.unwrap();
            if tree_height >= current_height {
                break;
            }

            current_viewing_coord = generate_coord(current_viewing_coord);
        }

        viewing_distance
    }
}

fn generate_patch(input_string: String) -> Patch {
    let mut patch = Patch::new();

    for (y, line) in input_string.trim().lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        for (x, height) in line.chars().enumerate() {
            let position: Coordinate = (x as i32, y as i32);
            let height = height.to_digit(10).unwrap();
            patch.insert(position, height as i32);
        }
    }

    patch
}

fn part_1(input_string: String) -> usize {
    let patch = generate_patch(input_string);

    let mut visible_trees: HashSet<Coordinate> = HashSet::new();

    let mut test_visibility = |coord: Coordinate, tallest_tree: &mut Option<Height>| {
        let height = *patch.area.get(&coord).unwrap();
        match tallest_tree {
            None => {
                visible_trees.insert(coord);
                *tallest_tree = Some(height);
            }
            Some(current_height) => {
                if height > *current_height {
                    visible_trees.insert(coord);
                    *tallest_tree = Some(height);
                }
            }
        };
    };

    {
        // scan from left to right
        let mut tallest_tree: Option<Height>;
        for y in 0..=patch.max_y {
            tallest_tree = None;
            for x in 0..=patch.max_x {
                let coord: Coordinate = (x, y);
                test_visibility(coord, &mut tallest_tree);
            }
        }
    }

    {
        // scan from right to left
        let mut tallest_tree: Option<Height>;
        for y in 0..=patch.max_y {
            tallest_tree = None;
            for x in (0..=patch.max_x).rev() {
                let coord: Coordinate = (x, y);
                test_visibility(coord, &mut tallest_tree);
            }
        }
    }

    {
        // scan from up to down
        let mut tallest_tree: Option<Height>;
        for x in 0..=patch.max_x {
            tallest_tree = None;
            for y in 0..=patch.max_y {
                let coord: Coordinate = (x, y);
                test_visibility(coord, &mut tallest_tree);
            }
        }
    }

    {
        // scan from down to up
        let mut tallest_tree: Option<Height>;
        for x in 0..=patch.max_x {
            tallest_tree = None;
            for y in (0..=patch.max_y).rev() {
                let coord: Coordinate = (x, y);
                test_visibility(coord, &mut tallest_tree);
            }
        }
    }

    visible_trees.len()
}

fn part_2(input_string: String) -> usize {
    let patch = generate_patch(input_string);

    let mut best_scenic_score = 0;

    for y in 0..=patch.max_y {
        for x in 0..=patch.max_x {
            let coord: Coordinate = (x, y);
            let up = patch.get_viewing_distance(coord, |x| x.up());
            let left = patch.get_viewing_distance(coord, |x| x.left());
            let right = patch.get_viewing_distance(coord, |x| x.right());
            let down = patch.get_viewing_distance(coord, |x| x.down());
            let scenic_score = up * left * right * down;

            if scenic_score > best_scenic_score {
                best_scenic_score = scenic_score;
            }
        }
    }
    best_scenic_score
}

fn main() {
    let input_string = include_str!("input.txt");
    let part_1_result = part_1(input_string.to_string());
    println!("Part 1: {}", part_1_result);
    assert_eq!(part_1_result, 1695);

    let part_2_result = part_2(input_string.to_string());
    println!("Part 2: {}", part_2_result);
    assert_eq!(part_2_result, 287040);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input_string = r###"
30373
25512
65332
33549
35390
"###
        .to_string();

        assert_eq!(part_1(input_string.to_string()), 21);
        assert_eq!(part_2(input_string.to_string()), 8);
    }
}
