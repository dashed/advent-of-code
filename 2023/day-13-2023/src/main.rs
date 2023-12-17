use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Terrain {
    Ash,
    Rock,
}

type Map = HashMap<Coordinate, Terrain>;
type Coordinate = (i32, i32);

trait Reflection {
    fn reflect_about_horizontal_line(&self, y_coord: i32) -> Coordinate;
    fn reflect_about_vertical_line(&self, x_coord: i32) -> Coordinate;
}

impl Reflection for Coordinate {
    fn reflect_about_horizontal_line(&self, y_coord: i32) -> Coordinate {
        let (x, y) = self;
        let distance = (y - y_coord).abs();

        if y < &y_coord {
            let reflected_y = y_coord + distance - 1;
            return (*x, reflected_y);
        }

        let reflected_y = y_coord - distance + 1;
        (*x, reflected_y)
    }

    // fn reflect_about_vertical_line(&self, x_coord: i32) -> Coordinate {
    //     let (x, y) = self;
    //     let reflected_x = 2 * x_coord - x;
    //     (reflected_x, *y)
    // }

    fn reflect_about_vertical_line(&self, x_coord: i32) -> Coordinate {
        let (x, y) = self;
        let distance = (x - x_coord).abs();

        if x < &x_coord {
            let reflected_x = x_coord + distance - 1;
            return (reflected_x, *y);
        }

        let reflected_x = x_coord + distance + 1;
        (reflected_x, *y)
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
    #[allow(dead_code)]
    fn to_str(&self) -> String {
        let mut output = String::new();
        for y_coord in 0..=self.max_y_coord {
            for x_coord in 0..=self.max_x_coord {
                let coordinate = (x_coord, y_coord);
                let terrain = self.map.get(&coordinate).unwrap();
                match terrain {
                    Terrain::Ash => output.push('.'),
                    Terrain::Rock => output.push('#'),
                }
            }
            output.push('\n');
        }
        output
    }

    #[allow(dead_code)]
    fn to_str_highlight(&self, highlight: Coordinate) -> String {
        let mut output = String::new();
        for y_coord in 0..=self.max_y_coord {
            for x_coord in 0..=self.max_x_coord {
                let coordinate = (x_coord, y_coord);
                if highlight == coordinate {
                    output.push('[');
                }
                let terrain = self.map.get(&coordinate).unwrap();
                match terrain {
                    Terrain::Ash => output.push('.'),
                    Terrain::Rock => output.push('#'),
                }
                if highlight == coordinate {
                    output.push(']');
                }
            }
            output.push('\n');
        }
        output
    }

    fn has_reflection_on_line(&self, coordinate: Coordinate) -> bool {
        let mut step = 0;

        let (x, y) = coordinate;
        assert!(x < self.max_x_coord);
        assert!(x >= 0);

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

            let reflected_terrain = self.map.get(&reflected_coordinate).unwrap();

            if current_terrain != reflected_terrain {
                return false;
            }

            step += 1;
        }

        true
    }

    fn has_reflection_on_column(&self, coordinate: Coordinate) -> bool {
        let mut step = 0;

        let (x, y) = coordinate;
        assert!(y < self.max_y_coord);
        assert!(y >= 0);

        loop {
            let current_coord = (x, y - step);
            let reflected_coordinate = (x, y + step + 1);

            if !current_coord.within_bounds(self.max_x_coord, self.max_y_coord) {
                break;
            }

            if !reflected_coordinate.within_bounds(self.max_x_coord, self.max_y_coord) {
                break;
            }

            let current_terrain = self.map.get(&current_coord).unwrap();
            let reflected_terrain = self.map.get(&reflected_coordinate).unwrap();

            if current_terrain != reflected_terrain {
                return false;
            }

            step += 1;
        }

        true
    }

