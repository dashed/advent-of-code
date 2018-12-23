// https://adventofcode.com/2018/day/22

// imports

use std::collections::HashMap;

// code

type Coordinate = (i32, i32);
type GeologicIndex = i32;

const MOUTH_OF_CAVE: Coordinate = (0, 0);

enum RegionType {
    Rocky,
    Narrow,
    Wet,
}

struct Cave {
    geologic_indices: HashMap<Coordinate, GeologicIndex>,
}

fn main() {
    // input

    let depth = 4002;
    let target: Coordinate = (5, 746);
}
