use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Terrain {
    Ash,
    Rock,
}

type Map = HashMap<Coordinate, Terrain>;
type Coordinate = (i32, i32);

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

struct Valley {
    map: Map,
    max_y_coord: i32,
    max_x_coord: i32,
}

impl Valley {
    fn has_reflection_on_line(&self, coordinate: Coordinate) -> bool {
        let mut step = 0;

        let (x, y) = coordinate;
        assert!(x <= self.max_x_coord);
        assert!(x >= 1);

        loop {
            let current_coord = (x - step, y);
            let reflected_coordinate = (x + step + 1, y);

            if !current_coord.within_bounds(self.max_x_coord, self.max_y_coord) {
                break;
            }

            if !reflected_coordinate.within_bounds(self.max_x_coord, self.max_y_coord) {
                break;
            }

            let current_terrain = self.map.get(&current_coord).unwrap();
            // println!("c: {:?}: {:?}", current_coord, current_terrain);

            let reflected_terrain = self.map.get(&reflected_coordinate).unwrap();
            // println!("r: {:?}: {:?}", reflected_coordinate, reflected_terrain);

            if current_terrain != reflected_terrain {
                return false;
            }

            step += 1;
        }

        true
    }

    fn find_reflect_across_verticle_line(&self) -> i32 {
        // find a perfect reflection across a vertical line between two columns
        'x_coord_loop: for x_coord in 1..=(self.max_x_coord - 1) {
            for y_coord in 0..=self.max_y_coord {
                let coordinate = (x_coord, y_coord);
                if !self.has_reflection_on_line(coordinate) {
                    continue 'x_coord_loop;
                }
            }

            // found a perfect reflection
            // return the number of columns to the left of each vertical line of reflection
            println!("Found perfect reflection at x_coord: {}", x_coord);
            return x_coord + 1;
        }
        // No perfect reflection found
        0
    }
}

fn generate_valley(input: Vec<&str>) -> Valley {
    let max_y_coord = input.len() as i32 - 1;
    let max_x_coord = input[0].len() as i32 - 1;
    let mut map = Map::new();

    for (y_coord, line) in input.iter().enumerate() {
        for (x_coord, character) in line.trim().chars().enumerate() {
            let coordinate = (x_coord as i32, y_coord as i32);
            match character {
                '#' => {
                    map.insert(coordinate, Terrain::Rock);
                }
                '.' => {
                    map.insert(coordinate, Terrain::Ash);
                }
                _ => panic!("Unknown character"),
            }
        }
    }

    Valley {
        map,
        max_y_coord,
        max_x_coord,
    }
}

fn generate_maps(input_string: &str) -> Vec<Valley> {
    let mut maps = vec![];

    let inputs: Vec<&str> = input_string.trim().lines().collect();
    let mut buffer: Vec<&str> = vec![];

    for input in inputs {
        let input = input.trim();
        if input.is_empty() && !buffer.is_empty() {
            let map = generate_valley(buffer);
            maps.push(map);
            buffer = vec![];
        } else {
            buffer.push(input);
        }
    }

    if !buffer.is_empty() {
        let map = generate_valley(buffer);
        maps.push(map);
    }

    maps
}

fn part_1(input_string: &str) -> i32 {
    let _inputs: Vec<&str> = input_string.trim().lines().collect();

    let maps = generate_maps(input_string);

    maps.into_iter()
        .map(|valley| valley.find_reflect_across_verticle_line())
        .sum::<i32>() as i32
}

fn main() {
    let input_string = include_str!("input.txt");

    // Part 1

    let answer = part_1(input_string);
    println!("Part 1: {}", answer);
    assert_eq!(answer, 7204);

    // Part 2

    // let answer = part_2(input_string);
    // println!("Part 2: {}", answer);
    // assert_eq!(answer, 1672318386674);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle() {
        let input_string = r###"
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"###;

        assert_eq!(part_1(input_string), 405);
    }
}
