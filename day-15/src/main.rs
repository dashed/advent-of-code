// https://adventofcode.com/2018/day/15

// imports

use core::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::BinaryHeap;
use std::collections::HashMap;

// code

enum RoundState {
    Incomplete,
    Complete
}

trait Transitions {
    fn up(&self) -> Coordinate;
    fn down(&self) -> Coordinate;
    fn left(&self) -> Coordinate;
    fn right(&self) -> Coordinate;
}

type Coordinate = (i32, i32);

type Path = Vec<Coordinate>;

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

#[derive(PartialEq, Hash, Eq, Clone, Debug)]
struct DistanceCoordinate(Distance, Coordinate);

impl PartialOrd for DistanceCoordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // reversed for the binary heap which is a max-heap
        if self.0 != other.0 {
            return Some(other.0.cmp(&self.0));
        }
        return Some(reading_order(&other.1, &self.1));
    }
}

impl Ord for DistanceCoordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.partial_cmp(other).unwrap();
        return ord;
        // match ord {
        //     Ordering::Greater => Ordering::Less,
        //     Ordering::Less => Ordering::Greater,
        //     Ordering::Equal => ord,
        // }
    }
}

impl Into<Coordinate> for DistanceCoordinate {
    fn into(self) -> Coordinate {
        return self.1;
    }
}

// adapted from day 6
// https://math.stackexchange.com/a/139604/10247
type Distance = i32;
fn get_manhattan_distance(start: Coordinate, end: Coordinate) -> Distance {
    let (a, b) = start;
    let (c, d) = end;

    return (a - c).abs() + (b - d).abs();
}

// sort coordinates according to their reading order
fn reading_order(first_coord: &Coordinate, second_coord: &Coordinate) -> Ordering {
    let (x1, y1) = first_coord;
    let (x2, y2) = second_coord;

    if y1 != y2 {
        return y1.cmp(y2);
    }

    return x1.cmp(x2);
}

enum MapState {
    Wall,
    Cavern,
}

type Terrain = HashMap<Coordinate, MapState>;
type UnitPlacement = BTreeMap<Coordinate, Unit>;

struct Map {
    terrain: Terrain,
    units: UnitPlacement,
}

impl Map {
    fn new() -> Map {
        Map {
            terrain: HashMap::new(),
            units: BTreeMap::new(),
        }
    }

    fn to_string(&self) -> String {
        let max_x = self
            .terrain
            .iter()
            .map(|((x, _y), _map_state)| x)
            .max()
            .unwrap();
        let max_y = self
            .terrain
            .iter()
            .map(|((_x, y), _map_state)| y)
            .max()
            .unwrap();

        let mut map_string: Vec<String> = vec![];

        for y in 0..=*max_y {
            let mut row_string = String::from("");

            for x in 0..=*max_x {
                let position = (x, y);

                match self.terrain.get(&position) {
                    None => {
                        row_string.push_str("#");
                    }
                    Some(map_state) => {
                        match map_state {
                            MapState::Wall => {
                                // invariant: a unit cannot be within a wall
                                assert!(!self.units.contains_key(&position));
                                row_string.push_str("#");
                            }
                            MapState::Cavern => match self.units.get(&position) {
                                None => {
                                    row_string.push_str(".");
                                }
                                Some(unit) => {
                                    row_string.push_str(&unit.to_string());
                                }
                            },
                        }
                    }
                }
            }

            map_string.push(row_string);
        }

        return map_string.join("\n");
    }

    fn insert(&mut self, position: Coordinate, cell: char) {
        match cell {
            '#' => {
                self.terrain.insert(position, MapState::Wall);
            }
            '.' => {
                self.terrain.insert(position, MapState::Cavern);
            }
            'G' => {
                self.terrain.insert(position, MapState::Cavern);
                self.units.insert(position, Unit::new_goblin());
            }
            'E' => {
                self.terrain.insert(position, MapState::Cavern);
                self.units.insert(position, Unit::new_elf());
            }
            _ => {
                assert!(false, "Unknown cell: {}", cell);
            }
        }
    }

