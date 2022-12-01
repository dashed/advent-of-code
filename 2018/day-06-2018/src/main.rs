// https://adventofcode.com/2018/day/6

// imports

use std::collections::HashMap;

// helpers

type Position = (i32, i32);
type Destination = Position;
type Distance = i32;

type Area = i32;
// mapping a Region to Area
type Regions = HashMap<Destination, Area>;

// https://math.stackexchange.com/a/139604/10247
fn get_manhattan_distance(x: Position, y: Position) -> i32 {
    let (a, b) = x;
    let (c, d) = y;

    (a - c).abs() + (b - d).abs()
}

fn parse_to_coord(input: &str) -> Position {
    let result: Vec<i32> = input
        .split(',')
        .map(|x| -> i32 { x.trim().parse().unwrap() })
        .collect();

    (*result.first().unwrap(), *result.get(1).unwrap())
}

fn get_x(src: Position) -> i32 {
    let (x, _y) = src;
    x
}

fn get_y(src: Position) -> i32 {
    let (_x, y) = src;
    y
}

fn is_better_top_edge(reference: Position, target: Position) -> bool {
    let ref_point = get_y(reference);
    let target_point = get_y(target);
    target_point > ref_point
}

fn is_better_bottom_edge(reference: Position, target: Position) -> bool {
    let ref_point = get_y(reference);
    let target_point = get_y(target);
    target_point < ref_point
}

fn is_better_left_edge(reference: Position, target: Position) -> bool {
    let ref_point = get_x(reference);
    let target_point = get_x(target);
    target_point < ref_point
}

fn is_better_right_edge(reference: Position, target: Position) -> bool {
    let ref_point = get_x(reference);
    let target_point = get_x(target);
    target_point > ref_point
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

    fn is_strictly_inside_bounding_box(&self, target: Position) -> bool {
        let (x, y) = target;

        self.get_x_start() < x
            && x < self.get_x_end()
            && self.get_y_start() < y
            && y < self.get_y_end()
    }

    // left-most x coord
    fn get_x_start(&self) -> i32 {
        let (x, _y) = self.left;
        x
    }

    // right-most x coord
    fn get_x_end(&self) -> i32 {
        let (x, _y) = self.right;
        x
    }

    // bottom-most y coord
    fn get_y_start(&self) -> i32 {
        let (_x, y) = self.bottom;
        y
    }

    // top-most y coord
    fn get_y_end(&self) -> i32 {
        let (_x, y) = self.top;
        y
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

        cloned
    }
}

fn part_1(input_string: &str) -> Option<i32> {
    let destinations: Vec<Position> = input_string.trim().lines().map(parse_to_coord).collect();

    // from the given destinations, generate the bounding box.

    let bounding_box = destinations
        .iter()
        .fold(None, |acc: Option<BoundingBox>, dest| match acc {
            None => Some(BoundingBox::new(*dest)),
            Some(bounding_box) => {
                Some(bounding_box.add_point(*dest))
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
            regions.insert(destination, 0);
        }

        regions
    };

    for x in bounding_box.get_x_start()..=bounding_box.get_x_end() {
        for y in bounding_box.get_y_start()..=bounding_box.get_y_end() {
            // find region that this position belongs to
            let position = (x, y);

            let mut distances: Vec<(Destination, Distance)> = destinations
                .iter()
                .map(|dest| {
                    let distance_to_position = get_manhattan_distance(position, *dest);
                    (*dest, distance_to_position)
                })
                .collect();

            // sort by distance from largest to smallest
            distances.sort_by_key(|&(_dest, distance)| {
                distance
            });

            let (dest, smallest_distance) = distances.first().unwrap();
            let (_dest2, second_smallest_distance) = distances.get(1).unwrap();

            if smallest_distance < second_smallest_distance {
                // invariant: position belongs to the region defined by dest

                // if a position is on the edge of the bounding box,
                // then the region defined by dest has infinite area.
                if !bounding_box.is_strictly_inside_bounding_box(position) {
                    regions.remove(dest);
                    continue;
                }

                regions.entry(*dest).and_modify(|e| *e += 1);
            }
        }
    }

    let largest_region_size =
        regions
            .iter()
            .fold(None, |acc: Option<i32>, (destination, region_area)| {
                if !bounding_box.is_strictly_inside_bounding_box(*destination) {
                    return acc;
                }

                match acc {
                    None => Some(*region_area),
                    Some(largest_region_area_size) => {
                        if region_area > &largest_region_area_size {
                            return Some(*region_area);
                        }

                        acc
                    }
                }
            });

    largest_region_size
}

fn part_2(input_string: &str) -> Option<i32> {
    let destinations: Vec<Position> = input_string.trim().lines().map(parse_to_coord).collect();

    // from the given destinations, generate the bounding box.

    let bounding_box = destinations
        .iter()
        .fold(None, |acc: Option<BoundingBox>, dest| match acc {
            None => Some(BoundingBox::new(*dest)),
            Some(bounding_box) => {
                Some(bounding_box.add_point(*dest))
            }
        });

    if bounding_box.is_none() {
        println!("No bounding box generated.");
        return None;
    }

    let bounding_box = bounding_box.unwrap();

    let distance = 10000;
    let gap = distance / destinations.len() as i32 + 1;

    let mut size_of_region = 0;
    for x in (bounding_box.get_x_start() - gap)..=(bounding_box.get_x_end() + gap) {
        for y in (bounding_box.get_y_start() - gap)..=(bounding_box.get_y_end() + gap) {
            let position = (x, y);

            let mut total = 0;

            for destination in &destinations {
                let distance_to_position = get_manhattan_distance(position, *destination);

                total += distance_to_position;

                if total >= distance {
                    continue;
                }
            }

            if total >= distance {
                continue;
            }

            size_of_region += 1;
        }
    }

    Some(size_of_region)
}

fn main() {
    let input_string = include_str!("input.txt");

    let largest_region_size = part_1(input_string);

    match largest_region_size {
        None => {
            println!("Part 1 -- no region found");
        }
        Some(largest_region_size) => {
            println!("Part 1 -- largest area size: {}", largest_region_size);
        }
    }

    let largest_region_size = part_2(input_string);

    match largest_region_size {
        None => {
            println!("Part 2 -- no region found");
        }
        Some(largest_region_size) => {
            println!("Part 2 -- largest area size: {}", largest_region_size);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_to_coord() {
        assert_eq!(parse_to_coord("1, 6"), (1, 6));
    }

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
