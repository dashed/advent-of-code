// https://adventofcode.com/2018/day/3

// imports

use std::cmp;
use std::collections::HashSet;
use std::str::Lines;

// part 1

#[derive(Debug, PartialEq, Clone)]
struct Fabric {
    id: String,

    // starting coordinates
    left: i32,
    top: i32,

    // size
    height: i32,
    width: i32,
}

impl Fabric {

    fn right(&self) -> i32 {
        return self.left + self.width;
    }

    fn bottom(&self) -> i32 {
        return self.top + self.height;
    }

    fn generate_claim_points(&self) -> HashSet<(i32, i32)> {
        // NOTE: I don't like this :(

        let mut points: HashSet<(i32, i32)> = HashSet::new();

        for x in self.left..self.right() {
            for y in self.top..self.bottom() {
                points.insert((x, y));
            }
        }

        return points;
    }

    fn is_overlapping(&self, other: &Fabric) -> bool {
        // determines if self is on the left side of other, and not overlapping
        let self_left_of_other = self.right() < other.left;
        // determines if self is on the right side of other, and not overlapping
        let self_right_of_other = self.left > other.right();
        // determines if self is above other, and not overlapping
        let self_above_of_other = self.bottom() < other.top;
        // determines if self is below other, and not overlapping
        let self_below_of_other = self.top > other.bottom();

        // self does not overlap other if any of the above conditions is true
        let not_overlapping =
            self_left_of_other || self_right_of_other || self_above_of_other || self_below_of_other;

        return !not_overlapping;
    }

    fn generate_intersection_fabric(&self, other: &Fabric) -> Option<Fabric> {
        if !self.is_overlapping(other) {
            return None;
        }

        let left = cmp::max(self.left, other.left);
        let top = cmp::max(self.top, other.top);

        let overlapping_width = cmp::min(self.right(), other.right()) - left;
        let overlapping_height = cmp::min(self.bottom(), other.bottom()) - top;

        let area = overlapping_width * overlapping_height;

        if area <= 0 {
            return None;
        }

        assert!(overlapping_width > 0);
        assert!(overlapping_height > 0);

        let intersection_fabric = Fabric {
            id: format!("Insection of: {} and {}", self.id, other.id),
            left: left,
            top: top,
            height: overlapping_height,
            width: overlapping_width,
        };

        Some(intersection_fabric)
    }
}

fn parse_to_fabric(input: &str) -> Fabric {
    let mut parts = input.split_whitespace();

    let id = parts.next().unwrap();

    let (left, top): (i32, i32) = {
        let location_string = parts.nth(1).unwrap().to_string();

        let location_string: String = location_string
            .chars()
            // ignore the last charcter which is expected to be a colon :
            .take(location_string.len() - 1)
            .collect();

        let locations: Vec<i32> = location_string
            .split(',')
            .map(|x| -> i32 { return x.parse().unwrap() })
            .collect();

        (*locations.get(0).unwrap(), *locations.get(1).unwrap())
    };

    let (height, width): (i32, i32) = {
        let size_string = parts.next().unwrap().to_string();

        let sizes: Vec<i32> = size_string
            .split('x')
            .map(|x| -> i32 { return x.parse().unwrap() })
            .collect();

        (*sizes.get(0).unwrap(), *sizes.get(1).unwrap())
    };

    Fabric {
        id: id.to_string(),
        left: left,
        top: top,
        height: height,
        width: width,
    }
}

fn part_1(inputs: Lines) {
    let fabrics: Vec<Fabric> = inputs.map(|x| parse_to_fabric(x)).collect();

    let mut known_intersection_points: HashSet<(i32, i32)> = HashSet::new();

    for fabric in fabrics.clone() {
        for other_fabric in fabrics.clone() {

            if fabric == other_fabric {
                continue;
            }

            let intersection_fabric = fabric.generate_intersection_fabric(&other_fabric);

            if intersection_fabric.is_some() {
                let intersection_fabric = intersection_fabric.unwrap();
                let claimed_points = intersection_fabric.generate_claim_points();
                known_intersection_points.extend(&claimed_points);
            }
        }
    }

    // Not: 91586
    println!("Overlapping area: {:?}", known_intersection_points.len());
}

fn main() {
    let input_string = include_str!("input.txt");

    let inputs = input_string.lines();

    part_1(inputs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_to_fabric() {
        let expected = Fabric {
            id: "#123".to_string(),
            left: 3,
            top: 2,
            height: 5,
            width: 4,
        };

        assert_eq!(parse_to_fabric("#123 @ 3,2: 5x4"), expected);
    }

    #[test]
    fn test_overlap() {

        fn get_overlapping_area(this: &Fabric, other: &Fabric) -> i32 {
            let intersection_fabric = this.generate_intersection_fabric(&other);

            if intersection_fabric.is_some() {
                let intersection_fabric = intersection_fabric.unwrap();
                println!("{:?}", intersection_fabric);
                let claimed_points = intersection_fabric.generate_claim_points();

                println!("{:?}", claimed_points);

                return claimed_points.len() as i32;
            }

            return 0;
        }


        let fabric_1 = parse_to_fabric("#1 @ 1,3: 4x4");
        let fabric_2 = parse_to_fabric("#2 @ 3,1: 4x4");
        let fabric_3 = parse_to_fabric("#3 @ 5,5: 2x2");

        assert_eq!(get_overlapping_area(&fabric_1, &fabric_2), 4);
        assert_eq!(get_overlapping_area(&fabric_1, &fabric_3), 0);
        assert_eq!(get_overlapping_area(&fabric_2, &fabric_3), 0);
    }

}