    fn is_wall(&self, position: Coordinate) -> bool {
        match self.terrain.get(&position) {
            None => true,
            Some(map_state) => match map_state {
                MapState::Wall => true,
                MapState::Cavern => false,
            },
        }
    }

    fn is_occupied(&self, position: Coordinate) -> bool {
        if self.is_wall(position) {
            return true;
        }

        // check if the position is occupied by a unit
        return self.units.contains_key(&position);
    }

    fn get_elves(&self) -> Vec<(&Coordinate, &Unit)> {
        return self
            .units
            .iter()
            .filter(|(_position, unit)| {
                return unit.is_elf();
            })
            .collect();
    }

    fn has_elves(&self) -> bool {
        return self
            .units
            .iter()
            .filter(|(_position, unit)| {
                return unit.is_elf();
            })
            .next()
            .is_some();
    }

    fn get_goblins(&self) -> Vec<(&Coordinate, &Unit)> {
        return self
            .units
            .iter()
            .filter(|(_position, unit)| {
                return unit.is_goblin();
            })
            .collect();
    }

    fn has_goblins(&self) -> bool {
        return self
            .units
            .iter()
            .filter(|(_position, unit)| {
                return unit.is_goblin();
            })
            .next()
            .is_some();
    }

    // checks if a round can be executed
    fn can_run_round(&self) -> bool {
        if self.units.is_empty() {
            return false;
        }

        let (_position, unit) = self.units.iter().next().unwrap();

        if unit.is_elf() {
            return self.has_goblins();
        }

        if unit.is_goblin() {
            return self.has_elves();
        }

        unreachable!();
    }
    // get open squares adjacent to position
    fn get_adjacent_open_squares(&self, position: Coordinate) -> Vec<Coordinate> {
        let coords = vec![
            position.up(),
            position.down(),
            position.left(),
            position.right(),
        ];

        return coords
            .into_iter()
            .filter(|coord| {
                return !self.is_occupied(*coord);
            })
            .collect();
    }

