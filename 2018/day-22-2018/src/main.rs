// https://adventofcode.com/2018/day/22

// imports

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

// code

type Cost = i32;

// adapted from day 6
// https://math.stackexchange.com/a/139604/10247
type Distance = i32;
fn get_manhattan_distance(start: Coordinate, end: Coordinate) -> Distance {
    let (a, b) = start;
    let (c, d) = end;

    (a - c).abs() + (b - d).abs()
}

#[derive(PartialEq, Hash, Eq, Clone, Debug)]
struct TimeCoordinate {
    time: Time,
    distance: Distance,
    position: Coordinate,
    tool: Tool,
}

impl TimeCoordinate {
    fn new(time: Time, distance: Distance, position: Coordinate, tool: Tool) -> Self {
        TimeCoordinate {
            time,
            distance,
            position,
            tool,
        }
    }

    fn get_cost(&self) -> Cost {
        self.distance + self.time
    }

    fn move_to_square(&self, distance: Distance, new_position: Coordinate) -> Self {
        assert!(get_manhattan_distance(new_position, self.position) == 1);
        assert!(distance >= 0);

        let mut next = self.clone();

        next.distance = distance;
        next.position = new_position;
        next.time += 1;

        next
    }

    fn switch_tool(&self, next_tool: Tool) -> Self {
        assert!(next_tool != self.tool);

        let mut next = self.clone();

        next.tool = next_tool;
        next.time += TIME_TO_SWITCH_TOOL;

        assert!(next.time > self.time);

        next
    }
}

impl Ord for TimeCoordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        // reversed for the binary heap which is a max-heap
        (self.get_cost()).cmp(&(other.get_cost())).reverse()
    }
}

impl PartialOrd for TimeCoordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // reversed for the binary heap which is a max-heap
        Some(self.cmp(other))
    }
}

// takes 7 minutes to switch tools
const TIME_TO_SWITCH_TOOL: Time = 7;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Tool {
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
        (*x, y - 1)
    }

    fn down(&self) -> Coordinate {
        let (x, y) = self;
        (*x, y + 1)
    }

    fn left(&self) -> Coordinate {
        let (x, y) = self;
        (x - 1, *y)
    }

    fn right(&self) -> Coordinate {
        let (x, y) = self;
        (x + 1, *y)
    }
}

#[derive(Debug, Clone, PartialEq)]
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

    fn to_str(&self) -> String {
        let result = match self {
            RegionType::Rocky => ".",
            RegionType::Wet => "=",
            RegionType::Narrow => "|",
        };
        result.to_string()
    }

    fn required_tools(&self) -> HashSet<Tool> {
        let mut set = HashSet::new();

        match self {
            RegionType::Rocky => {
                set.insert(Tool::ClimbingGear);
                set.insert(Tool::Torch);
                set
            }
            RegionType::Wet => {
                set.insert(Tool::ClimbingGear);
                set.insert(Tool::None);
                set
            }
            RegionType::Narrow => {
                set.insert(Tool::None);
                set.insert(Tool::Torch);
                set
            }
        }
    }
}

struct Cave {
    depth: Depth,
    target: Coordinate,
    geologic_indices: HashMap<Coordinate, GeologicIndex>,
    region_types: HashMap<Coordinate, RegionType>,
}

impl Cave {
    fn new(depth: Depth, target: Coordinate) -> Self {
        let mut geologic_indices = HashMap::new();
        let region_types = HashMap::new();

        // The region at 0,0 (the mouth of the cave) has a geologic index of 0.
        geologic_indices.insert(MOUTH_OF_CAVE, 0);

        // The region at the coordinates of the target has a geologic index of 0.
        geologic_indices.insert(target, 0);

        Cave {
            depth,
            target,
            geologic_indices,
            region_types,
        }
    }

    fn get_risk_level(&mut self, coord: &Coordinate) -> RiskLevel {
        self.get_region_type(coord).risk_level()
    }

    fn get_region_type(&mut self, coord: &Coordinate) -> RegionType {
        if let Some(region_type) = self.region_types.get(coord) {
            return region_type.clone();
        }

        let result = self.get_erosion_level(coord) % 3;

        let result = match result {
            0 => RegionType::Rocky,
            1 => RegionType::Wet,
            2 => RegionType::Narrow,
            _ => {
                unreachable!();
            }
        };

        self.region_types.insert(*coord, result.clone());

        result
    }

    fn get_adjacent_squares(&self, coord: &Coordinate) -> Vec<Coordinate> {
        let adjacent = vec![coord.left(), coord.right(), coord.up(), coord.down()];

        adjacent
            .into_iter()
            .filter(|coord| {
                let (x, y) = coord;
                x >= &0 && y >= &0
            })
            .collect()
    }

