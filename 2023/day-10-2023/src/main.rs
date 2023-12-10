use std::{
    collections::{HashMap, HashSet},
    vec,
};

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

#[derive(Debug, Clone, PartialEq, Eq)]
enum Terrain {
    Ground,
    VerticalPipe,
    HorizontalPipe,
    // L is a 90-degree bend connecting north and east.
    NorthEastPipe,
    // J is a 90-degree bend connecting north and west.
    NorthWestPipe,
    // 7 is a 90-degree bend connecting south and west.
    SouthWestPipe,
    // F is a 90-degree bend connecting south and east.
    SouthEastPipe,
}

impl Terrain {
    fn from_char(character: char) -> Self {
        match character {
            '.' => Terrain::Ground,
            '|' => Terrain::VerticalPipe,
            '-' => Terrain::HorizontalPipe,
            'L' => Terrain::NorthEastPipe,
            'J' => Terrain::NorthWestPipe,
            '7' => Terrain::SouthWestPipe,
            'F' => Terrain::SouthEastPipe,
            _ => panic!("Unknown character: {}", character),
        }
    }
}

type Map = HashMap<Coordinate, Terrain>;

#[derive(Debug)]
struct MetalIsland {
    terrain: Map,
    starting_coordinate: Coordinate,
    max_y_coord: i32,
    max_x_coord: i32,
}

impl MetalIsland {
    fn new(input_string: &str) -> Self {
        let inputs: Vec<&str> = input_string.trim().lines().collect();

        let max_y_coord = inputs.len() as i32 - 1;
        let max_x_coord = inputs[0].len() as i32 - 1;
        let mut terrain = Map::new();

        let mut starting_coordinate: Option<Coordinate> = None;

        for (y_coord, line) in inputs.iter().enumerate() {
            for (x_coord, character) in line.trim().chars().enumerate() {
                let coordinate = (x_coord as i32, y_coord as i32);
                match character {
                    '.' | '|' | '-' | 'L' | 'J' | '7' | 'F' => {
                        terrain.insert(coordinate, Terrain::from_char(character));
                    }
                    'S' => {
                        starting_coordinate = Some(coordinate);
                    }
                    _ => {
                        panic!("Unknown character: {}", character);
                    }
                }
            }
        }

        assert!(starting_coordinate.is_some());
        if let Some(starting_coordinate) = starting_coordinate {
            let possibilities: Vec<(&str, Coordinate, Vec<Terrain>)> = vec![
                (
                    "north",
                    starting_coordinate.north(),
                    vec![
                        Terrain::VerticalPipe,
                        Terrain::SouthWestPipe,
                        Terrain::SouthEastPipe,
                    ],
                ),
                (
                    "south",
                    starting_coordinate.south(),
                    vec![
                        Terrain::VerticalPipe,
                        Terrain::NorthWestPipe,
                        Terrain::NorthEastPipe,
                    ],
                ),
                (
                    "west",
                    starting_coordinate.west(),
                    vec![
                        Terrain::HorizontalPipe,
                        Terrain::NorthEastPipe,
                        Terrain::SouthEastPipe,
                    ],
                ),
                (
                    "east",
                    starting_coordinate.east(),
                    vec![
                        Terrain::HorizontalPipe,
                        Terrain::NorthWestPipe,
                        Terrain::SouthWestPipe,
                    ],
                ),
            ];

            let possibilities: Vec<&str> = possibilities
                .into_iter()
                .filter(|(_, coordinate, _)| coordinate.within_bounds(max_x_coord, max_y_coord))
                .filter(|(_, coordinate, terrain_filters)| {
                    let actual_terrain = terrain.get(coordinate).unwrap();
                    // check if terrain_filters is in terrain_filters
                    terrain_filters.contains(actual_terrain)
                })
                .map(|(path, _, _)| path)
                .collect();
            assert!(possibilities.len() == 2);

            let start_terrain =
                if possibilities.contains(&"north") && possibilities.contains(&"south") {
                    Terrain::VerticalPipe
                } else if possibilities.contains(&"west") && possibilities.contains(&"east") {
                    Terrain::HorizontalPipe
                } else if possibilities.contains(&"north") && possibilities.contains(&"east") {
                    Terrain::NorthEastPipe
                } else if possibilities.contains(&"north") && possibilities.contains(&"west") {
                    Terrain::NorthWestPipe
                } else if possibilities.contains(&"south") && possibilities.contains(&"west") {
                    Terrain::SouthWestPipe
                } else if possibilities.contains(&"south") && possibilities.contains(&"east") {
                    Terrain::SouthEastPipe
                } else {
                    panic!("Unknown possibilities: {:?}", possibilities);
                };

            terrain.insert(starting_coordinate, start_terrain);
        }

        MetalIsland {
            terrain,
            starting_coordinate: starting_coordinate.unwrap(),
            max_y_coord,
            max_x_coord,
        }
    }

