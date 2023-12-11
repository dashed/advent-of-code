use itertools::Itertools;
use std::collections::HashSet;

type Coordinate = (i64, i64);

// https://en.wikipedia.org/wiki/Taxicab_geometry
fn get_manhattan_distance(start: Coordinate, end: Coordinate) -> i64 {
    let (a, b) = start;
    let (c, d) = end;

    (a - c).abs() + (b - d).abs()
}

type Map = HashSet<Coordinate>;

#[derive(Debug, Clone)]
struct Space {
    galaxies: Map,
    max_y_coord: i64,
    max_x_coord: i64,
    covered_x_coords: HashSet<i64>,
    covered_y_coords: HashSet<i64>,
}

impl Space {
    fn expand_x_coord(&mut self, x_coord: i64, expansion_increment: i64) {
        let x_coord_start = x_coord;

        for galaxy_coord in self.galaxies.clone().into_iter() {
            let (x_coord, y_coord) = galaxy_coord;
            if x_coord < x_coord_start {
                continue;
            }
            assert!(x_coord != x_coord_start);

            let new_x_coord: i64 = x_coord + expansion_increment;
            self.galaxies.remove(&(x_coord, y_coord));
            self.galaxies.insert((new_x_coord, y_coord));

            self.covered_x_coords.insert(new_x_coord);
            self.covered_x_coords.remove(&x_coord_start);
        }

        self.max_x_coord += expansion_increment;
    }

    fn expand_y_coord(&mut self, y_coord: i64, expansion_increment: i64) {
        let y_coord_start = y_coord;

        for galaxy_coord in self.galaxies.clone().into_iter() {
            let (x_coord, y_coord) = galaxy_coord;
            if y_coord < y_coord_start {
                continue;
            }
            assert!(y_coord != y_coord_start);

            let new_y_coord: i64 = y_coord + expansion_increment;
            self.galaxies.remove(&(x_coord, y_coord));
            self.galaxies.insert((x_coord, new_y_coord));

            self.covered_y_coords.insert(new_y_coord);
            self.covered_y_coords.remove(&y_coord_start);
        }

        self.max_y_coord += expansion_increment;
    }

    fn expand_space(&mut self, expansion_increment: i64) {
        for x_coord in (0..=self.max_x_coord).rev() {
            if self.covered_x_coords.contains(&x_coord) {
                continue;
            }
            self.expand_x_coord(x_coord, expansion_increment);
        }

        for y_coord in (0..=self.max_y_coord).rev() {
            if self.covered_y_coords.contains(&y_coord) {
                continue;
            }
            self.expand_y_coord(y_coord, expansion_increment);
        }
    }
}

fn create_space(input_string: &str) -> Space {
    let inputs: Vec<&str> = input_string.trim().lines().collect();
    let max_y_coord = inputs.len() as i64 - 1;
    let max_x_coord = inputs[0].len() as i64 - 1;

    let mut galaxies = Map::new();

    let mut covered_x_coords: HashSet<i64> = HashSet::new();
    let mut covered_y_coords: HashSet<i64> = HashSet::new();

    for (y_coord, line) in inputs.iter().enumerate() {
        for (x_coord, character) in line.trim().chars().enumerate() {
            let coordinate = (x_coord as i64, y_coord as i64);
            match character {
                '#' => {
                    covered_y_coords.insert(y_coord as i64);
                    covered_x_coords.insert(x_coord as i64);
                    galaxies.insert(coordinate);
                }
                '.' => {
                    // Do nothing
                }
                _ => {
                    panic!("Unexpected character: {}", character);
                }
            }
        }
    }

    Space {
        galaxies,
        max_x_coord,
        max_y_coord,
        covered_x_coords,
        covered_y_coords,
    }
}

fn part_1(input_string: &str) -> i64 {
    let mut space = create_space(input_string);
    space.expand_space(1);

    let mut total_distance = 0;
    for combos in space.galaxies.iter().combinations(2) {
        assert!(combos.len() == 2);
        let (galaxy_1, galaxy_2) = (combos[0], combos[1]);
        let distance = get_manhattan_distance(*galaxy_1, *galaxy_2);
        // println!("Distance: {}", distance);
        total_distance += distance;
    }

    total_distance
}

fn part_2(input_string: &str, expansion_increment: i64) -> i64 {
    let mut space = create_space(input_string);
    space.expand_space(expansion_increment - 1);

    let mut total_distance = 0;
    for combos in space.galaxies.iter().combinations(2) {
        assert!(combos.len() == 2);
        let (galaxy_1, galaxy_2) = (combos[0], combos[1]);
        let distance = get_manhattan_distance(*galaxy_1, *galaxy_2);
        // println!("Distance: {}", distance);
        total_distance += distance;
    }

    total_distance
}

fn main() {
    let input_string = include_str!("input.txt");

    // Part 1

    let answer = part_1(input_string);
    println!("Part 1: {}", answer);
    assert_eq!(answer, 10077850);

    // Part 2

    let answer = part_2(input_string, 1_000_000);
    println!("Part 2: {}", answer);
    assert_eq!(answer, 504715068438);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle() {
        let input_string = r###"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"###;

        assert_eq!(part_1(input_string), 374);
        assert_eq!(part_2(input_string, 10), 1030);
        assert_eq!(part_2(input_string, 100), 8410);
    }
}
