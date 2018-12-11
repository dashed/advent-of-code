// https://adventofcode.com/2018/day/6

// imports

use std::collections::HashSet;

// helpers

type Position = (i32, i32);

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

fn main() {
    let input_string = include_str!("input.txt");

    let destinations: Vec<Position> = input_string.lines().map(parse_to_coord).collect();

    let ignored_destinations: HashSet<Position> = HashSet::new();

    // filter out destinations on the edge of the bounding box (i.e. grid).
    // we want to do this because regions for these destinations have infinite area.


    println!("{:?}", destinations);
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
