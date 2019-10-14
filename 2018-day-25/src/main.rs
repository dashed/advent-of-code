// https://adventofcode.com/2018/day/25

// imports

use std::collections::HashSet;

// code

// adapted from day 6
// https://math.stackexchange.com/a/139604/10247
type Distance = i32;
fn get_manhattan_distance(start: Coordinate, end: Coordinate) -> Distance {
    let (a, b, e, g) = start;
    let (c, d, f, h) = end;

    return (a - c).abs() + (b - d).abs() + (e - f).abs() + (g - h).abs();
}

type Coordinate = (i32, i32, i32, i32);

#[derive(Debug, Clone)]
struct Constellation {
    points: HashSet<Coordinate>,
}

impl Constellation {
    fn new(point: Coordinate) -> Self {
        let mut points = HashSet::new();
        points.insert(point);

        return Constellation { points };
    }

    fn merge(self, other: Self) -> Self {
        let mut new_points = self.points.clone();
        new_points.extend(other.points);

        return Constellation { points: new_points };
    }

    fn can_merge(&mut self, other: &Self) -> bool {
        for current_point in &self.points {
            if other.can_add(*current_point) {
                return true;
            }
        }

        return false;
    }

    fn can_add(&self, new_point: Coordinate) -> bool {
        for point in &self.points {
            assert!(new_point != *point);
            if get_manhattan_distance(*point, new_point) <= 3 {
                return true;
            }
        }
        return false;
    }

    fn add(&mut self, point: Coordinate) {
        self.points.insert(point);
    }
}

fn parse_input(input_string: &str) -> Vec<Coordinate> {
    let mut output: Vec<Coordinate> = vec![];

    for input in input_string.trim().lines() {
        let input = input.trim();

        let coords: Vec<i32> = input
            .split(",")
            .map(|x| x.trim())
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        output.push((coords[0], coords[1], coords[2], coords[3]));
    }

    return output;
}

fn part_1(input_string: &str) -> usize {
    let mut constellations: Vec<Constellation> = vec![];
    let points = parse_input(input_string);

    'points_loop: for current_point in points {
        // find a constellation to join
        for constellation in constellations.iter_mut() {
            if constellation.can_add(current_point) {
                constellation.add(current_point);
                continue 'points_loop;
            }
        }

        // invariant: no constellation found
        let constellation = Constellation::new(current_point);
        constellations.push(constellation);
    }

    // merge constallations

    loop {
        let num_of_constellations_before = constellations.len();

        constellations = constellations
            .into_iter()
            .fold(vec![], |mut acc, mut constellation| {
                // find a constellation to join
                for potential_constellation in acc.iter_mut() {
                    if constellation.can_merge(&potential_constellation) {
                        let new_constellation =
                            constellation.merge(potential_constellation.clone());
                        *potential_constellation = new_constellation;
                        return acc;
                    }
                }

                // invariant: no constellation found
                acc.push(constellation);

                return acc;
            });

        let num_of_constellations_after = constellations.len();

        if num_of_constellations_before == num_of_constellations_after {
            break;
        }
    }

    return constellations.len();
}

fn main() {
    let input_string = include_str!("input.txt");

    println!("Part 1: {}", part_1(input_string));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("input.txt")), 352);

        let input_string = r####"
 0,0,0,0
 3,0,0,0
 0,3,0,0
 0,0,3,0
 0,0,0,3
 0,0,0,6
 9,0,0,0
12,0,0,0
        "####;

        assert_eq!(part_1(input_string), 2);

        let input_string = r####"
-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0
        "####;

        assert_eq!(part_1(input_string), 4);

        let input_string = r####"
1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2
        "####;

        assert_eq!(part_1(input_string), 3);

        let input_string = r####"
1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2
        "####;

        assert_eq!(part_1(input_string), 8);
    }
}
