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

// struct GridPoints {
//     current: Position,
//     bounding_box: BoundingBox,
// }

// impl Iterator for GridPoints {
//     type Item = Position;

//     fn next(&mut self) -> Option<Position> {
//         let (x, y) = self.current;

//         // invariants
//         assert!(self.bounding_box.get_x_start() <= x);
//         assert!(x <= self.bounding_box.get_x_end());
//         assert!(self.bounding_box.get_y_start() <= y);
//         assert!(y <= self.bounding_box.get_y_end());

//         if x < self.bounding_box.get_x_end() {
//             let new_position = (x + 1, y);
//             self.current = new_position;
//             return Some(new_position);
//         }

//         if y < self.bounding_box.get_y_end() {
//             let new_x = self.bounding_box.get_x_start();
//             let new_position = (new_x, y + 1);
//             self.current = new_position;
//             return Some(new_position);
//         }

//         return None;
//     }
// }

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

fn part_1(input_string: &str) -> Option<i32> {

    let destinations: Vec<Position> = input_string.trim().lines().map(parse_to_coord).collect();

    // from the given destinations, generate the bounding box.

    let bounding_box = destinations
        .iter()
        .fold(None, |acc: Option<BoundingBox>, dest| match acc {
            None => return Some(BoundingBox::new(*dest)),
            Some(bounding_box) => {
                return Some(bounding_box.add_point(*dest));
            }
        });

    if bounding_box.is_none() {
        println!("No bounding box generated.");
        return None;
    }

    let bounding_box = bounding_box.unwrap();

    let mut regions = {
        let mut regions: Regions = HashMap::new();

        for destination in destinations.clone() {
            let mut region = HashSet::new();
            // a destination is part of its own region
            region.insert(destination);

            regions.insert(destination, region);
        }

        regions
    };

    for x in bounding_box.get_x_start()..(bounding_box.get_x_end() + 1) {
        for y in bounding_box.get_y_start()..(bounding_box.get_y_end() + 1) {
            // find region that this position belongs to
            let position = (x, y);

            let result = destinations.iter().fold(
                GridPositionState::FreeClaim,
                |acc: GridPositionState, destination| -> GridPositionState {

                    if *destination == position {
                                let mut new_set = HashSet::new();
                                new_set.insert(*destination);

                                return GridPositionState::Region(new_set, 0);
                    }

                    match acc {
                        GridPositionState::FreeClaim => {
                            let distance = get_manhattan_distance(position, *destination);

                            let mut set = HashSet::new();
                            set.insert(*destination);

                            return GridPositionState::Region(set, distance);
                        }
                        GridPositionState::Region(mut set, best_distance) => {
                            assert!(set.len() > 0);
                            assert!(!set.contains(destination));


                            let distance = get_manhattan_distance(position, *destination);

                            if distance > best_distance || best_distance == 0 {
                                return GridPositionState::Region(set, best_distance);
                            }

                            if distance < best_distance {
                                let mut new_set = HashSet::new();
                                new_set.insert(*destination);

                                return GridPositionState::Region(new_set, distance);
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
                        regions.entry(*destination).and_modify(|x| {
                            assert!(x.len() > 0);
                            assert!(x.contains(destination));
                            if position != *destination {
                                assert!(!x.contains(&position));
                            } else {
                                assert!(position == *destination);
                            }

                            x.insert(position);
                        });
                    }
                }
            }
        }
    }

    let largest_region_size = regions
        .iter()
        .fold(None, |acc, (destination, region_area)| {
            if !bounding_box.is_strictly_inside_bounding_box(*destination) {
                return acc;
            }

            match acc {
                None => return Some(region_area.len() as i32),
                Some(best_region_area_size) => {
                    let region_area_size = region_area.len() as i32;

                    if region_area_size > best_region_area_size {
                        return Some(region_area_size as i32);
                    }

                    return acc;
                }
            }
        });

    return largest_region_size;
}

fn main() {
    let input_string = include_str!("input.txt");

    let largest_region_size = part_1(input_string);

    match largest_region_size {
        None => {
            println!("Part 1 -- no region found");
        }
        Some(largest_region_size) => {
            // not 13444
            // not 12570
            // not 5826
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

    #[test]
    fn test_part_1() {
        let input = r###"
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
        "###;

        assert_eq!(part_1(input), Some(17));
    }

}
