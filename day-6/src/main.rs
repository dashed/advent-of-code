// https://adventofcode.com/2018/day/6

// helpers

type Position = (i32, i32);

// https://math.stackexchange.com/a/139604/10247
fn get_manhattan_distance(x: Position, y: Position) -> i32 {
    let (a, b) = x;
    let (c, d) = y;

    return (a - c).abs() + (b - d).abs();
}

fn parse_to_coord(input: &str) -> Position {
    let result: Vec<i32> = input
        .split(',')
        .map(|x| -> i32 { x.trim().parse().unwrap() })
        .collect();

    (*result.get(0).unwrap(), *result.get(1).unwrap())
}

fn main() {
    let input_string = include_str!("input.txt");

    let destinations: Vec<Position> = input_string.lines().map(parse_to_coord).collect();

    println!("{:?}", destinations);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_manhattan_distance() {
        assert_eq!(get_manhattan_distance((0, 0), (0, 0)), 0);
        assert_eq!(get_manhattan_distance((0, 0), (3, 3)), 6);
    }
}
