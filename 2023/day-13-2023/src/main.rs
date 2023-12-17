use std::collections::HashMap;

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

struct Valley {
    map: Map,
    max_y_coord: i32,
    max_x_coord: i32,
}

fn generate_map(input: Vec<&str>) -> Map {
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

    map
}

fn generate_maps(input_string: &str) -> Vec<Map> {
    let mut maps = vec![];

    let inputs: Vec<&str> = input_string.trim().lines().collect();
    let mut buffer: Vec<&str> = vec![];

    for input in inputs {

        let input = input.trim();
        if input.is_empty() && buffer.len() > 0 {
            let map = generate_map(buffer);
            maps.push(map);
            buffer = vec![];
        } else {
            buffer.push(input);
        }
    }

    if buffer.len() > 0 {
        let map = generate_map(buffer);
        maps.push(map);
    }

    maps
}

fn part_1(input_string: &str) -> i64 {
    let inputs: Vec<&str> = input_string.trim().lines().collect();

    let maps = generate_maps(input_string);
    0
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
        // assert_eq!(part_2(input_string, 10), 1030);
        // assert_eq!(part_2(input_string, 100), 8410);
    }
}
