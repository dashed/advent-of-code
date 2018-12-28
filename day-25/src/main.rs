// https://adventofcode.com/2018/day/25

// imports

use std::collections::HashSet;

// code

type Coordinate = (i32, i32, i32, i32);

struct Constellation {
    points: HashSet<Coordinate>,
}

impl Constellation {
    fn new(point: Coordinate) -> Self {
        let mut points = HashSet::new();
        points.insert(point);

        return Constellation { points };
    }
}

// adapted from day 6
// https://math.stackexchange.com/a/139604/10247
type Distance = i32;
fn get_manhattan_distance(start: Coordinate, end: Coordinate) -> Distance {
    let (a, b, e, g) = start;
    let (c, d, f, h) = end;

    return (a - c).abs() + (b - d).abs() + (e - f).abs() + (g - h).abs();
}

fn main() {
    let input_string = include_str!("input.txt");

    println!("{}", input_string);
}
