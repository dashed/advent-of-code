// https://adventofcode.com/2018/day/3

// imports

use std::str::Lines;

// part 1

#[derive(Debug, PartialEq)]
struct Fabric<'id> {
    id: &'id str,

    // starting coordinates
    left: i32,
    top: i32,

    // size
    height: i32,
    width: i32,
}

fn parse_to_fabric(input: &str) -> Fabric {
    let mut parts = input.split_whitespace();

    let id = parts.next().unwrap();

    let (left, top): (i32, i32) = {
        let location_string = parts.nth(1).unwrap().to_string();

        let location_string: String = location_string
            .chars()
            // ignore the last charcter which is expected to be a colon :
            .take(location_string.len() - 1)
            .collect();

        let locations: Vec<i32> = location_string
            .split(',')
            .map(|x| -> i32 { return x.parse().unwrap() })
            .collect();

        (*locations.get(0).unwrap(), *locations.get(1).unwrap())
    };

    let (height, width): (i32, i32) = {
        let size_string = parts.next().unwrap().to_string();

        let sizes: Vec<i32> = size_string
            .split('x')
            .map(|x| -> i32 { return x.parse().unwrap() })
            .collect();

        (*sizes.get(0).unwrap(), *sizes.get(1).unwrap())
    };

    Fabric {
        id: id,
        left: left,
        top: top,
        height: height,
        width: width,
    }
}

fn part_1(inputs: Lines) {
    for input in inputs {
        let fabric = parse_to_fabric(input);
        println!("{}", input);
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    let inputs = input_string.lines();

    // part_1(inputs);

    let fabric = parse_to_fabric("#123 @ 3,2: 5x4");

    println!("{:?}", fabric);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_to_fabric() {
        let expected = Fabric {
            id: "#123",
            left: 3,
            top: 2,
            height: 5,
            width: 4,
        };

        assert_eq!(parse_to_fabric("#123 @ 3,2: 5x4"), expected);
    }

}
