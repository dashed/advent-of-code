use std::collections::HashMap;
use std::collections::HashSet;

type Coordinate = (i32, i32);

enum Track {
    // |
    Vertical,
    // -
    Horizontal,
    // +
    Intersection,
    // top to left /
    TopToLeft,
    // top to right \
    TopToRight,
    // bottom to left /
    BottomToLeft,
    // bottom to right \
    BottomToRight,
}

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

#[derive(Debug, Eq, PartialEq, Hash)]
struct Cart {
    current_position: Coordinate,
    // when a cart arrives at an intersection, this rule determines the cart's
    // next destination
    turning_option: TurningOption,
}

type Carts = HashSet<Cart>;

fn main() {
    let input_string = include_str!("input.txt");

    println!("{:?}", input_string);

    let carts: Carts = HashSet::new();
}
