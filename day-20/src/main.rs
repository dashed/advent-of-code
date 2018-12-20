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
    #[allow(dead_code)]
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

type Distance = usize;

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
    // the fewest number of doors to pass through to reach room defined by Coordinate
    room_distance: HashMap<Coordinate, Distance>,
}

// invariant: the routes will take you through every door in the facility at least once

impl Map {
    fn new() -> Self {
        Map {
            room_distance: HashMap::new(),
        }
    }

    fn distance_to_farthest_room(&self) -> Distance {
        return *self.room_distance.values().into_iter().max().unwrap();
    }

    fn visit_room(
        &mut self,
        direction: OpenDirections,
        current_position: Coordinate,
    ) -> Coordinate {
        // generate new position

        let new_position = match direction {
            OpenDirections::North => current_position.north(),
            OpenDirections::South => current_position.south(),
            OpenDirections::West => current_position.west(),
            OpenDirections::East => current_position.east(),
        };

        let current_distance = self.room_distance.get(&current_position).unwrap();
        let new_distance = current_distance + 1;

        // is this room reached by the least amount of doors?

        match self.room_distance.get(&new_position) {
            None => {
                self.room_distance.insert(new_position, new_distance);
            }
            Some(best_distance) => {
                if new_distance < *best_distance {
                    self.room_distance.insert(new_position, new_distance);
                }
            }
        }

        return new_position;
    }

    fn parse_route(&mut self, route: Route, current_position: Coordinate) -> Coordinate {
        let Route(directions) = route;

        if directions.len() <= 0 {
            return current_position;
        }

        // generate new position

        let mut new_position = current_position;

        for direction in directions {
            new_position = self.visit_room(direction, new_position);
        }

        return new_position;
    }

    fn generate_positions(
        &mut self,
        choices: Vec<Routes>,
        current_position: Coordinate,
    ) -> HashSet<Coordinate> {
        // generate new positions for every choice taken

        let new_positions: HashSet<Coordinate> =
            choices
                .into_iter()
                .fold(HashSet::new(), |mut acc, routes_choice| {
                    let new_positions = self.parse_routes(routes_choice, current_position);
                    acc.extend(new_positions);
                    return acc;
                });

        return new_positions;
    }

    fn parse_branch_group(
        &mut self,
        branch_group: BranchGroup,
        more_routes: Option<Routes>,
        current_position: Coordinate,
    ) -> HashSet<Coordinate> {
        let BranchGroup(branches) = branch_group;

        let new_positions: HashSet<Coordinate> = match branches {
            Branches::CanSkip(first_choice, other_choices) => {
                let mut choices = vec![*first_choice];
                choices.extend(other_choices);
                let mut new_positions = self.generate_positions(choices, current_position);

                // all of these choices could be skipped, so current_position can be a position to start from
                new_positions.insert(current_position);

                new_positions
            }
            Branches::CannotSkip(first_choice, other_choices) => {
                let mut choices = vec![*first_choice];
                choices.extend(other_choices);

                self.generate_positions(choices, current_position)
            }
        };

        // for each new positions, continue taking more_routes

        match more_routes {
            None => {
                return new_positions;
            }
            Some(more_routes) => {
                return new_positions.into_iter().fold(
                    HashSet::new(),
                    |mut acc, position: Coordinate| {
                        let new_positions = self.parse_routes(more_routes.clone(), position);
                        acc.extend(new_positions);
                        return acc;
                    },
                );
            }
        }
    }

    fn parse_routes(
        &mut self,
        routes: Routes,
        current_position: Coordinate,
    ) -> HashSet<Coordinate> {
        match routes {
            Routes::Route(route, more_routes) => {
                let new_position = self.parse_route(route, current_position);
                match *more_routes {
                    None => {
                        let mut set = HashSet::new();
                        set.insert(new_position);
                        return set;
                    }
                    Some(more_routes) => {
                        return self.parse_routes(more_routes, new_position);
                    }
                }
            }
            Routes::Branch(branch_group, more_routes) => {
                return self.parse_branch_group(branch_group, *more_routes, current_position);
            }
        }
    }

    fn parse_directions(&mut self, directions: Directions) {
        let Directions(routes) = directions;

        let current_position = (0, 0);
        self.room_distance.insert(current_position, 0);

        self.parse_routes(routes, current_position);
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    let directions = parse_input(input_string);
    let mut map = Map::new();
    map.parse_directions(directions);

    println!("Part 1: {}", map.distance_to_farthest_room());

    // find number of rooms that are reachable by at least 1000 doors
    let num_of_rooms = map
        .room_distance
        .values()
        .into_iter()
        .filter(|x| {
            return *x >= &1000;
        })
        .count();

    println!("Part 2: {}", num_of_rooms);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let examples: Vec<(&str, Distance)> = vec![
            ("^WNE$", 3),
            ("^ENWWW(NEEE|SSE(EE|N))$", 10),
            ("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$", 18),
            ("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$", 23),
            (
                "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$",
                31,
            ),
        ];

        for (input_string, distance) in examples {
            let directions = parse_input(input_string);
            assert_eq!(directions.to_string(), input_string);

            let mut map = Map::new();
            map.parse_directions(directions);

            assert_eq!(map.distance_to_farthest_room(), distance);
        }
    }

}
