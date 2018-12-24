// https://adventofcode.com/2018/day/23

// imports

extern crate combine;
use combine::combinator::{skip, token};
use combine::parser::char::{char, digit, letter, spaces};
use combine::stream::easy;
use combine::{between, choice, many1, sep_by, Parser};

// code

type Radius = i32;
type Coordinate = (i32, i32, i32);

#[derive(Debug)]
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

    println!("{:?}", nanobots);
}