    // returns true if combat has ended (i.e. round didn't run)
    fn execute_round(&mut self) -> RoundState {
        if !self.can_run_round() {
            return RoundState::Incomplete;
        }

        // track number of actions performed this round by each unit.
        // an action is either a movement or an attack
        let mut num_of_actions_performed = 0;

        let position_of_units: Vec<Coordinate> = {
            let mut units = vec![];
            for (position_of_unit, _unit) in self.units.clone().into_iter() {
                units.push(position_of_unit);
            }
            units.sort_by(reading_order);
            units
        };

        for position_of_unit in position_of_units {
            // check if this unit is still alive.
            if !self.units.contains_key(&position_of_unit) {
                continue;
            }

            let unit: Unit = self.units.get(&position_of_unit).unwrap().clone();

            assert!(unit.is_alive());

            // Each unit begins its turn by identifying all possible targets (enemy units).
            let targets = if unit.is_elf() {
                self.get_goblins()
            } else if unit.is_goblin() {
                self.get_elves()
            } else {
                unreachable!();
            };

            let targets = targets.clone();

            // If no targets remain, combat ends.
            if targets.len() <= 0 {
                return RoundState::Incomplete;
            }

            let adjacent_targets: Vec<(Coordinate, Unit)> = {
                let mut adjacent_targets: Vec<(Coordinate, Unit)> = targets
                    .iter()
                    .map(|(position_of_target, target)| -> (Coordinate, Unit) {
                        (*position_of_target.clone(), (*target).clone())
                    })
                    .filter(|(position_of_target, _target)| {
                        return get_manhattan_distance(position_of_unit, *position_of_target) <= 1;
                    })
                    .collect();

                adjacent_targets.sort_by(|item_1, item_2| {
                    let (position_of_target_1, _target_1) = item_1;
                    let (position_of_target_2, _target_2) = item_2;
                    return reading_order(position_of_target_1, position_of_target_2);
                });

                adjacent_targets
            };

            if adjacent_targets.len() >= 1 {
                // If the unit is already in range of a target,
                // it does not move, but continues its turn with an attack.

                num_of_actions_performed += 1;

                let (position_of_target, chosen_target) = adjacent_targets.first().clone().unwrap();

                let mut chosen_target: Unit = (*chosen_target).clone();

                unit.attack(&mut chosen_target);

                if chosen_target.is_dead() {
                    self.units.remove(&position_of_target);
                } else {
                    self.units.insert(*position_of_target, chosen_target);
                }

                continue;
            }

            // Otherwise, since it is not in range of a target, it moves.

            let mut reachable_paths: Vec<Path> = targets
                .into_iter()
                .map(|(position_of_target, _target)| {
                    // for each target, identify open squares adjacent to position_of_target
                    let adjacent_open_squares = self.get_adjacent_open_squares(*position_of_target);

                    let reachable_paths: Vec<Path> = adjacent_open_squares
                        .into_iter()
                        .map(|adjacent_coord| {
                            return get_reachable_path(self, position_of_unit, adjacent_coord);
                        })
                        .filter(|reachable| {
                            // filter out un-reachable adjacent coords
                            return reachable.is_some();
                        })
                        .map(|reachable| {
                            return reachable.unwrap();
                        })
                        .filter(|reachable| {
                            // only consider non-empty paths
                            return reachable.len() >= 1;
                        })
                        .collect();
                    return reachable_paths;
                })
                .fold(
                    vec![],
                    |mut acc: Vec<Path>, reachable_paths: Vec<Path>| -> Vec<Path> {
                        acc.extend(reachable_paths);
                        return acc;
                    },
                );

            reachable_paths.sort_by(|path_1, path_2| {
                let len_1 = path_1.len();
                let len_2 = path_2.len();

                if len_1 != len_2 {
                    return len_1.cmp(&len_2);
                }

                let coord_1 = path_1.first().unwrap();
                let coord_2 = path_2.first().unwrap();

                return reading_order(coord_1, coord_2);
            });

            if reachable_paths.len() >= 1 {
                let path: &Path = reachable_paths.first().unwrap();
                let next_move: Coordinate = *path.first().unwrap();

                // println!(
                //     "{} at {:?} next_move: {:?}",
                //     unit.to_string(),
                //     position_of_unit,
                //     next_move
                // );

                self.units.remove(&position_of_unit);
                self.units.insert(next_move, unit);

                num_of_actions_performed += 1;
            }
        }

        // combat has ended if no action was performed
        if num_of_actions_performed <= 0 {
            return RoundState::Incomplete;
        }
        return RoundState::Complete;
    }
}

#[derive(Debug, Clone)]
enum UnitType {
    Goblin,
    Elf,
}

#[derive(Debug, Clone)]
struct Unit {
    unit_type: UnitType,
    hit_points: i32,
    attack_power: i32,
}

impl Unit {
    fn new(unit_type: UnitType) -> Unit {
        Unit {
            unit_type,
            hit_points: 200,
            attack_power: 3,
        }
    }

    fn attack(&self, other_unit: &mut Unit) {
        other_unit.hit_points = other_unit.hit_points - self.attack_power;
    }

    fn new_elf() -> Unit {
        Unit::new(UnitType::Elf)
    }

    fn new_goblin() -> Unit {
        Unit::new(UnitType::Goblin)
    }

    fn to_string(&self) -> String {
        match self.unit_type {
            UnitType::Goblin => "G".to_string(),
            UnitType::Elf => "E".to_string(),
        }
    }

    fn is_alive(&self) -> bool {
        return self.hit_points > 0;
    }

    fn is_dead(&self) -> bool {
        return !self.is_alive();
    }

