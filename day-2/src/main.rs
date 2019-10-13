// https://adventofcode.com/2015/day/2

// code

#[derive(Debug)]
struct Dimensions {
    length: u32,
    width: u32,
    height: u32,
}

fn main() {
    let input_string = include_str!("input.txt");

    let dimensions = parse_input(input_string);

    println!("{:?}", dimensions);
}

fn parse_input(input_string: &str) -> Vec<Dimensions> {
    let mut output: Vec<Dimensions> = vec![];

    for input in input_string.trim().lines() {
        let input = input.trim();

        let coords: Vec<u32> = input
            .split("x")
            .map(|x| x.trim())
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let result = Dimensions {
            length: coords[0],
            width: coords[1],
            height: coords[2],
        };

        output.push(result);
    }

    return output;
}
