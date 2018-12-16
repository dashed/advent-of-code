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

// impl Track {
//     fn has_horizontal(&self) -> bool {
//         match self {
//             Track::Horizontal => true,
//             Track::Intersection => true,
//             _ => false,
//         }
//     }

//     fn has_vertical(&self) -> bool {
//         match self {
//             Track::Vertical => true,
//             Track::Intersection => true,
//             _ => false,
//         }
//     }
// }

type Map = HashMap<Coordinate, Track>;

#[derive(Debug, PartialEq, Eq, Hash)]
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

#[derive(Debug, Eq, PartialEq, Hash)]
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

    fn tick(&mut self, map: &Map) {
        let (x, y) = self.position;

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

type Carts = HashSet<Cart>;

fn main() {
    let input_string = include_str!("input.txt");

    let mut carts: Carts = HashSet::new();

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
                    carts.insert(cart);
                }

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

    for mut cart in carts {
        cart.tick(&map);
    }
}