    fn is_elf(&self) -> bool {
        match self.unit_type {
            UnitType::Elf => true,
            _ => false,
        }
    }

    fn is_goblin(&self) -> bool {
        match self.unit_type {
            UnitType::Goblin => true,
            _ => false,
        }
    }
}

// checks if there is an open path between start and end
// an open path means a set of coordinates which are not either a wall or occupied by a unit
// if a path exists, then the vector containing the coordinates from start to end is returned
fn get_reachable_path(map: &Map, start: Coordinate, end: Coordinate) -> Option<Vec<Coordinate>> {
    if start == end {
        return Some(vec![]);
    }

    // invariant: manhattan distance between start and end is at least 1

    if map.is_wall(start) || map.is_wall(end) {
        return None;
    }

    if map.is_occupied(end) {
        return None;
    }

    // NOTE: start could be occupied

    let mut available_squares: BinaryHeap<DistanceCoordinate> = BinaryHeap::new();
    // keep track of the best minimum distances for a coordinate
    let mut distances: HashMap<Coordinate, Distance> = HashMap::new();
    let mut best_edges: HashMap<Coordinate, Coordinate> = HashMap::new();

    // backtrack from end towards start
    available_squares.push(DistanceCoordinate(0, end));
    distances.insert(end, 0);

    while let Some(current_square) = available_squares.pop() {
        // invariant: current_square has the lowest cost
        // in case of a tie, positions are sorted according to the reading order

        let DistanceCoordinate(current_distance, current_position) = current_square;

        if get_manhattan_distance(start, current_position) <= 1 {
            let mut path = vec![current_position];
            let mut current = current_position;
            while current != end {
                let nearest_edge = best_edges.get(&current).unwrap();
                path.push(*nearest_edge);
                current = *nearest_edge;
            }

            return Some(path);
        }

        match distances.get(&current_position) {
            None => {
                unreachable!();
            }
            Some(best_distance) => {
                if current_distance > *best_distance {
                    continue;
                }
            }
        }

        for adjacent_square in map.get_adjacent_open_squares(current_position) {
            let adjacent_distance = current_distance + 1;

            match distances.get(&adjacent_square) {
                None => {
                    best_edges.insert(adjacent_square, current_position);
                    distances.insert(adjacent_square, adjacent_distance);
                    available_squares.push(DistanceCoordinate(adjacent_distance, adjacent_square));
                }
                Some(best_distance) => {
                    // NOTE: this potentially adds duplicates to the available_squares min-heap;
                    // but that's fine :P
                    // see: https://www3.cs.stonybrook.edu/~rezaul/papers/TR-07-54.pdf

                    if adjacent_distance < *best_distance {
                        distances.insert(adjacent_square, adjacent_distance);
                        available_squares
                            .push(DistanceCoordinate(adjacent_distance, adjacent_square));
                        best_edges.insert(adjacent_square, current_position);
                    }
                }
            }
        }
    }

    return None;
}

// combat begins in a series of rounds
// in each round, a unit takes a turn, resolving all of its actions before completing their turn
// invariant: units cannot attack nor move diagonally

// invariant: Units cannot move into walls or other units.

// Rules for units
// On each unit's turn, it tries to move into range of an enemy (if it isn't already) and then attack (if it is in range).
// If the unit is already in range of a target, it does not move, but continues its turn with an attack.
// Otherwise, since it is not in range of a target, it moves.

fn parse_input(input_string: &str) -> Map {
    let mut map = Map::new();

    for (y, line) in input_string.trim().lines().enumerate() {
        for (x, map_state_as_char) in line.chars().enumerate() {
            let position: Coordinate = (x as i32, y as i32);

            map.insert(position, map_state_as_char);
        }
    }

    return map;
}

