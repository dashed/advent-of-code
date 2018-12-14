use std::collections::HashMap;

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

enum TurningOption {
    Left,
    Straight,
    Right,
}

struct Cart {
    current_position: Coordinate,
    // when a cart arrives at an intersection, this rule determines the cart's
    // next destination
    turning_option: TurningOption,
}

fn main() {
    println!("Hello, world!");
}
