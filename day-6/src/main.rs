// https://adventofcode.com/2018/day/6

// imports

use std::collections::HashMap;
use std::collections::HashSet;

// helpers

type Position = (i32, i32);
type Destination = Position;
type Distance = i32;

// set of Positions (i.e. grid coordinates) that belong to a region
type Region = HashSet<Position>;
// mapping a Region belonging to a Destination
type Regions = HashMap<Destination, Region>;

enum GridPositionState {
    FreeClaim,
    // current position belongs to region of given Destination and distance
    Region(HashSet<Destination>, Distance),
}

// https://math.stackexchange.com/a/139604/10247
fn get_manhattan_distance(x: Position, y: Position) -> i32 {
    let (a, b) = x;
    let (c, d) = y;

    return (a - c).abs() + (b - d).abs();
}

fn parse_to_coord(input: &str) -> Position {
    let result: Vec<i32> = input
        .split(',')
        .map(|x| -> i32 { x.trim().parse().unwrap() })
        .collect();

    (*result.get(0).unwrap(), *result.get(1).unwrap())
}

fn get_x(src: Position) -> i32 {
    let (x, _y) = src;
    return x;
}

fn get_y(src: Position) -> i32 {
    let (_x, y) = src;
    return y;
}

fn is_better_top_edge(reference: Position, target: Position) -> bool {
    let ref_point = get_y(reference);
    let target_point = get_y(target);
    return target_point > ref_point;
}

fn is_better_bottom_edge(reference: Position, target: Position) -> bool {
    let ref_point = get_y(reference);
    let target_point = get_y(target);
    return target_point < ref_point;
}

fn is_better_left_edge(reference: Position, target: Position) -> bool {
    let ref_point = get_x(reference);
    let target_point = get_x(target);
    return target_point < ref_point;
}

fn is_better_right_edge(reference: Position, target: Position) -> bool {
    let ref_point = get_x(reference);
    let target_point = get_x(target);
    return target_point > ref_point;
}

struct GridPoints {
    current: Position,
    bounding_box: BoundingBox,
}

impl Iterator for GridPoints {
    type Item = Position;

    fn next(&mut self) -> Option<Position> {
        let (x, y) = self.current;

        // invariants
        assert!(self.bounding_box.get_x_start() <= x);
        assert!(x <= self.bounding_box.get_x_end());
        assert!(self.bounding_box.get_y_start() <= y);
        assert!(y <= self.bounding_box.get_y_end());

        if x < self.bounding_box.get_x_end() {
            let new_position = (x + 1, y);
            self.current = new_position;
            return Some(new_position);
        }

        if y < self.bounding_box.get_y_end() {
            let new_x = self.bounding_box.get_x_start();
            let new_position = (new_x, y + 1);
            self.current = new_position;
            return Some(new_position);
        }

        return None;
    }
}

#[derive(Debug, Clone)]
struct BoundingBox {
    top: Position,
    bottom: Position,
    left: Position,
    right: Position,
}

impl BoundingBox {
    fn new(start: Position) -> BoundingBox {
        BoundingBox {
            top: start,
            bottom: start,
            left: start,
            right: start,
        }
    }

    fn generate_grid(&self) -> GridPoints {
        let starting_position: Position = (self.get_x_start(), self.get_y_start());

        let grid = GridPoints {
            current: starting_position,
            bounding_box: self.clone(),
        };

        return grid;
    }

    fn is_strictly_inside_bounding_box(&self, target: Position) -> bool {
        let (x, y) = target;

        return self.get_x_start() < x
            && x < self.get_x_end()
            && self.get_y_start() < y
            && y < self.get_y_end();
    }

    // left-most x coord
    fn get_x_start(&self) -> i32 {
        let (x, _y) = self.left;
        return x;
    }

    // right-most x coord
    fn get_x_end(&self) -> i32 {
        let (x, _y) = self.right;
        return x;
    }

    // bottom-most y coord
    fn get_y_start(&self) -> i32 {
        let (_x, y) = self.bottom;
        return y;
    }

