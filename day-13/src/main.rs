// https://adventofcode.com/2018/day/13

// imports

use core::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;

// code

type Coordinate = (i32, i32);

// sort coordinates according to their reading order
fn reading_order(first_coord: &Coordinate, second_coord: &Coordinate) -> Ordering {
    let (x1, y1) = first_coord;
    let (x2, y2) = second_coord;

    if y1 != y2 {
        return y1.cmp(y2);
    }

    return x1.cmp(x2);
}

#[derive(PartialEq, Hash, Eq, Clone, Debug)]
struct OrderedCoordinate(Coordinate);

impl OrderedCoordinate {
    fn coordinate(&self) -> Coordinate {
        return self.0.clone();
    }
}

impl PartialOrd for OrderedCoordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(reading_order(&self.0, &other.0));
    }
}

impl Ord for OrderedCoordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.partial_cmp(other).unwrap();
        return ord;
    }
}

impl Into<OrderedCoordinate> for Coordinate {
    fn into(self) -> OrderedCoordinate {
        return OrderedCoordinate(self);
    }
}

#[derive(Debug, Clone)]
enum Track {
    // |
    Vertical,
    // -
    Horizontal,
    // +
    Intersection,

    // curves
    // invariant: Curves connect exactly two perpendicular pieces of track

    // match configuration:
    //   /-
    //   |
    BottomAndRight,

    // match configuration:
    //    |
    //   -/
    TopAndLeft,

    // match configuration:
    //   -\
    //    |
    BottomAndLeft,
    // match configuration:
    //   |
    //   \-
    TopAndRight,
}

impl Track {
    fn to_string(&self) -> String {
        let result = match self {
            Track::Vertical => "|",
            Track::Horizontal => "-",
            Track::Intersection => "+",
            Track::BottomAndRight | Track::TopAndLeft => "/",
            Track::BottomAndLeft | Track::TopAndRight => "\\",
        };

        return result.to_string();
    }
}

fn is_horizontal(cell: char) -> bool {
    match cell {
        '-' | '+' => true,
        _ => false,
    }
}

fn is_vertical(cell: char) -> bool {
    match cell {
        '|' | '+' => true,
        _ => false,
    }
}

type Map = HashMap<Coordinate, Track>;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum TurningOption {
    Left,
    Straight,
    Right,
}

