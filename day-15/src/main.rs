// https://adventofcode.com/2018/day/15

// imports

use std::collections::BTreeMap;
use std::collections::HashMap;

// code

type Coordinate = (i32, i32);

// adapted from day 6
// https://math.stackexchange.com/a/139604/10247
fn get_manhattan_distance(start: Coordinate, end: Coordinate) -> i32 {
    let (a, b) = start;
    let (c, d) = end;

    return (a - c).abs() + (b - d).abs();
}

// pick a coordinate from a vector of coordinates according to the reading order rules
fn pick_coord(mut coords: Vec<Coordinate>) -> Coordinate {
    assert!(coords.len() > 0);
    coords.sort();
    return coords.first().unwrap().clone();
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

    fn is_wall(&self, position: &Coordinate) -> bool {
        match self.terrain.get(position) {
            None => true,
            Some(map_state) => match map_state {
                MapState::Wall => true,
                MapState::Cavern => false,
            },
        }
    }

    fn is_occupied(&self, position: &Coordinate) -> bool {
        if self.is_wall(position) {
            return true;
        }

        // check if the position is occupied by a unit
        return self.units.contains_key(position);
    }

    fn get_elves(&self) -> Vec<&Unit> {
        return self
            .units
            .iter()
            .filter(|(_position, unit)| {
                return unit.is_elf();
            })
            .map(|(_position, unit)| unit)
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

    fn get_goblins(&self) -> Vec<&Unit> {
        return self
            .units
            .iter()
            .filter(|(_position, unit)| {
                return unit.is_goblin();
            })
            .map(|(_position, unit)| unit)
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

    // returns true if combat has ended (i.e. round didn't run)
    fn execute_round(&mut self) -> bool {
        if !self.can_run_round() {
            return true;
        }

        for (position, unit) in self.units.iter() {
            // Each unit begins its turn by identifying all possible targets (enemy units).
            let targets = if unit.is_elf() {
                self.get_goblins()
            } else if unit.is_goblin() {
                self.get_elves()
            } else {
                unreachable!();
            };

            // If no targets remain, combat ends.
            if targets.len() <= 0 {
                return true;
            }

        }

        return false;
    }
}

enum UnitType {
    Goblin,
    Elf,
}

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

fn is_reachable(map: Map, start: Coordinate, end: Coordinate) -> bool {
    // TODO: apply shortest path algorithm

    if map.is_wall(&start) || map.is_wall(&end) {
        return false;
    }

    if map.is_occupied(&end) {
        return false;
    }

    // backtrack from end towards start
    let current_position = end;

    while current_position != start {
        // TODO: implement
    }

    return false;
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

    for (y, line) in input_string.lines().enumerate() {
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

    map.execute_round();

    // println!("{:?}", input_string);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pick_coord() {
        assert_eq!(pick_coord(vec![(1, 1), (0, 0), (1, 0)]), (0, 0));
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

        assert_eq!(parse_input(input_string).to_string(), input_string);
    }
}