    // top-most y coord
    fn get_y_end(&self) -> i32 {
        let (_x, y) = self.top;
        return y;
    }

    fn add_point(&self, src: Position) -> BoundingBox {
        let mut cloned = self.clone();

        if is_better_top_edge(self.top, src) {
            cloned.top = src;
        }

        if is_better_bottom_edge(self.bottom, src) {
            cloned.bottom = src;
        }

        if is_better_left_edge(self.left, src) {
            cloned.left = src;
        }

        if is_better_right_edge(self.right, src) {
            cloned.right = src;
        }

        return cloned;
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    let destinations: Vec<Position> = input_string.lines().map(parse_to_coord).collect();

    // let ignored_destinations: HashSet<Position> = HashSet::new();

    // from the given destinations, generate the bounding box.

    let bounding_box = destinations
        .iter()
        .fold(None, |acc: Option<BoundingBox>, dest| match acc {
            None => return Some(BoundingBox::new(*dest)),
            Some(bounding_box) => {
                return Some(bounding_box.add_point(*dest));
            }
        });
    // println!("{:?}", bounding_box);

    if bounding_box.is_none() {
        println!("No bounding box generated.");
        return;
    }

    let bounding_box = bounding_box.unwrap();

    // filter out destinations on the edge of the bounding box (i.e. grid).
    // we want to do this because regions for these destinations have infinite area.

    let valid_destinations: Vec<Position> = destinations
        .iter()
        .map(|x| -> Position { *x })
        .filter(|destination: &Position| -> bool {
            return bounding_box.is_strictly_inside_bounding_box(*destination);
        })
        .collect();

    let mut regions = {
        let mut regions: Regions = HashMap::new();

        for destination in valid_destinations.clone() {
            let mut region = HashSet::new();
            // a destination is part of its own region
            region.insert(destination);

            regions.insert(destination, region);
        }

        regions
    };

    for position in bounding_box.generate_grid() {
        // find region that this position belongs to

        let result = valid_destinations.iter().fold(
            GridPositionState::FreeClaim,
            |acc: GridPositionState, destination| -> GridPositionState {
                match acc {
                    GridPositionState::FreeClaim => {
                        let distance = get_manhattan_distance(position, *destination);

                        let mut set = HashSet::new();
                        set.insert(*destination);

                        return GridPositionState::Region(set, distance);
                    }
                    GridPositionState::Region(mut set, best_distance) => {
                        assert!(set.len() > 0);

                        let some_destination = set.iter().next().unwrap();

                        let distance = get_manhattan_distance(position, *some_destination);

                        if distance > best_distance {
                            return GridPositionState::Region(set, best_distance);
                        }

                        if distance < best_distance {
                            let mut set = HashSet::new();
                            set.insert(*destination);

                            return GridPositionState::Region(set, distance);
                        }

                        // invariant: distance == best_distance
                        set.insert(*destination);
                        return GridPositionState::Region(set, distance);
                    }
                }
            },
        );

        match result {
            GridPositionState::FreeClaim => {
                unreachable!();
            }
            GridPositionState::Region(set, _best_distance) => {
                if set.len() == 1 {
                    let destination = set.iter().next().unwrap();
                    regions.entry(*destination).and_modify(|set| {
                        set.insert(position);
                    });
                }
            }
        }
    }

    let largest_region_size = regions
        .iter()
        .fold(None, |acc, (_destination, region_area)| {
            // println!("{:?}", region_area.len());

            match acc {
                None => return Some(region_area.len()),
                Some(best_region_area_size) => {
                    let region_area_size = region_area.len();

                    if region_area_size > best_region_area_size {
                        return Some(region_area_size);
                    }

                    return acc;
                }
            }
        });

    match largest_region_size {
        None => {
            println!("Part 1 -- no region found");
        }
        Some(largest_region_size) => {

            // not 13444
            // not 12570
            println!("Part 1 -- largest area size: {}", largest_region_size);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_manhattan_distance() {
        assert_eq!(get_manhattan_distance((0, 0), (0, 0)), 0);
        assert_eq!(get_manhattan_distance((0, 0), (3, 3)), 6);
    }
}
