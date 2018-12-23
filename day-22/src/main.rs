// https://adventofcode.com/2018/day/22

// imports

use std::collections::HashMap;

// code

enum CurrentTool {
    None, // neither
    Torch,
    ClimbingGear,
}

type Coordinate = (i32, i32);
type GeologicIndex = i32;
type RiskLevel = i32;
type ErosionLevel = i32;
type Depth = i32;
type Time = i32;

const MOUTH_OF_CAVE: Coordinate = (0, 0);

trait Transitions {
    fn up(&self) -> Coordinate;
    fn down(&self) -> Coordinate;
    fn left(&self) -> Coordinate;
    fn right(&self) -> Coordinate;
}

impl Transitions for Coordinate {
    fn up(&self) -> Coordinate {
        let (x, y) = self;
        return (*x, y - 1);
    }

    fn down(&self) -> Coordinate {
        let (x, y) = self;
        return (*x, y + 1);
    }

    fn left(&self) -> Coordinate {
        let (x, y) = self;
        return (x - 1, *y);
    }

    fn right(&self) -> Coordinate {
        let (x, y) = self;
        return (x + 1, *y);
    }
}

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

    fn to_string(&self) -> String {
        let result = match self {
            RegionType::Rocky => ".",
            RegionType::Wet => "=",
            RegionType::Narrow => "|",
        };
        return result.to_string();
    }
}

struct Cave {
    depth: Depth,
    target: Coordinate,
    geologic_indices: HashMap<Coordinate, GeologicIndex>,
    current_tool: CurrentTool,

    // shortest amount of time to reach the region defined by Coordinate
    shortest_time: HashMap<Coordinate, Time>,
}

impl Cave {
    fn new(depth: Depth, target: Coordinate) -> Self {
        let mut geologic_indices = HashMap::new();
        let mut shortest_time = HashMap::new();

        // The region at 0,0 (the mouth of the cave) has a geologic index of 0.
        geologic_indices.insert(MOUTH_OF_CAVE, 0);
        shortest_time.insert(MOUTH_OF_CAVE, 0);

        // The region at the coordinates of the target has a geologic index of 0.
        geologic_indices.insert(target, 0);

        // You start at 0,0 (the mouth of the cave) with the torch equipped
        let current_tool = CurrentTool::Torch;

        Cave {
            depth,
            target,
            geologic_indices,
            current_tool,
            shortest_time,
        }
    }

    fn get_risk_level(&mut self, coord: &Coordinate) -> RiskLevel {
        return self.get_region_type(coord).risk_level();
    }

    fn get_region_type(&mut self, coord: &Coordinate) -> RegionType {
        let result = self.get_erosion_level(coord) % 3;

        match result {
            0 => RegionType::Rocky,
            1 => RegionType::Wet,
            2 => RegionType::Narrow,
            _ => {
                unreachable!();
            }
        }
    }

    fn get_erosion_level(&mut self, coord: &Coordinate) -> ErosionLevel {
        return (self.get_geologic_index(coord) + self.depth) % 20183;
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

        if *coord == MOUTH_OF_CAVE {
            return 0;
        }

        if *coord == self.target {
            return 0;
        }

        let (x, y) = coord;
        let geologic_index = if *y == 0 {
            // If the region's Y coordinate is 0,
            // the geologic index is its X coordinate times 16807.
            x * 16807
        } else if *x == 0 {
            // If the region's X coordinate is 0,
            // the geologic index is its Y coordinate times 48271.
            y * 48271
        } else {
            // Otherwise, the region's geologic index is
            // the result of multiplying the erosion levels of the regions at X-1,Y and X,Y-1.
            self.get_erosion_level(&coord.left()) * self.get_erosion_level(&coord.up())
        };

        self.geologic_indices.insert(*coord, geologic_index);

        return geologic_index;
    }

    #[allow(dead_code)]
    fn to_string(&mut self) -> String {
        let (target_x, target_y) = self.target;

        let mut map_string: Vec<String> = vec![];

        for y in 0..=target_y {
            let mut row_string = String::from("");

            for x in 0..=target_x {
                let coord = (x, y);

                if coord == MOUTH_OF_CAVE {
                    row_string.push_str("M");
                    continue;
                }

                if coord == self.target {
                    row_string.push_str("T");
                    continue;
                }

                let result = self.get_region_type(&coord).to_string();

                row_string.push_str(&result);
            }

            map_string.push(row_string);
        }

        return map_string.join("\n");
    }
}

fn part_1(depth: Depth, target: Coordinate) -> RiskLevel {
    let (target_x, target_y) = target;

    let mut cave = Cave::new(depth, target);

    let mut total_risk: RiskLevel = 0;

    for x in 0..=target_x {
        for y in 0..=target_y {
            let coord = (x, y);

            total_risk += cave.get_risk_level(&coord);
        }
    }

    // println!("{}", cave.to_string());

    return total_risk;
}

fn main() {
    // input

    let depth = 4002;
    let target: Coordinate = (5, 746);

    let part_1 = part_1(depth, target);
    println!("Part 1: {}", part_1);

    // let cave = Cave::new(depth, target);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let part_1 = part_1(510, (10, 10));

        assert_eq!(part_1, 114);
    }

}
