// https://adventofcode.com/2015/day/2

// code

#[derive(Debug)]
struct Dimensions {
    length: u32,
    width: u32,
    height: u32,
}

impl Dimensions {
    fn get_slack(&self) -> u32 {
        let axis_1 = self.length * self.width;
        let axis_2 = self.width * self.height;
        let axis_3 = self.length * self.height;

        let slacks = vec![axis_1, axis_2, axis_3];

        slacks.into_iter().min_by(|x, y| x.cmp(y)).unwrap()
    }

    fn get_area(&self) -> u32 {
        let axis_1 = 2 * self.length * self.width;
        let axis_2 = 2 * self.width * self.height;
        let axis_3 = 2 * self.length * self.height;

        axis_1 + axis_2 + axis_3 + self.get_slack()
    }

    fn required_amount_of_ribbon(&self) -> u32 {
        let axis_1 = 2 * self.length + 2 * self.width;
        let axis_2 = 2 * self.width + 2 * self.height;
        let axis_3 = 2 * self.length + 2 * self.height;

        let possible_wraps = vec![axis_1, axis_2, axis_3];

        let minimum_wrap = possible_wraps.into_iter().min_by(|x, y| x.cmp(y)).unwrap();

        let bow_tie = self.length * self.width * self.height;

        minimum_wrap + bow_tie
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    println!("Part 1: {}", part_1(input_string));

    println!("Part 2: {}", part_2(input_string));
}

fn parse_input(input_string: &str) -> Vec<Dimensions> {
    let mut output: Vec<Dimensions> = vec![];

    for input in input_string.trim().lines() {
        let input = input.trim();

        let coords: Vec<u32> = input
            .split('x')
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

    output
}

fn part_1(input_string: &str) -> u32 {
    let dimensions = parse_input(input_string);

    let sum: u32 = dimensions
        .iter()
        .fold(0, |acc, item| -> u32 { acc + item.get_area() });

    sum
}

fn part_2(input_string: &str) -> u32 {
    let dimensions = parse_input(input_string);

    let sum: u32 = dimensions.iter().fold(0, |acc, item| -> u32 {
        acc + item.required_amount_of_ribbon()
    });

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("input.txt")), 1586300);

        let input_string = r####"
        2x3x4
        "####;

        assert_eq!(part_1(input_string), 58);

        let input_string = r####"
        1x1x10
        "####;

        assert_eq!(part_1(input_string), 43);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("input.txt")), 3737498);

        let input_string = r####"
        2x3x4
        "####;

        assert_eq!(part_2(input_string), 34);

        let input_string = r####"
        1x1x10
        "####;

        assert_eq!(part_2(input_string), 14);
    }
}
