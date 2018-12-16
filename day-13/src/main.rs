// https://adventofcode.com/2018/day/13

// imports

use std::time::Duration;
use std::thread;
use std::collections::HashMap;
use std::collections::HashSet;

type Coordinate = (i32, i32);

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

    fn tick(&mut self, map: &Map) {
        let (x, y) = self.position;

        // println!("{:?}", self.position);

        // generate next position

        let next_position = match self.orientation {
            Orientation::Up => (x, y - 1),
            Orientation::Down => (x, y + 1),
            Orientation::Left => (x - 1, y),
            Orientation::Right => (x + 1, y),
        };

        self.position = next_position;

        // generate next orientation

        let next_track: Track = match map.get(&next_position) {
            None => {
                assert!(false, "No track found at: {:?}", next_position);
                unreachable!();
            }
            Some(track) => track.clone(),
        };

        match next_track {
            Track::BottomAndRight => {
                self.orientation = match self.orientation {
                    Orientation::Up => Orientation::Right,
                    Orientation::Left => Orientation::Down,
                    _ => {
                        unreachable!("Unexpected orientation: {:?}", self.orientation);
                    }
                }
            }
            Track::TopAndLeft => {
                self.orientation = match self.orientation {
                    Orientation::Down => Orientation::Left,
                    Orientation::Right => Orientation::Up,
                    _ => {
                        unreachable!("Unexpected orientation: {:?}", self.orientation);
                    }
                }
            }
            Track::BottomAndLeft => {
                self.orientation = match self.orientation {
                    Orientation::Up => Orientation::Left,
                    Orientation::Right => Orientation::Down,
                    _ => {
                        unreachable!("Unexpected orientation: {:?}", self.orientation);
                    }
                }
            }
            Track::TopAndRight => {
                self.orientation = match self.orientation {
                    Orientation::Down => Orientation::Right,
                    Orientation::Left => Orientation::Up,
                    _ => {
                        unreachable!("Unexpected orientation: {:?}", self.orientation);
                    }
                }
            }
            Track::Intersection => {
                self.orientation = self.orientation.turn(&self.turning_option);
                self.turning_option = self.turning_option.next();
            }
            _ => {}
        }
    }
}

type CrashedCarts = HashSet<Coordinate>;

struct Carts {
    carts: HashMap<Coordinate, Cart>,
}

impl Carts {
    fn new() -> Carts {
        Carts {
            carts: HashMap::new(),
        }
    }

    fn add_cart(&mut self, cart: Cart) {
        self.carts.insert(cart.position, cart);
    }

    fn get_cart(&self, position: &Coordinate) -> Option<&Cart> {
        return self.carts.get(position).clone();
    }

    fn tick(&mut self, map: &Map) -> Option<CrashedCarts> {
        let mut crashed_carts: CrashedCarts = HashSet::new();
        let mut next_positions: HashSet<Coordinate> = HashSet::new();
        let mut next_carts: HashMap<Coordinate, Cart> = HashMap::new();

        let next_carts_iter = self.carts.iter().map(|(_position, cart)| -> Cart {
            let mut cart: Cart = cart.clone();
            cart.tick(&map);
            return cart;
        });

        for cart in next_carts_iter {
            if next_positions.contains(&cart.position) {
                next_carts.remove(&cart.position);
                crashed_carts.insert(cart.position);
            } else {
                next_positions.insert(cart.position);
                next_carts.insert(cart.position, cart);
            }
        }

        self.carts = next_carts;

        if crashed_carts.len() > 0 {
            return Some(crashed_carts);
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

fn part_1(input_string: &str) -> Coordinate {


    let num_of_lines = input_string.lines().into_iter().count() as i32;
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

        println!("num_of_lines: {:?}", num_of_lines);
        println!("num_of_cols: {:?}", num_of_cols);

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

    // print_map(&map, &carts, num_of_cols - 1, num_of_lines - 1);

    let mut num_of_ticks = 0;
    loop {
        let crashed_carts = carts.tick(&map);
        num_of_ticks += 1;
        // print_map(&map, &carts, num_of_cols - 1, num_of_lines - 1);
        // thread::sleep(Duration::from_millis(500));

        match crashed_carts {
            None => {}
            Some(crashed_carts) => {

                // not: 29,104
                println!("{:?}", crashed_carts);
                println!("crashed at tick: {}", num_of_ticks);

                return crashed_carts.iter().next().unwrap().clone();
            }
        }
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    let crashed_position = part_1(input_string);

    println!("Part 1: {:?}", crashed_position);
}


// /---\
// |   v
// | /-+-\
// | | | |
// \-+-/ |
//   |   |
//   \---/


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input_string = r###"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/
        "###;

        assert_eq!(part_1(input_string), (7,3));

        let input_string = r###"/---\
| />+<--\
| | ^   |
\-+-/   |
  \-----/
        "###;

        assert_eq!(part_1(input_string), (4,1));
    }

}
