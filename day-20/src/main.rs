// https://adventofcode.com/2018/day/20

// imports

use std::collections::HashMap;
use std::collections::HashSet;

// code

/*

Based on https://adriann.github.io/rust_parser.html

Grammar:

branches:
    routes |            (with a skip option; i.e. the entire branch group can be effectively skipped)
    routes | routes
    routes | branches


branch_group:
    ( branches )

routes:
    route
    branch_group
    route routes
    branch_group routes

route:
    direction route
    direction

direction:
    N, S, W, E

start := ^
end := $
input:
    start routes end

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

#[derive(Debug, Clone)]
struct Route(Vec<OpenDirections>);

impl Route {
    fn to_string(&self) -> String {
        let directions = &self.0;
        let directions: Vec<String> = directions.iter().map(|d| d.to_string()).collect();

        return directions.join("");
    }
}

#[derive(Debug, Clone)]
enum Branches {
    // invariant: branches must have at least one option

    // this allows any of the options to be skipped
    CanSkip(Box<Routes>, Vec<Routes>),

    // only one of the options must be chosen
    CannotSkip(Box<Routes>, Vec<Routes>),
}

impl Branches {
    fn to_string(&self) -> String {
        match self {
            Branches::CanSkip(routes, rest_routes) => {
                let rest_str: Vec<String> = rest_routes
                    .iter()
                    .map(|routes| routes.to_string())
                    .collect();

                if rest_str.len() <= 0 {
                    return format!("{}|", routes.to_string());
                }

                return format!("{}|{}|", routes.to_string(), rest_str.join("|"));
            }
            Branches::CannotSkip(routes, rest_routes) => {
                let rest_str: Vec<String> = rest_routes
                    .iter()
                    .map(|routes| routes.to_string())
                    .collect();
                return format!("{}|{}", routes.to_string(), rest_str.join("|"));
            }
        }
    }
}

#[derive(Debug, Clone)]
struct BranchGroup(Branches);

impl BranchGroup {
    fn to_string(&self) -> String {
        let branches = &self.0;
        return format!("({})", branches.to_string());
    }
}

// NOTE: wrap in box because recursive types is illegal in Rust
// https://stackoverflow.com/questions/25296195/why-are-recursive-struct-types-illegal-in-rust
#[derive(Debug, Clone)]
enum Routes {
    Route(Route, Box<Option<Routes>>),
    Branch(BranchGroup, Box<Option<Routes>>),
}

impl Routes {
    fn to_string(&self) -> String {
        match self {
            Routes::Route(route, routes) => {
                let rest: String = if routes.is_none() {
                    "".to_string()
                } else {
                    routes.clone().unwrap().to_string()
                };
                return format!("{}{}", route.to_string(), rest);
            }
            Routes::Branch(branch_group, routes) => {
                let rest: String = if routes.is_none() {
                    "".to_string()
                } else {
                    routes.clone().unwrap().to_string()
                };
                return format!("{}{}", branch_group.to_string(), rest);
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Directions(Routes);

impl Directions {
    fn to_string(&self) -> String {
        return format!("^{}$", self.0.to_string());
    }
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

fn parse_branches(tokens: &Vec<Tokens>, start_at: TokenPosition) -> ParseResult<Branches> {
    let mut current_position = start_at;

    let starting_routes: Box<Routes> = match parse_routes(tokens, current_position) {
        None => {
            return None;
        }
        Some((routes, next_position)) => {
            current_position = next_position;
            Box::new(routes)
        }
    };

    // parse branch or: |

    match tokens.get(current_position) {
        None => {
            return None;
        }
        Some(token) => {
            if token == &Tokens::BranchOr {
                current_position += 1;
            } else {
                return None;
            }
        }
    }

    // parse for more routes

    let mut can_skip_all = true;
    let mut rest_routes: Vec<Routes> = vec![];

    loop {
        match parse_routes(tokens, current_position) {
            None => {
                break;
            }
            Some((routes, next_position)) => {
                current_position = next_position;
                rest_routes.push(routes);
            }
        }

        match tokens.get(current_position) {
            None => {
                return None;
            }
            Some(token) => {
                match token {
                    Tokens::BranchOr => {
                        current_position += 1;
                        can_skip_all = true;
                    }
                    Tokens::ParenClose => {
                        // this is a peek
                        can_skip_all = false;
                        break;
                    }
                    _ => {
                        return None;
                    }
                }
            }
        }
    }

    // expect next token to be a closing ).
    // peek on next token
    match tokens.get(current_position) {
        None => {
            return None;
        }
        Some(token) => {
            match token {
                Tokens::ParenClose => {
                    // this is a peek
                }
                _ => {
                    return None;
                }
            }
        }
    }

    let result = if can_skip_all {
        Branches::CanSkip(starting_routes, rest_routes)
    } else {
        Branches::CannotSkip(starting_routes, rest_routes)
    };

    return Some((result, current_position));
}

fn parse_branch_group(tokens: &Vec<Tokens>, start_at: TokenPosition) -> ParseResult<BranchGroup> {
    let mut current_position = start_at;

    // parse opening parentheses

    match tokens.get(current_position) {
        None => {
            return None;
        }
        Some(token) => {
            if token == &Tokens::ParenOpen {
                current_position += 1;
            } else {
                return None;
            }
        }
    }

    // parse branches
    let branches: Branches = match parse_branches(tokens, current_position) {
        None => {
            return None;
        }
        Some((branches, next_position)) => {
            current_position = next_position;
            branches
        }
    };

    // parse closing parentheses

    match tokens.get(current_position) {
        None => {
            return None;
        }
        Some(token) => {
            if token == &Tokens::ParenClose {
                current_position += 1;
            } else {
                return None;
            }
        }
    }

    let result = (BranchGroup(branches), current_position);
    return Some(result);
}

fn parse_routes(tokens: &Vec<Tokens>, start_at: TokenPosition) -> ParseResult<Routes> {
    match parse_branch_group(tokens, start_at) {
        None => {}
        Some((branch_group, next_position)) => match parse_routes(tokens, next_position) {
            None => {
                let result = Routes::Branch(branch_group, Box::new(None));
                return Some((result, next_position));
            }
            Some((routes, next_position)) => {
                let result = Routes::Branch(branch_group, Box::new(Some(routes)));
                return Some((result, next_position));
            }
        },
    }

    match parse_route(tokens, start_at) {
        None => {
            return None;
        }
        Some((starting_route, next_position)) => match parse_routes(tokens, next_position) {
            None => {
                let result = Routes::Route(starting_route, Box::new(None));
                return Some((result, next_position));
            }
            Some((routes, next_position)) => {
                let result = Routes::Route(starting_route, Box::new(Some(routes)));
                return Some((result, next_position));
            }
        },
    }
}

fn parse_input(input_string: &str) -> Directions {
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

    // parse ending token
    match parse_end(&tokenized, current_position) {
        None => {
            panic!("Expect starting token: $");
        }
        Some((_, _next_position)) => {}
    }

    return Directions(routes);
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
    fn to_string(&self) -> String {
        let result = match self {
            OpenDirections::North => "N",
            OpenDirections::South => "S",
            OpenDirections::West => "W",
            OpenDirections::East => "E",
        };

        return result.to_string();
    }

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

    fn parse_directions(directions: Directions) {}
}

fn main() {
    let input_string = include_str!("input.txt");

    // let input_string = "^(SWSSE(N|ES(ENSW|)WWW(NNNWESSS|)|)|)$";
    // let map = Map::new();

    // map.parse_input(input_string);

    // let input_string = "^N(E|W)N$";

    let directions = parse_input(input_string);

    // println!("{:?}", result);

    assert_eq!(directions.to_string(), input_string);

    println!("{}", directions.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input_string = "^WNE$";
        let parse_result = parse_input(input_string);
        assert_eq!(parse_result.to_string(), input_string);

        let input_string = "^ENWWW(NEEE|SSE(EE|N))$";
        let parse_result = parse_input(input_string);
        assert_eq!(parse_result.to_string(), input_string);

        let input_string = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";
        let parse_result = parse_input(input_string);
        assert_eq!(parse_result.to_string(), input_string);

        let input_string = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$";
        let parse_result = parse_input(input_string);
        assert_eq!(parse_result.to_string(), input_string);

        let input_string = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";
        let parse_result = parse_input(input_string);
        assert_eq!(parse_result.to_string(), input_string);
    }

}
