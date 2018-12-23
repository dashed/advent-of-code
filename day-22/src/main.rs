// https://adventofcode.com/2018/day/22

// imports

use std::collections::HashMap;

// code

type Coordinate = (i32, i32);
type GeologicIndex = i32;
type RiskLevel = i32;
type ErosionLevel = i32;

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

    fn get_erosion_level(&self, coord: &Coordinate) -> ErosionLevel {
        return 0;
    }

    fn get_geologic_index(&mut self, coord: &Coordinate) -> GeologicIndex {
        match self.geologic_indices.get(coord) {
            Some(index) => {
                return *index;
            }
            None => {
                // generate one
            }
        }

        let (x, y) = coord;
        let mut geologic_index = 0;

        // If the region's Y coordinate is 0,
        // the geologic index is its X coordinate times 16807.
        if *y == 0 {
            geologic_index = x * 16807;
        }

        // If the region's X coordinate is 0,
        // the geologic index is its Y coordinate times 48271.
        if *x == 0 {
            geologic_index = y * 16807;
        }

        // Otherwise, the region's geologic index is
        // the result of multiplying the erosion levels of the regions at X-1,Y and X,Y-1.

        geologic_index =
            self.get_erosion_level(&(x - 1, *y)) * self.get_erosion_level(&(*x, y - 1));

        self.geologic_indices.insert(*coord, geologic_index);

        return geologic_index;
    }
}

fn main() {
    // input

    let depth = 4002;
    let target: Coordinate = (5, 746);

    let cave = Cave::new(target);
}