    fn is_valid_coordinate(&self, coordinate: &Coordinate) -> bool {
        coordinate.within_bounds(self.max_x_coord, self.max_y_coord)
    }

    fn get_terrain(&self, coordinate: &Coordinate) -> Option<&Terrain> {
        self.terrain.get(coordinate)
    }

    fn farthest_point(&self) -> i64 {
        let mut max_distance = 0;

        let mut visited_coords: HashSet<Coordinate> = HashSet::new();
        // vector of (coordinate, distance)
        let mut coords_to_visit: Vec<(Coordinate, i64)> = vec![(self.starting_coordinate, 0)];

        loop {
            if coords_to_visit.is_empty() {
                break;
            }

            let (current_coordinate, distance) = coords_to_visit.remove(0);
            visited_coords.insert(current_coordinate);

            if distance > max_distance {
                max_distance = distance;
            }

            let current_terrain = self.get_terrain(&current_coordinate).unwrap();

            match current_terrain {
                Terrain::Ground => {
                    panic!("Ground at {:?}", current_coordinate);
                }
                Terrain::VerticalPipe => {
                    let north_coordinate = current_coordinate.north();
                    let south_coordinate = current_coordinate.south();
                    if self.is_valid_coordinate(&north_coordinate)
                        && !visited_coords.contains(&north_coordinate)
                    {
                        coords_to_visit.push((north_coordinate, distance + 1));
                    }
                    if self.is_valid_coordinate(&south_coordinate)
                        && !visited_coords.contains(&south_coordinate)
                    {
                        coords_to_visit.push((south_coordinate, distance + 1));
                    }
                }
                Terrain::HorizontalPipe => {
                    let west_coordinate = current_coordinate.west();
                    let east_coordinate = current_coordinate.east();
                    if self.is_valid_coordinate(&west_coordinate)
                        && !visited_coords.contains(&west_coordinate)
                    {
                        coords_to_visit.push((west_coordinate, distance + 1));
                    }
                    if self.is_valid_coordinate(&east_coordinate)
                        && !visited_coords.contains(&east_coordinate)
                    {
                        coords_to_visit.push((east_coordinate, distance + 1));
                    }
                }
                Terrain::NorthEastPipe => {
                    let north_coordinate = current_coordinate.north();
                    let east_coordinate = current_coordinate.east();
                    if self.is_valid_coordinate(&north_coordinate)
                        && !visited_coords.contains(&north_coordinate)
                    {
                        coords_to_visit.push((north_coordinate, distance + 1));
                    }
                    if self.is_valid_coordinate(&east_coordinate)
                        && !visited_coords.contains(&east_coordinate)
                    {
                        coords_to_visit.push((east_coordinate, distance + 1));
                    }
                }
                Terrain::NorthWestPipe => {
                    let north_coordinate = current_coordinate.north();
                    let west_coordinate = current_coordinate.west();
                    if self.is_valid_coordinate(&north_coordinate)
                        && !visited_coords.contains(&north_coordinate)
                    {
                        coords_to_visit.push((north_coordinate, distance + 1));
                    }
                    if self.is_valid_coordinate(&west_coordinate)
                        && !visited_coords.contains(&west_coordinate)
                    {
                        coords_to_visit.push((west_coordinate, distance + 1));
                    }
                }
                Terrain::SouthWestPipe => {
                    let south_coordinate = current_coordinate.south();
                    let west_coordinate = current_coordinate.west();
                    if self.is_valid_coordinate(&south_coordinate)
                        && !visited_coords.contains(&south_coordinate)
                    {
                        coords_to_visit.push((south_coordinate, distance + 1));
                    }
                    if self.is_valid_coordinate(&west_coordinate)
                        && !visited_coords.contains(&west_coordinate)
                    {
                        coords_to_visit.push((west_coordinate, distance + 1));
                    }
                }
                Terrain::SouthEastPipe => {
                    let south_coordinate = current_coordinate.south();
                    let east_coordinate = current_coordinate.east();
                    if self.is_valid_coordinate(&south_coordinate)
                        && !visited_coords.contains(&south_coordinate)
                    {
                        coords_to_visit.push((south_coordinate, distance + 1));
                    }
                    if self.is_valid_coordinate(&east_coordinate)
                        && !visited_coords.contains(&east_coordinate)
                    {
                        coords_to_visit.push((east_coordinate, distance + 1));
                    }
                }
            }
        }

        max_distance
    }

