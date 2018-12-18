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

fn parse_input(input_string: &str) {
    let map = Map::new();

    for (y, line) in input_string.lines().enumerate() {
        for (x, map_state_as_char) in line.chars().enumerate() {
            let position: Coordinate = (x as i32, y as i32);

            print!("{}", map_state_as_char);
        }
        println!("");
    }
}

fn main() {
    // ensures reading order is satisfied
    assert!((0, 0) < (1, 0));
    assert!((0, 0) < (0, 1));
    assert!((0, 0) < (1, 1));
    assert!((1, 0) < (1, 1));
    assert!((0, 0) < (1, 1));

    let input_string = include_str!("input.txt");

    parse_input(input_string);

    // println!("{:?}", input_string);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pick_coord() {
        assert_eq!(pick_coord(vec![(1, 1), (0, 0), (1, 0)]), (0, 0));
    }
}