    fn get_erosion_level(&mut self, coord: &Coordinate) -> ErosionLevel {
        (self.get_geologic_index(coord) + self.depth) % 20183
    }

    fn find_target(&mut self) -> Option<Time> {
        let mut available_squares: BinaryHeap<TimeCoordinate> = BinaryHeap::new();
        // keep track of the best minimum time spent for a coordinate
        let mut best_costs: HashMap<(Tool, Coordinate), Time> = HashMap::new();

        // You start at 0,0 (the mouth of the cave) with the torch equipped

        available_squares.push(TimeCoordinate::new(
            0,
            get_manhattan_distance(MOUTH_OF_CAVE, self.target),
            MOUTH_OF_CAVE,
            Tool::Torch,
        ));

        while let Some(current_square) = available_squares.pop() {
            let current_position = current_square.position;
            let current_tool = current_square.tool.clone();

            let current_region = self.get_region_type(&current_position);

            if (current_region == RegionType::Rocky && current_tool == Tool::None)
                || (current_region == RegionType::Wet && current_tool == Tool::Torch)
                || (current_region == RegionType::Narrow && current_tool == Tool::ClimbingGear)
            {
                continue;
            }

            if best_costs
                .get(&(current_tool.clone(), current_position))
                .unwrap_or(&i32::max_value())
                <= &current_square.time
            {
                continue;
            }

            best_costs.insert(
                (current_tool.clone(), current_position),
                current_square.time,
            );

            if current_position == self.target && current_tool == Tool::Torch {
                return Some(current_square.time);
            }

            // add all possible movements
            for adjacent_square in self.get_adjacent_squares(&current_position) {
                let next = current_square.move_to_square(
                    get_manhattan_distance(adjacent_square, self.target),
                    adjacent_square,
                );

                available_squares.push(next);
            }

            // switch tools

            let required_tools = self
                .get_region_type(&current_square.position)
                .required_tools();

            for next_tool in required_tools {
                if next_tool != current_square.tool {
                    let next = current_square.switch_tool(next_tool);

                    available_squares.push(next);
                }
            }
        }

        None
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

        geologic_index
    }

    #[allow(dead_code)]
    fn to_str_mut(&mut self) -> String {
        let (target_x, target_y) = self.target;

        let mut map_string: Vec<String> = vec![];

        for y in 0..=target_y {
            let mut row_string = String::from("");

            for x in 0..=target_x {
                let coord = (x, y);

                if coord == MOUTH_OF_CAVE {
                    row_string.push('M');
                    continue;
                }

                if coord == self.target {
                    row_string.push('T');
                    continue;
                }

                let result = self.get_region_type(&coord).to_str();

                row_string.push_str(&result);
            }

            map_string.push(row_string);
        }

        map_string.join("\n")
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

    total_risk
}

fn part_2(depth: Depth, target: Coordinate) -> Option<Time> {
    let mut cave = Cave::new(depth, target);

    assert!(cave.get_region_type(&target) == RegionType::Rocky);

    cave.find_target()
}

fn main() {
    // input

    let depth = 4002;
    let target: Coordinate = (5, 746);

    // let depth = 11820;
    // let target: Coordinate = (7, 782);

    let part_1 = part_1(depth, target);
    println!("Part 1: {}", part_1);

    let part_2 = part_2(depth, target);
    println!("Part 2: {:?}", part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(510, (10, 10)), 114);
        assert_eq!(part_1(4002, (5, 746)), 4479);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(510, (10, 10)), Some(45));
        // assert_eq!(part_2(11820, (7, 782)), Some(1075));
    }

    #[test]
    fn test_time_cost_min_heap() {
        let mut available_squares: BinaryHeap<TimeCoordinate> = BinaryHeap::new();

        let items = vec![
            TimeCoordinate::new(5, 1, (1, 26), Tool::Torch),
            TimeCoordinate::new(1, 2, (1, 26), Tool::Torch),
            TimeCoordinate::new(4, 3, (1, 26), Tool::Torch),
            TimeCoordinate::new(1, 4, (1, 26), Tool::Torch),
        ];
        available_squares.extend(items);

        let mut actual: Vec<TimeCoordinate> = vec![];
        while let Some(item) = available_squares.pop() {
            actual.push(item);
        }

        let expected = vec![
            TimeCoordinate::new(1, 2, (1, 26), Tool::Torch),
            TimeCoordinate::new(1, 4, (1, 26), Tool::Torch),
            TimeCoordinate::new(5, 1, (1, 26), Tool::Torch),
            TimeCoordinate::new(4, 3, (1, 26), Tool::Torch),
        ];

        assert_eq!(actual, expected);
    }
}