    fn loop_coords(&self) -> HashSet<Coordinate> {
        let mut visited_coords: HashSet<Coordinate> = HashSet::new();
        // vector of (coordinate, distance)
        let mut coords_to_visit: Vec<(Coordinate, i64)> = vec![(self.starting_coordinate, 0)];

        loop {
            if coords_to_visit.is_empty() {
                break;
            }

            let (current_coordinate, distance) = coords_to_visit.remove(0);
            visited_coords.insert(current_coordinate);

            let current_terrain = self.get_terrain(&current_coordinate).unwrap();

            match current_terrain {
                Terrain::Ground => {
                    panic!("Ground at {:?}", current_coordinate);
                }
                Terrain::VerticalPipe => {
                    let north_coordinate = current_coordinate.north();
                    let south_coordinate = current_coordinate.south();
                    if self.is_valid_coordinate(&north_coordinate)
                        && !visited_coords.contains(&north_coordinate)
                    {
                        coords_to_visit.push((north_coordinate, distance + 1));
                    }
                    if self.is_valid_coordinate(&south_coordinate)
                        && !visited_coords.contains(&south_coordinate)
                    {
                        coords_to_visit.push((south_coordinate, distance + 1));
                    }
                }
                Terrain::HorizontalPipe => {
                    let west_coordinate = current_coordinate.west();
                    let east_coordinate = current_coordinate.east();
                    if self.is_valid_coordinate(&west_coordinate)
                        && !visited_coords.contains(&west_coordinate)
                    {
                        coords_to_visit.push((west_coordinate, distance + 1));
                    }
                    if self.is_valid_coordinate(&east_coordinate)
                        && !visited_coords.contains(&east_coordinate)
                    {
                        coords_to_visit.push((east_coordinate, distance + 1));
                    }
                }
                Terrain::NorthEastPipe => {
                    let north_coordinate = current_coordinate.north();
                    let east_coordinate = current_coordinate.east();
                    if self.is_valid_coordinate(&north_coordinate)
                        && !visited_coords.contains(&north_coordinate)
                    {
                        coords_to_visit.push((north_coordinate, distance + 1));
                    }
                    if self.is_valid_coordinate(&east_coordinate)
                        && !visited_coords.contains(&east_coordinate)
                    {
                        coords_to_visit.push((east_coordinate, distance + 1));
                    }
                }
                Terrain::NorthWestPipe => {
                    let north_coordinate = current_coordinate.north();
                    let west_coordinate = current_coordinate.west();
                    if self.is_valid_coordinate(&north_coordinate)
                        && !visited_coords.contains(&north_coordinate)
                    {
                        coords_to_visit.push((north_coordinate, distance + 1));
                    }
                    if self.is_valid_coordinate(&west_coordinate)
                        && !visited_coords.contains(&west_coordinate)
                    {
                        coords_to_visit.push((west_coordinate, distance + 1));
                    }
                }
                Terrain::SouthWestPipe => {
                    let south_coordinate = current_coordinate.south();
                    let west_coordinate = current_coordinate.west();
                    if self.is_valid_coordinate(&south_coordinate)
                        && !visited_coords.contains(&south_coordinate)
                    {
                        coords_to_visit.push((south_coordinate, distance + 1));
                    }
                    if self.is_valid_coordinate(&west_coordinate)
                        && !visited_coords.contains(&west_coordinate)
                    {
                        coords_to_visit.push((west_coordinate, distance + 1));
                    }
                }
                Terrain::SouthEastPipe => {
                    let south_coordinate = current_coordinate.south();
                    let east_coordinate = current_coordinate.east();
                    if self.is_valid_coordinate(&south_coordinate)
                        && !visited_coords.contains(&south_coordinate)
                    {
                        coords_to_visit.push((south_coordinate, distance + 1));
                    }
                    if self.is_valid_coordinate(&east_coordinate)
                        && !visited_coords.contains(&east_coordinate)
                    {
                        coords_to_visit.push((east_coordinate, distance + 1));
                    }
                }
            }
        }

        visited_coords
    }

