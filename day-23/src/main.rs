// https://adventofcode.com/2018/day/23

// imports

extern crate combine;
use combine::combinator::token;
use combine::parser::char::{char, digit, letter, spaces};
use combine::stream::easy;
use combine::{between, choice, many1, sep_by, Parser};

// code

// adapted from day 6
// https://math.stackexchange.com/a/139604/10247
type Distance = i32;
fn get_manhattan_distance(start: Coordinate, end: Coordinate) -> Distance {
    let (a, b, e) = start;
    let (c, d, f) = end;

    return (a - c).abs() + (b - d).abs() + (e - f).abs();
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
    let integer = many1(digit()).map(|string: String| -> i32 {
        return string.parse::<i32>().unwrap();
    });

    let negative_integer = (char('-'), integer.clone()).map(|(_, parsed_int): (_, i32)| -> i32 {
        return -parsed_int;
    });

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

    let parse_position = (position_start, char('='), coord_list).map(
        |(_, _, list): (_, _, Vec<i32>)| -> Coordinate {
            return (list[0], list[1], list[2]);
        },
    );

    let parse_radius = (char('r'), char('='), integer).map(|(_, _, radius): (_, _, Radius)| {
        return radius;
    });

    let mut parse_nanobot = (parse_position, char(',').skip(spaces()), parse_radius).map(
        |(position, _, radius): (Coordinate, _, Radius)| -> NanoBot {
            return NanoBot::new(position, radius);
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

fn main() {
    let input_string = include_str!("input.txt");

    let nanobots: Vec<NanoBot> = input_string
        .lines()
        .map(|s| s.trim())
        .map(|s| parse_nanobot(s))
        .collect();

    let strongest_nanobot: NanoBot = nanobots
        .iter()
        .max_by_key(|b| {
            return b.radius;
        })
        .unwrap()
        .clone();

    let num_in_range: Vec<NanoBot> = nanobots
        .iter()
        .filter(|bot| {
            return get_manhattan_distance(bot.position, strongest_nanobot.position)
                <= strongest_nanobot.radius;
        })
        .map(|b| b.clone())
        .collect();

    println!("Part 1: {}", num_in_range.len());
}
