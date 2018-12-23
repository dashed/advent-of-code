// https://adventofcode.com/2018/day/22

// imports

use std::collections::HashMap;

// code

type Coordinate = (i32, i32);
type GeologicIndex = i32;
type RiskLevel = i32;

const MOUTH_OF_CAVE: Coordinate = (0, 0);

enum RegionType {
    Rocky,
    Narrow,
    Wet,
}

impl RegionType {
    fn risk_level(&self) -> RiskLevel {
        match self {
            RegionType::Rocky => 0,
            RegionType::Wet => 1,
            RegionType::Narrow => 2,
        }
    }
}

struct Cave {
    geologic_indices: HashMap<Coordinate, GeologicIndex>,
}

impl Cave {
    fn new(target: Coordinate) -> Self {
        let mut geologic_indices = HashMap::new();

        // The region at 0,0 (the mouth of the cave) has a geologic index of 0.
        geologic_indices.insert(MOUTH_OF_CAVE, 0);

        // The region at the coordinates of the target has a geologic index of 0.
        geologic_indices.insert(target, 0);

        Cave { geologic_indices }
    }
}

fn main() {
    // input

    let depth = 4002;
    let target: Coordinate = (5, 746);

    let cave = Cave::new(target);
}
