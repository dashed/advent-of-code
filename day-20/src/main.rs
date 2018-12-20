// https://adventofcode.com/2018/day/20

// imports

use std::collections::HashMap;
use std::collections::HashSet;

// code

/*

Based on https://adriann.github.io/rust_parser.html

Grammar:

branches
    routes |            (with a skip option; i.e. the entire branch group can be effectively skipped)
    routes | routes
    routes | branches


branch_group
    ( branches )

routes
    branch_group routes
    route routes
    route
    branch_group

route
    direction route
    direction

direction:
    N, S, W, E

start := ^
end := $
input -> start routes end

*/

// https://en.wikipedia.org/wiki/Lexical_analysis#Tokenization
#[derive(Debug, PartialEq)]
enum Tokens {
    Start,
    End,
    OpenDirection(OpenDirections),
    ParenOpen,
    ParenClose,
    BranchOr,
}

fn tokenize(input_string: &str) -> Vec<Tokens> {
    return input_string
        .trim()
        .chars()
        .map(|c| match c {
            '^' => Tokens::Start,
            '$' => Tokens::End,
            '(' => Tokens::ParenOpen,
            ')' => Tokens::ParenClose,
            '|' => Tokens::BranchOr,
            _ => Tokens::OpenDirection(OpenDirections::from_char(c)),
        })
        .collect();
}

struct Route(Vec<OpenDirections>);

enum Branches {
    // invariant: branches must have at least one option
    CanSkip(Routes, Vec<Routes>),
    CannotSkip(Routes, Vec<Routes>),
}

struct BranchGroup(Branches);

// TODO: flesh out
enum Routes {
    Route,
    Branch,
}

// keep track where in the token stream to start reading tokens from
type TokenPosition = usize;

// after parsing/consuming a portion of the token stream; a result T might be generated.
// the starting position of the token stream for the next parse is also returned
//
// if None was returned, this indicates that parsing wasn't successful,
// and the token stream was not consumed
//
type ParseResult<T> = Option<(T, TokenPosition)>;

fn parse_start(tokens: &Vec<Tokens>, start_at: TokenPosition) -> ParseResult<()> {
    match tokens.get(start_at) {
        None => {
            return None;
        }
        Some(token) => {
            if token == &Tokens::Start {
                return Some(((), start_at + 1));
            }
            return None;
        }
    }
}

fn parse_end(tokens: &Vec<Tokens>, start_at: TokenPosition) -> ParseResult<()> {
    match tokens.get(start_at) {
        None => {
            return None;
        }
        Some(token) => {
            if token == &Tokens::End {
                return Some(((), start_at + 1));
            }
            return None;
        }
    }
}

type Distance = i32;

#[derive(Debug, PartialEq)]
enum OpenDirections {
    North,
    South,
    West,
    East,
}

impl OpenDirections {
    fn from_char(d: char) -> Self {
        match d {
            'N' => OpenDirections::North,
            'S' => OpenDirections::South,
            'W' => OpenDirections::West,
            'E' => OpenDirections::East,
            _ => {
                unreachable!();
            }
        }
    }
}

type Coordinate = (i32, i32);

trait Transitions {
    fn north(&self) -> Coordinate;
    fn south(&self) -> Coordinate;
    fn west(&self) -> Coordinate;
    fn east(&self) -> Coordinate;
}

impl Transitions for Coordinate {
    fn north(&self) -> Coordinate {
        let (x, y) = self;
        return (*x, y - 1);
    }

    fn south(&self) -> Coordinate {
        let (x, y) = self;
        return (*x, y + 1);
    }

    fn west(&self) -> Coordinate {
        let (x, y) = self;
        return (x - 1, *y);
    }

    fn east(&self) -> Coordinate {
        let (x, y) = self;
        return (x + 1, *y);
    }
}

struct Map {
    // a coordinate indicates a room;
    // and from this room indicates possible directions to go into
    map: HashMap<Coordinate, HashSet<OpenDirections>>,

    // the fewest number of doors to pass through to reach room defined by Coordinate
    room_distance: HashMap<Coordinate, Distance>,
}

// invariant: the routes will take you through every door in the facility at least once

impl Map {
    fn new() -> Self {
        Map {
            map: HashMap::new(),

            room_distance: HashMap::new(),
        }
    }

    fn parse_input(&self, input_string: &str) {
        let tokenized = tokenize(input_string);

        // starting position in the token stream
        let mut current_position = 0;

        match parse_start(&tokenized, current_position) {
            None => {
                panic!("Expect starting token: ^");
            }
            Some((_, next_position)) => {
                current_position = next_position;
            }
        }
    }

    // parse route starting from the current position
    fn parse_route(&self, current_position: Coordinate) {}
}

fn main() {
    let input_string = include_str!("input.txt");

    let map = Map::new();

    map.parse_input(input_string);
}