    fn num_of_enclosed_tiles(&self) -> i64 {
        let loop_coords = self.loop_coords();

        let mut num_of_enclosed_tiles = 0;

        for y_coord in 0..=self.max_y_coord {
            let mut is_inside_loop = false;

            for x_coord in 0..=self.max_x_coord {
                let coordinate = (x_coord, y_coord);
                let terrain = self.get_terrain(&coordinate).unwrap();
                let is_loop_coord = loop_coords.contains(&coordinate);

                if is_inside_loop && !is_loop_coord {
                    num_of_enclosed_tiles += 1;
                }

                // For convention, let's say the direction is pointing up, and that right side is the inside of the loop.
                // We flip the is_inside_loop flag when we encounter a vertical pipe.
                // For pipes that connect to either east or west, we flip the is_inside_loop flag when we encounter
                // NorthEastPipe and NorthWestPipe.

                match terrain {
                    Terrain::Ground => {}
                    Terrain::VerticalPipe => {
                        if is_loop_coord {
                            is_inside_loop = !is_inside_loop;
                        }
                    }
                    Terrain::HorizontalPipe => {
                        // do nothing
                    }
                    Terrain::NorthEastPipe => {
                        if is_loop_coord {
                            is_inside_loop = !is_inside_loop;
                        }
                    }
                    Terrain::NorthWestPipe => {
                        if is_loop_coord {
                            is_inside_loop = !is_inside_loop;
                        }
                    }
                    Terrain::SouthWestPipe => {}
                    Terrain::SouthEastPipe => {}
                }
            }
        }

        num_of_enclosed_tiles
    }
}

fn part_1(input_string: &str) -> i64 {
    let island = MetalIsland::new(input_string);
    island.farthest_point()
}

fn part_2(input_string: &str) -> i64 {
    let island = MetalIsland::new(input_string);
    island.num_of_enclosed_tiles()
}

fn main() {
    let input_string = include_str!("input.txt");

    // Part 1

    let answer = part_1(input_string);
    println!("Part 1: {}", answer);
    assert_eq!(answer, 6786);

    // Part 2

    let answer = part_2(input_string);
    println!("Part 2: {}", answer);
    assert_eq!(answer, 495);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle() {
        let input_string = r###"
.....
.S-7.
.|.|.
.L-J.
.....
"###;

        assert_eq!(part_1(input_string), 4);

        let input_string = r###"
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
        "###;

        assert_eq!(part_1(input_string), 8);

        let input_string = r###"
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
        "###;

        assert_eq!(part_2(input_string), 4);

        let input_string = r###"
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
        "###;

        assert_eq!(part_2(input_string), 8);

        let input_string = r###"
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
        "###;

        assert_eq!(part_2(input_string), 10);
    }
}