    fn find_reflect_across_vertical_line(&self) -> i32 {
        // find a perfect reflection across a vertical line between two columns
        'x_coord_loop: for x_coord in (0..=(self.max_x_coord - 1)).rev() {
            for y_coord in 0..=self.max_y_coord {
                let coordinate = (x_coord, y_coord);
                if !self.has_reflection_on_line(coordinate) {
                    continue 'x_coord_loop;
                }
            }

            // found a perfect reflection
            // return the number of columns to the left of each vertical line of reflection
            return x_coord + 1;
        }
        // No perfect reflection found
        0
    }

    fn find_reflect_across_vertical_line_with_smudge(&self, smudge: Coordinate) -> i32 {
        // find a perfect reflection across a vertical line between two columns
        'x_coord_loop: for x_coord in (0..=(self.max_x_coord - 1)).rev() {
            let reflected_smudge = smudge.reflect_about_vertical_line(x_coord + 1);

            if !reflected_smudge.within_bounds(self.max_x_coord, self.max_y_coord) {
                continue 'x_coord_loop;
            }

            for y_coord in 0..=self.max_y_coord {
                let coordinate = (x_coord, y_coord);
                if !self.has_reflection_on_line(coordinate) {
                    continue 'x_coord_loop;
                }
            }

            // found a perfect reflection
            // return the number of columns to the left of each vertical line of reflection
            return x_coord + 1;
        }
        // No perfect reflection found
        0
    }

    fn find_and_fix_reflect_across_vertical_line(&self) -> i32 {
        let old_reflection = self.find_reflect_across_vertical_line();
        // find a perfect reflection across a vertical line between two columns
        for x_coord in 0..=self.max_x_coord {
            for y_coord in 0..=self.max_y_coord {
                let mut new_map = self.map.clone();
                let coordinate = (x_coord, y_coord);
                let terrain = new_map.get(&coordinate).unwrap().clone();
                match terrain {
                    Terrain::Ash => {
                        new_map.insert(coordinate, Terrain::Rock);
                    }
                    Terrain::Rock => {
                        new_map.insert(coordinate, Terrain::Ash);
                    }
                }

                let new_valley = Valley {
                    map: new_map,
                    max_y_coord: self.max_y_coord,
                    max_x_coord: self.max_x_coord,
                };

                let result = new_valley.find_reflect_across_vertical_line_with_smudge(coordinate);
                if result > 0 {
                    // fixed smudge needs to impact the result
                    // reflected smudge coordinate about fixed_x_coord
                    let reflected_coordinate = coordinate.reflect_about_vertical_line(result);
                    let reflected_x_coord = reflected_coordinate.0;

                    if reflected_x_coord >= result
                        && result != old_reflection
                        && reflected_coordinate.within_bounds(self.max_x_coord, self.max_y_coord)
                    {
                        return result;
                    }
                }
            }
        }
        0
    }

    fn find_reflect_across_horizontal_line(&self) -> i32 {
        // find a perfect reflection across a horizontal line between two rows
        'y_coord_loop: for y_coord in (0..=(self.max_y_coord - 1)).rev() {
            for x_coord in 0..=self.max_x_coord {
                let coordinate = (x_coord, y_coord);
                if !self.has_reflection_on_column(coordinate) {
                    continue 'y_coord_loop;
                }
            }

            // found a perfect reflection
            return y_coord + 1;
        }
        // No perfect reflection found
        0
    }

    fn find_reflect_across_horizontal_line_with_smudge(&self, smudge: Coordinate) -> i32 {
        // find a perfect reflection across a horizontal line between two rows
        'y_coord_loop: for y_coord in (0..=(self.max_y_coord - 1)).rev() {
            let reflected_smudge = smudge.reflect_about_horizontal_line(y_coord + 1);

            if !reflected_smudge.within_bounds(self.max_x_coord, self.max_y_coord) {
                continue 'y_coord_loop;
            }

            for x_coord in 0..=self.max_x_coord {
                let coordinate = (x_coord, y_coord);
                if !self.has_reflection_on_column(coordinate) {
                    continue 'y_coord_loop;
                }
            }

            // found a perfect reflection
            return y_coord + 1;
        }
        // No perfect reflection found
        0
    }

    fn find_and_fix_reflect_across_horizontal_line(&self) -> i32 {
        let old_reflection = self.find_reflect_across_horizontal_line();
        // find a perfect reflection across a horizontal line between two rows
        for y_coord in 0..=self.max_y_coord {
            for x_coord in 0..=self.max_x_coord {
                let mut new_map = self.map.clone();
                let coordinate = (x_coord, y_coord);
                let terrain = new_map.get(&coordinate).unwrap().clone();
                match terrain {
                    Terrain::Ash => {
                        new_map.insert(coordinate, Terrain::Rock);
                        assert!(new_map.get(&coordinate).unwrap() == &Terrain::Rock);
                        Terrain::Rock
                    }
                    Terrain::Rock => {
                        new_map.insert(coordinate, Terrain::Ash);
                        assert!(new_map.get(&coordinate).unwrap() == &Terrain::Ash);
                        Terrain::Ash
                    }
                };

                let new_valley = Valley {
                    map: new_map,
                    max_y_coord: self.max_y_coord,
                    max_x_coord: self.max_x_coord,
                };

                let result = new_valley.find_reflect_across_horizontal_line_with_smudge(coordinate);

                if result > 0 {
                    // fixed smudge needs to impact the result
                    // reflected smudge coordinate about fixed_y_coord
                    let reflected_coordinate = coordinate.reflect_about_horizontal_line(result);
                    let reflected_y_coord = reflected_coordinate.1;

                    if reflected_y_coord >= result
                        && result != old_reflection
                        && reflected_coordinate.within_bounds(self.max_x_coord, self.max_y_coord)
                    {
                        return result;
                    }
                }
            }
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
    let maps = generate_maps(input_string);

    maps.into_iter()
        .map(|valley| {
            let horizontal = valley.find_reflect_across_horizontal_line();

            if horizontal > 0 {
                return horizontal * 100;
            }

            valley.find_reflect_across_vertical_line()
        })
        .sum::<i32>() as i32
}

fn part_2(input_string: &str) -> i32 {
    let maps = generate_maps(input_string);

    maps.into_iter()
        .map(|valley| {
            let horizontal = valley.find_and_fix_reflect_across_horizontal_line();

            if horizontal > 0 {
                return horizontal * 100;
            }

            

            // assert!(
            //     result > 0,
            //     "No result found for valley:\n{}",
            //     valley.to_str()
            // );

            valley.find_and_fix_reflect_across_vertical_line()
        })
        .sum::<i32>() as i32
}

fn main() {
    let input_string = include_str!("input.txt");

    // Part 1

    let answer = part_1(input_string);
    println!("Part 1: {}", answer);
    assert_eq!(answer, 29213);

    // Part 2

    let answer = part_2(input_string);
    println!("Part 2: {}", answer);
    assert_eq!(answer, 37453);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        //         let input_string = r###"
        //                 ...#.###...
        //                 ###.##.##.#
        //                 #...#.##.#.
        //                 .####..####
        //                 ##..###.#..
        //                 .#.#..#.#.#
        //                 #....####.#
        //                 #....####.#
        //                 .#.#..#.#.#
        //                 ##..###.#..
        //                 .####..#.##
        //                 #...#.##.#.
        //                 ###.##.##.#
        //                 ...#.###...
        //                 ...#.###...
        //                 "###;

        //         assert_eq!(part_2(input_string), 700);

        //         let input_string = r###"
        //         #...##..#
        //         #....#..#
        //         ..##..###
        //         #####.##.
        //         #####.##.
        //         ..##..###
        //         #....#..#
        //         "###;

        //         assert_eq!(part_2(input_string), 100);

        //         let input_string = r###"
        // .#.##.#.#
        // .##..##..
        // .#.##.#..
        // #......##
        // #......##
        // .#.##.#..
        // .##..##.#
        // "###;

        //         assert_eq!(part_2(input_string), 400);

        //         let input_string = r###"
        // #..#....#
        // ###..##..
        // .##.#####
        // .##.#####
        // ###..##..
        // #..#....#
        // #..##...#
        // "###;

        //         assert_eq!(part_2(input_string), 600);

        let input_string = r###"
...#.#.##
.#####.##
.#.##.#..
.#.##.#..
.#####.##
...#.#.##
###..##..
####.####
#..#.#.#.
##...#...
...###...
#####....
#..##..##
"###;

        assert_eq!(part_2(input_string), 8);
    }

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
        assert_eq!(part_2(input_string), 400);

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

.#.##.#.#
.##..##..
.#.##.#..
#......##
#......##
.#.##.#..
.##..##.#

#..#....#
###..##..
.##.#####
.##.#####
###..##..
#..#....#
#..##...#

#.##..##.
..#.##.#.
##..#...#
##...#..#
..#.##.#.
..##..##.
#.#.##.#.
"###;

        assert_eq!(part_1(input_string), 709);
        assert_eq!(part_2(input_string), 1400);

        let input_string = r###"
        ###.##.##
        ##.####.#
        ##.#..#.#
        ####..###
        ....##...
        ##.#..#.#
        ...#..#..
        ##..###.#
        ##......#
        ##......#
        ..#.##.#.
        ...#..#..
        ##.####.#
        ....##...
        ...####..
        ....##...
        ##.####.#
        "###;

        assert_eq!(part_1(input_string), 1);
        assert_eq!(part_2(input_string), 5);

        let input_string = r###"
        .##.##...##...##.
        #####..##..##..##
        .....##..##..##..
        .##.#.#.####.#.#.
        .##...#.#..#.#...
        ....#..........#.
        #..#..#......#..#
        ....###.....####.
        .##...#.#..#.#...
        .....#..####..#..
        #..#...##..##...#
        ....#...#..#...#.
        #..#.##########.#
        #..##...####...##
        #####.##.##.##.##
        "###;

        assert_eq!(part_1(input_string), 2);
        assert_eq!(part_2(input_string), 10);
    }
}