fn main() {
    // ensures reading order is satisfied
    assert!((0, 0) < (1, 0));
    assert!((0, 0) < (0, 1));
    assert!((0, 0) < (1, 1));
    assert!((1, 0) < (1, 1));
    assert!((0, 0) < (1, 1));

    let input_string = include_str!("input.txt");

    let mut map = parse_input(input_string);


    println!("{}", map.to_string());

    let mut idx = 1;

    let mut num_of_rounds_completed = 0;
    loop {

        let round_state = map.execute_round();
        // println!("{}", map.to_string());
        match round_state {
            RoundState::Complete => {
                num_of_rounds_completed += 1;
            }
            RoundState::Incomplete => {
                break;
            }
        }
    }

    let sum_hit_points: i32 = map.units.iter().map(|(key, unit)| unit) .fold(0, |acc, unit| {
        return unit.hit_points;
    });

    println!("num_of_rounds_completed: {}", num_of_rounds_completed);
    println!("sum_hit_points: {}", sum_hit_points);
    println!("Part 1: {}", num_of_rounds_completed * sum_hit_points);

    // println!("{:?}", input_string);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_reading_order() {
        let test: Vec<Coordinate> = vec![(2, 27), (3, 26), (2, 26), (1, 26), (2, 25)];
        let expected = {
            let mut x = test.clone();
            x.reverse();
            x
        };

        {
            let mut test = test.clone();
            test.sort_by(reading_order);

            assert_eq!(test, expected);
        }

        {
            let mut available_squares: BinaryHeap<DistanceCoordinate> = BinaryHeap::new();

            let items = vec![
                DistanceCoordinate(5, (1, 26)),
                DistanceCoordinate(5, (2, 25)),
                DistanceCoordinate(4, (2, 30)),
            ];
            available_squares.extend(items);

            let mut actual: Vec<Coordinate> = vec![];
            while let Some(item) = available_squares.pop() {
                actual.push(item.into());
            }

            assert_eq!(actual, vec![(2, 30), (2, 25), (1, 26)]);
        }
    }

    #[test]
    fn test_map() {
        let input_string = r###"
#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########
        "###
        .trim();

        let map = parse_input(input_string);

        assert_eq!(map.to_string(), input_string);

        assert_eq!(map.is_wall((0, 0)), true);
        assert_eq!(map.is_occupied((0, 0)), true);

        assert_eq!(map.is_wall((1, 1)), false);
        assert_eq!(map.is_occupied((1, 1)), true);

        assert_eq!(map.is_wall((2, 1)), false);
        assert_eq!(map.is_occupied((2, 1)), false);

        assert_eq!(map.get_elves().len(), 1);
        assert_eq!(map.has_elves(), true);
        assert_eq!(map.get_goblins().len(), 8);
        assert_eq!(map.has_goblins(), true);
    }

    #[test]
    fn test_is_reachable() {
        let input_string = r###"
###################
#.E...............#
#################.#
#...G.............#
###################
        "###
        .trim();

        let map = parse_input(input_string);

        assert_eq!(
            get_reachable_path(
                &map,
                (2, 1), /* position of the elf */
                (5, 3)  /* position of square adjacent to goblin */
            )
            .map(|path| path.len()),
            Some(29)
        );
    }

    #[test]
    fn test_scenario() {
        let input_string = r###"
#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########
        "###
        .trim();

        let mut map = parse_input(input_string);

        // round 1
        map.execute_round();
        assert_eq!(map.to_string(), r###"
#########
#.G...G.#
#...G...#
#...E..G#
#.G.....#
#.......#
#G..G..G#
#.......#
#########
        "###
        .trim());

        // round 2
        map.execute_round();
        assert_eq!(map.to_string(), r###"
#########
#..G.G..#
#...G...#
#.G.E.G.#
#.......#
#G..G..G#
#.......#
#.......#
#########
        "###
        .trim());

        // round 3
        map.execute_round();
        assert_eq!(map.to_string(), r###"
#########
#.......#
#..GGG..#
#..GEG..#
#G..G...#
#......G#
#.......#
#.......#
#########
        "###
        .trim());
    }

}
