// https://adventofcode.com/2018/day/23

// imports

extern crate combine;
use combine::combinator::token;
use combine::parser::char::{char, digit, letter, spaces};
use combine::stream::easy;
use combine::{between, choice, many1, sep_by, Parser};
use std::cmp;
use std::collections::BTreeMap;

// code

// adapted from day 6
// https://math.stackexchange.com/a/139604/10247
type Distance = i32;
fn get_manhattan_distance(start: Coordinate, end: Coordinate) -> Distance {
    let (a, b, e) = start;
    let (c, d, f) = end;

    (a - c).abs() + (b - d).abs() + (e - f).abs()
}

type Radius = i32;
type Coordinate = (i32, i32, i32);

#[derive(Debug, Clone)]
struct NanoBot {
    position: Coordinate,
    radius: Radius,
}

impl NanoBot {
    fn new(position: Coordinate, radius: Radius) -> Self {
        NanoBot { position, radius }
    }
}

fn parse_nanobot(input: &str) -> NanoBot {
    let integer = many1(digit()).map(|string: String| -> i32 { string.parse::<i32>().unwrap() });

    let negative_integer =
        (char('-'), integer.clone()).map(|(_, parsed_int): (_, i32)| -> i32 { -parsed_int });

    let parse_integer = choice((negative_integer, integer.clone()));

    let parse_integer_list = sep_by(parse_integer, spaces().skip(char(',')));

    let position_start = many1(letter()).and_then(|word: String| {
        if word == "pos" {
            Ok(word)
        } else {
            Err(easy::Error::Expected(easy::Info::Borrowed("pos")))
        }
    });

    let coord_list = between(token('<'), token('>'), parse_integer_list);

    let parse_position = (position_start, char('='), coord_list)
        .map(|(_, _, list): (_, _, Vec<i32>)| -> Coordinate { (list[0], list[1], list[2]) });

    let parse_radius = (char('r'), char('='), integer).map(|(_, _, radius): (_, _, Radius)| radius);

    let mut parse_nanobot = (parse_position, char(',').skip(spaces()), parse_radius).map(
        |(position, _, radius): (Coordinate, _, Radius)| -> NanoBot {
            NanoBot::new(position, radius)
        },
    );

    let result: Result<(NanoBot, &str), easy::ParseError<&str>> = parse_nanobot.easy_parse(input);

    match result {
        Ok((value, _remaining_input)) => {
            return value;
        }
        Err(err) => println!("{}", err),
    }

    unreachable!();
}

fn part_1(input_string: String) -> usize {
    let nanobots: Vec<NanoBot> = input_string
        .trim()
        .lines()
        .map(|s| s.trim())
        .map(parse_nanobot)
        .collect();

    let strongest_nanobot: NanoBot = nanobots.iter().max_by_key(|b| b.radius).unwrap().clone();

    let num_in_range: Vec<NanoBot> = nanobots
        .iter()
        .filter(|bot| {
            get_manhattan_distance(bot.position, strongest_nanobot.position)
                <= strongest_nanobot.radius
        })
        .cloned()
        .collect();

    num_in_range.len()
}

fn part_2(input_string: String) -> i32 {
    let nanobots: Vec<NanoBot> = input_string
        .trim()
        .lines()
        .map(|s| s.trim())
        .map(parse_nanobot)
        .collect();

    let queue: Vec<(i32, i32)> = nanobots
        .into_iter()
        .map(|bot| {
            let distance = get_manhattan_distance(bot.position, (0, 0, 0));

            let segments = vec![
                (cmp::max(0, distance - bot.radius), 1),
                (distance + bot.radius + 1, -1),
            ];

            segments
        })
        .flat_map(|s| s.into_iter())
        .collect();

    let mut btree = BTreeMap::new();
    btree.extend(queue);

    let mut count = 0;
    let mut max_count = 0;
    let mut result = 0;

    for (distance, e) in btree {
        count += e;

        if count > max_count {
            result = distance;
            max_count = count;
        }
    }

    result
}

fn main() {
    let input_string = include_str!("input.txt");

    println!("Part 1: {}", part_1(input_string.to_string()));

    // Part 2
    // https://old.reddit.com/r/adventofcode/comments/a8s17l/2018_day_23_solutions/ecdqzdg/

    // visualization
    //       <===A===> <==B==>
    // <==C==>             <==D==>
    //     <======E======>
    // 1111223222222221222122211110 :: Bot count per x-coordinate
    //       ^                      :: Point of maximum intersection
    //
    // https://old.reddit.com/r/adventofcode/comments/a8s17l/2018_day_23_solutions/ecez07o/

    println!("Part 2: {}", part_2(input_string.to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r###"
pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1
        "###;

        assert_eq!(part_1(input.to_string()), 7);

        let input_string = include_str!("input.txt");
        assert_eq!(part_1(input_string.to_string()), 737);
    }

    #[test]
    fn test_part_2() {
        let input_string = include_str!("input.txt");

        assert_eq!(part_2(input_string.to_string()), 123356173);
    }
}
