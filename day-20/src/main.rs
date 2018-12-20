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

#[derive(Debug)]
struct Route(Vec<OpenDirections>);

enum Branches {
    // invariant: branches must have at least one option
    CanSkip(Box<Routes>, Vec<Routes>),
    CannotSkip(Box<Routes>, Vec<Routes>),
}

struct BranchGroup(Branches);

enum Routes {
    Route(Route, Vec<Routes>),
    Branch(BranchGroup, Vec<Routes>),
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

fn parse_route(tokens: &Vec<Tokens>, start_at: TokenPosition) -> ParseResult<Route> {
    let mut current_position = start_at;
    let mut route = vec![];

    loop {
        match tokens.get(current_position) {
            None => {
                break;
            }
            Some(token) => {
                if let Tokens::OpenDirection(direction) = token {
                    route.push(direction.clone());
                    current_position += 1;
                    continue;
                }
                break;
            }
        }
    }

    if route.len() <= 0 {
        // no route was parsed
        return None;
    }

    let next_position = start_at + route.len();
    let result = Route(route);

    return Some((result, next_position));
}

fn parse_routes(tokens: &Vec<Tokens>, start_at: TokenPosition) -> ParseResult<Routes> {
    let parse_result = parse_route(tokens, start_at);

    if parse_result.is_some() {
        let route = parse_result.unwrap();

        println!("{:?}", route);
    }

    return None;
}

type Distance = i32;

#[derive(Debug, PartialEq, Clone)]
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

        // parse starting token

        match parse_start(&tokenized, current_position) {
            None => {
                panic!("Expect starting token: ^");
            }
            Some((_, next_position)) => {
                current_position = next_position;
            }
        }

        // parse routes
        let routes: Routes = match parse_routes(&tokenized, current_position) {
            None => {
                panic!("No routes generated");
            }
            Some((routes, next_position)) => {
                current_position = next_position;
                routes
            }
        };

        // TODO: parse ending token
    }

    // parse route starting from the current position
    fn parse_route(&self, current_position: Coordinate) {}
}

fn main() {
    let input_string = include_str!("input.txt");

    let map = Map::new();

    map.parse_input(input_string);
}