impl TurningOption {
    fn next(&self) -> TurningOption {
        match self {
            TurningOption::Left => TurningOption::Straight,
            TurningOption::Straight => TurningOption::Right,
            TurningOption::Right => TurningOption::Left,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

impl Orientation {
    fn turn(&self, turning_option: &TurningOption) -> Orientation {
        match self {
            Orientation::Up => match turning_option {
                TurningOption::Left => Orientation::Left,
                TurningOption::Straight => self.clone(),
                TurningOption::Right => Orientation::Right,
            },
            Orientation::Down => match turning_option {
                TurningOption::Left => Orientation::Right,
                TurningOption::Straight => self.clone(),
                TurningOption::Right => Orientation::Left,
            },
            Orientation::Left => match turning_option {
                TurningOption::Left => Orientation::Down,
                TurningOption::Straight => self.clone(),
                TurningOption::Right => Orientation::Up,
            },
            Orientation::Right => match turning_option {
                TurningOption::Left => Orientation::Up,
                TurningOption::Straight => self.clone(),
                TurningOption::Right => Orientation::Down,
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Cart {
    orientation: Orientation,
    // current position
    position: Coordinate,
    // when a cart arrives at an intersection, this rule determines the cart's
    // next destination
    turning_option: TurningOption,
}

impl Cart {
    fn is_cart(cell: char) -> bool {
        match cell {
            '^' | 'v' | '<' | '>' => true,
            _ => false,
        }
    }

    fn new(cell: char, position: Coordinate) -> Cart {
        assert!(Cart::is_cart(cell));

        let orientation = match cell {
            '^' => Orientation::Up,
            'v' => Orientation::Down,
            '<' => Orientation::Left,
            '>' => Orientation::Right,
            _ => {
                unreachable!();
            }
        };

        Cart {
            orientation,
            position,
            turning_option: TurningOption::Left,
        }
    }

    fn to_string(&self) -> String {
        let orientation = match self.orientation {
            Orientation::Up => "^",
            Orientation::Down => "v",
            Orientation::Left => "<",
            Orientation::Right => ">",
        };

        return orientation.to_string();
    }

    fn tick(&self, map: &Map) -> Cart {
        let (x, y) = self.position;

        // println!("{:?}", self.position);

        // generate next position

        let next_position = match self.orientation {
            Orientation::Up => (x, y - 1),
            Orientation::Down => (x, y + 1),
            Orientation::Left => (x - 1, y),
            Orientation::Right => (x + 1, y),
        };

        // generate next orientation

        let next_track: Track = match map.get(&next_position) {
            None => {
                assert!(false, "No track found at: {:?}", next_position);
                unreachable!();
            }
            Some(track) => track.clone(),
        };

        let mut next_orientation = self.orientation.clone();
        let mut next_turning_option = self.turning_option.clone();

        match next_track {
            Track::BottomAndRight => {
                next_orientation = match self.orientation {
                    Orientation::Up => Orientation::Right,
                    Orientation::Left => Orientation::Down,
                    _ => {
                        unreachable!("Unexpected orientation: {:?}", self.orientation);
                    }
                }
            }
            Track::TopAndLeft => {
                next_orientation = match self.orientation {
                    Orientation::Down => Orientation::Left,
                    Orientation::Right => Orientation::Up,
                    _ => {
                        unreachable!("Unexpected orientation: {:?}", self.orientation);
                    }
                }
            }
            Track::BottomAndLeft => {
                next_orientation = match self.orientation {
                    Orientation::Up => Orientation::Left,
                    Orientation::Right => Orientation::Down,
                    _ => {
                        unreachable!("Unexpected orientation: {:?}", self.orientation);
                    }
                }
            }
            Track::TopAndRight => {
                next_orientation = match self.orientation {
                    Orientation::Down => Orientation::Right,
                    Orientation::Left => Orientation::Up,
                    _ => {
                        unreachable!("Unexpected orientation: {:?}", self.orientation);
                    }
                }
            }
            Track::Intersection => {
                next_orientation = self.orientation.turn(&self.turning_option);
                next_turning_option = self.turning_option.next();
            }
            Track::Vertical => {
                match self.orientation {
                    Orientation::Up | Orientation::Down => {}
                    _ => {
                        unreachable!("Unexpected orientation: {:?}", self.orientation);
                    }
                };
            }
            Track::Horizontal => {
                match self.orientation {
                    Orientation::Left | Orientation::Right => {}
                    _ => {
                        unreachable!("Unexpected orientation: {:?}", self.orientation);
                    }
                };
            }
        }

        Cart {
            orientation: next_orientation,
            position: next_position,
            turning_option: next_turning_option,
        }
    }
}

type CrashedCarts = HashSet<Coordinate>;

struct Carts {
    carts: BTreeMap<OrderedCoordinate, Cart>,
}

impl Carts {
    fn new() -> Carts {
        Carts {
            carts: BTreeMap::new(),
        }
    }

    fn add_cart(&mut self, cart: Cart) {
        self.carts.insert(cart.position.into(), cart);
    }

    fn get_cart(&self, position: &Coordinate) -> Option<&Cart> {
        let position: Coordinate = position.clone();
        return self.carts.get(&position.into()).clone();
    }

    fn tick(&mut self, map: &Map) -> Option<CrashedCarts> {
        let mut crashed_positions: HashSet<Coordinate> = HashSet::new();

        let (_, next_carts) = self.carts.iter().fold(
            (self.carts.clone(), BTreeMap::new()),
            |acc, (_current_position, current_cart)| {
                let (mut prev_carts, mut next_carts): (
                    BTreeMap<OrderedCoordinate, Cart>,
                    BTreeMap<OrderedCoordinate, Cart>,
                ) = acc;

                // remove current cart from the current map state
                prev_carts.remove(&current_cart.position.into());

                if crashed_positions.contains(&current_cart.position) {
                    // current cart was crashed by another cart that moved before itself.
                    return (prev_carts, next_carts);
                }

                let next_cart = current_cart.tick(&map);

                // does the next cart collide with any other cart in the map state?
                if prev_carts.contains_key(&next_cart.position.into()) {
                    crashed_positions.insert(next_cart.position);
                    return (prev_carts, next_carts);
                }

                // does the next cart collide with carts that already have moved?
                if next_carts.contains_key(&next_cart.position.into()) {
                    crashed_positions.insert(next_cart.position);
                    next_carts.remove(&next_cart.position.into());
                    return (prev_carts, next_carts);
                }

                next_carts.insert(next_cart.position.into(), next_cart);

                return (prev_carts, next_carts);
            },
        );

        // invariant: by the end of this tick, next_carts contain carts that haven't crashed

        self.carts = next_carts;

        if crashed_positions.len() > 0 {
            return Some(crashed_positions);
        }

        return None;
    }
}

#[allow(dead_code)]
fn print_map(map: &Map, carts: &Carts, max_x: i32, max_y: i32) {
    for y in 0..=max_y {
        for x in 0..=max_x {
            let position = (x, y);

            match carts.get_cart(&position) {
                None => match map.get(&position) {
                    None => {
                        print!(" ");
                    }
                    Some(track) => {
                        print!("{}", track.to_string());
                    }
                },
                Some(cart) => {
                    print!("{}", cart.to_string());
                }
            }
        }

        println!("");
    }
}

fn parse_input(input_string: &str) -> (Map, Carts) {
    #[allow(unused_variables)]
    let num_of_lines = input_string.lines().into_iter().count() as i32;
    #[allow(unused_variables)]
    let num_of_cols = input_string
        .lines()
        .into_iter()
        .map(|x| x.len())
        .max()
        .unwrap() as i32;

    let mut carts: Carts = Carts::new();

    let map: Map = {
        let mut map: Map = HashMap::new();

        let mut cell_map: HashMap<Coordinate, char> = HashMap::new();

        for (y, line) in input_string.lines().enumerate() {
            for (x, cell) in line.chars().enumerate() {
                let position: Coordinate = (x as i32, y as i32);
                // println!("{:?} {}", position, cell);

                // add carts
                if Cart::is_cart(cell) {
                    let cart = Cart::new(cell, position);
                    carts.add_cart(cart);
                }

                // convert cart to appropriate track
                let cell = match cell {
                    'v' | '^' => '|',
                    '<' | '>' => '-',
                    _ => cell,
                };

                cell_map.insert(position, cell);
            }
        }

        for (position, cell) in cell_map.iter() {
            let (x, y) = position.clone();
            let position = position.clone();

            match cell {
                '|' => {
                    map.insert(position, Track::Vertical);
                }
                '-' => {
                    map.insert(position, Track::Horizontal);
                }
                '+' => {
                    map.insert(position, Track::Intersection);
                }
                '/' => {
                    // match configuration:
                    //   /-
                    //   |
                    let is_configuration_1 = {
                        let valid_right_side = match cell_map.get(&(x + 1, y)) {
                            None => false,
                            Some(cell) => is_horizontal(*cell),
                        };

                        let valid_bottom_side = match cell_map.get(&(x, y + 1)) {
                            None => false,
                            Some(cell) => is_vertical(*cell),
                        };

                        valid_right_side && valid_bottom_side
                    };

                    // match configuration:
                    //    |
                    //   -/
                    let is_configuration_2 = {
                        let valid_left_side = match cell_map.get(&(x - 1, y)) {
                            None => false,
                            Some(cell) => is_horizontal(*cell),
                        };

                        let valid_top_side = match cell_map.get(&(x, y - 1)) {
                            None => false,
                            Some(cell) => is_vertical(*cell),
                        };

                        valid_left_side && valid_top_side
                    };

                    if is_configuration_1 && !is_configuration_2 {
                        map.insert(position, Track::BottomAndRight);
                        continue;
                    }

                    if !is_configuration_1 && is_configuration_2 {
                        map.insert(position, Track::TopAndLeft);
                        continue;
                    }

                    assert!(
                        false,
                        format!("Invalid placement of track: / at {:?}", position)
                    );
                }
                '\\' => {
                    // match configuration:
                    //   -\
                    //    |
                    let is_configuration_1 = {
                        let valid_left_side = match cell_map.get(&(x - 1, y)) {
                            None => false,
                            Some(cell) => is_horizontal(*cell),
                        };

                        let valid_bottom_side = match cell_map.get(&(x, y + 1)) {
                            None => false,
                            Some(cell) => is_vertical(*cell),
                        };

                        valid_left_side && valid_bottom_side
                    };

                    // match configuration:
                    //   |
                    //   \-
                    let is_configuration_2 = {
                        let valid_top_side = match cell_map.get(&(x, y - 1)) {
                            None => false,
                            Some(cell) => is_vertical(*cell),
                        };

                        let valid_right_side = match cell_map.get(&(x + 1, y)) {
                            None => false,
                            Some(cell) => is_horizontal(*cell),
                        };

                        valid_top_side && valid_right_side
                    };

                    if is_configuration_1 && !is_configuration_2 {
                        map.insert(position, Track::BottomAndLeft);
                        continue;
                    }

                    if !is_configuration_1 && is_configuration_2 {
                        map.insert(position, Track::TopAndRight);
                        continue;
                    }

                    assert!(
                        false,
                        format!("Invalid placement of track: \\ at {:?}", position)
                    );
                }
                ' ' => {}
                _ => {
                    assert!(false, "Unknown cell at {:?}: {}", position, cell);
                }
            }
        }

        map
    };

    return (map, carts);
}

fn part_1(input_string: &str) -> Coordinate {
    let (map, mut carts) = parse_input(input_string);

    // print_map(&map, &carts, num_of_cols - 1, num_of_lines - 1);

    loop {
        let crashed_carts = carts.tick(&map);
        // num_of_ticks += 1;
        // print_map(&map, &carts, num_of_cols - 1, num_of_lines - 1);
        // println!("==========");
        // use std::thread;
        // use std::time::Duration;
        // thread::sleep(Duration::from_millis(100));

        match crashed_carts {
            None => {}
            Some(crashed_carts) => {
                // not: 29,104

                let mut crashed_carts: Vec<Coordinate> = crashed_carts.into_iter().collect();
                crashed_carts.sort();

                // println!("{:?}", crashed_carts);
                // println!("crashed at tick: {}", num_of_ticks);

                return *crashed_carts.first().unwrap();
            }
        }
    }
}

fn part_2(input_string: &str) -> Option<Coordinate> {
    let (map, mut carts) = parse_input(input_string);

    // print_map(&map, &carts, num_of_cols - 1, num_of_lines - 1);

    loop {
        carts.tick(&map);
        // num_of_ticks += 1;
        // print_map(&map, &carts, num_of_cols - 1, num_of_lines - 1);
        // println!("==========");
        // use std::thread;
        // use std::time::Duration;
        // thread::sleep(Duration::from_millis(100));

        if carts.carts.len() <= 1 {
            return carts
                .carts
                .iter()
                .map(|(position, _cart)| -> Coordinate {
                    return position.coordinate();
                })
                .into_iter()
                .next();
        }
    }
}

fn main() {

    let input_string = include_str!("input.txt");

    let crashed_position = part_1(input_string);

    println!("Part 1: {:?}", crashed_position);

    let survivor = part_2(input_string);
    println!("Part 2: {:?}", survivor);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordered_coordinate() {
        let test: Vec<Coordinate> = vec![(2, 27), (3, 26), (2, 26), (1, 26), (2, 25)];
        let test: Vec<OrderedCoordinate> = test.into_iter().map(|x| x.into()).collect();
        let expected = {
            let mut test = test.clone();
            test.reverse();
            test
        };

        let mut actual = test.clone();
        actual.sort();

        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic]
    fn near_miss() {
        let input_string = r###"
   |
   |
->-+---
   |
   |
   ^
"###;

        part_1(input_string);
    }

    #[test]
    fn test_part_1() {
        let input_string = r###"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/
        "###;

        assert_eq!(part_1(input_string), (7, 3));

        // first crash occurs from left to right
        let input_string = r###"->-<-->-<--"###;

        assert_eq!(part_1(input_string), (2, 0));

        // carts shouldn't pass through each other
        let input_string = r###"--->--<---"###;

        assert_eq!(part_1(input_string), (5, 0));

        let input_string = r###"|
|
v
|
|
^
|
|
"###;

        assert_eq!(part_1(input_string), (0, 4));

        let input_string = r###"->+<-
  ^  "###;

        assert_eq!(part_1(input_string), (2, 0));

        let input_string = r###"-->>--"###;

        assert_eq!(part_1(input_string), (3, 0));

        let input_string = include_str!("input.txt");

        assert_eq!(part_1(input_string), (76, 108));
    }

    #[test]
    fn test_part_2() {
        let input_string = r###"/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/
        "###;

        assert_eq!(part_2(input_string), Some((6, 4)));
    }

}
