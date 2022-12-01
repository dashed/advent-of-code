// https://adventofcode.com/2019/day/3

type Coordinate = (i32, i32);

// a line segment is defined by two coordinates
type LineSegment = (Coordinate, Coordinate);

// https://math.stackexchange.com/a/139604/10247
type Distance = i32;
fn get_manhattan_distance(start: Coordinate, end: Coordinate) -> Distance {
    let (a, b) = start;
    let (c, d) = end;

    (a - c).abs() + (b - d).abs()
}

// based on http://www.cs.swan.ac.uk/~cssimon/line_intersection.html
fn line_segments_intersection(
    first_segment: LineSegment,
    second_segment: LineSegment,
) -> Option<Coordinate> {
    let (point_1, point_2) = first_segment;
    let (point_3, point_4) = second_segment;

    let (x_1, y_1) = point_1;
    let (x_2, y_2) = point_2;
    let (x_3, y_3) = point_3;
    let (x_4, y_4) = point_4;

    let parameter_1_numerator = (y_3 - y_4) * (x_1 - x_3) + (x_4 - x_3) * (y_1 - y_3);
    let parameter_1_denominator = (x_4 - x_3) * (y_1 - y_2) - (x_1 - x_2) * (y_4 - y_3);

    let parameter_2_numerator = (y_1 - y_2) * (x_1 - x_3) + (x_2 - x_1) * (y_1 - y_3);
    let parameter_2_denominator = (x_4 - x_3) * (y_1 - y_2) - (x_1 - x_2) * (y_4 - y_3);

    if parameter_1_denominator == 0 || parameter_2_denominator == 0 {
        return None;
    }

    let parameter_1: f64 = parameter_1_numerator as f64 / parameter_1_denominator as f64;
    let parameter_2: f64 = parameter_2_numerator as f64 / parameter_2_denominator as f64;

    if (0.0..=1.0).contains(&parameter_1) && (0.0..=1.0).contains(&parameter_2) {
        let x = x_1 as f64 + parameter_1 * (x_2 as f64 - x_1 as f64);
        let y = y_1 as f64 + parameter_1 * (y_2 as f64 - y_1 as f64);

        return Some((x as i32, y as i32));
    }

    None
}

fn process_wires(input_string: String) -> Vec<Vec<LineSegment>> {
    let inputs: Vec<&str> = input_string.split_whitespace().collect();

    let wires: Vec<Vec<LineSegment>> = inputs
        .into_iter()
        .map(|wire: &str| {
            let mut current_coord: Coordinate = (0, 0);

            let line_segments: Vec<LineSegment> = wire
                .trim()
                .split(',')
                .map(|instructions: &str| {
                    let instructions = instructions.trim();

                    let direction: char = instructions.chars().next().unwrap();
                    let steps: String = instructions.chars().skip(1).collect();
                    let steps: u32 = steps.parse().unwrap();

                    let previous_coord = current_coord;

                    match direction {
                        'U' => {
                            let (x, y) = current_coord;
                            current_coord = (x, y + (steps as i32));
                        }
                        'D' => {
                            let (x, y) = current_coord;
                            current_coord = (x, y - (steps as i32));
                        }
                        'L' => {
                            let (x, y) = current_coord;
                            current_coord = (x - (steps as i32), y);
                        }
                        'R' => {
                            let (x, y) = current_coord;
                            current_coord = (x + (steps as i32), y);
                        }
                        _ => {
                            panic!("Unknown direction: {}", direction);
                        }
                    }

                    assert_eq!(
                        get_manhattan_distance(previous_coord, current_coord),
                        steps as i32
                    );

                    let line_segment: LineSegment = (previous_coord, current_coord);

                    line_segment
                })
                .collect();

            line_segments
        })
        .collect();

    wires
}

fn part_1(input_string: String) -> Distance {
    let wires: Vec<Vec<LineSegment>> = process_wires(input_string);
    assert!(wires.len() >= 2);
    let wire_1: Vec<LineSegment> = wires[0].clone();
    let wire_2: Vec<LineSegment> = wires[1].clone();

    let mut intersections: Vec<Coordinate> = vec![];

    for segment_1 in wire_1 {
        for segment_2 in wire_2.iter() {
            match line_segments_intersection(segment_1, *segment_2) {
                None => {
                    continue;
                }
                Some(coord) => {
                    if coord == (0, 0) {
                        continue;
                    }

                    intersections.push(coord);
                }
            }
        }
    }

    let closest_intersection_to_port: Distance = intersections
        .into_iter()
        .map(|coord| {
            get_manhattan_distance((0, 0), coord)
        })
        .min()
        .unwrap();

    closest_intersection_to_port
}

fn part_2(input_string: String) -> i32 {
    let wires: Vec<Vec<LineSegment>> = process_wires(input_string);
    assert!(wires.len() >= 2);
    let wire_1: Vec<LineSegment> = wires[0].clone();
    let wire_2: Vec<LineSegment> = wires[1].clone();

    let mut steps_to_reach_intersections: Vec<i32> = vec![];

    let mut steps_wire_1 = 0;

    for segment_1 in wire_1 {
        let (segment_1_start, segment_1_end) = segment_1;
        steps_wire_1 += get_manhattan_distance(segment_1_start, segment_1_end);

        let mut steps_wire_2 = 0;

        for segment_2 in wire_2.iter() {
            let (segment_2_start, segment_2_end) = segment_2;
            steps_wire_2 += get_manhattan_distance(*segment_2_start, *segment_2_end);

            match line_segments_intersection(segment_1, *segment_2) {
                None => {
                    continue;
                }
                Some(intersection_coord) => {
                    if intersection_coord == (0, 0) {
                        continue;
                    }

                    // need to backtrack the amount of steps

                    let steps_wire_1_intersection =
                        steps_wire_1 - get_manhattan_distance(intersection_coord, segment_1_end);
                    let steps_wire_2_intersection =
                        steps_wire_2 - get_manhattan_distance(intersection_coord, *segment_2_end);

                    steps_to_reach_intersections
                        .push(steps_wire_1_intersection + steps_wire_2_intersection);
                }
            }
        }
    }

    let fewest_combined_steps: i32 = steps_to_reach_intersections.into_iter().min().unwrap();

    fewest_combined_steps
}

fn main() {
    let input_string = include_str!("input.txt");

    println!("Part 1: {}", part_1(input_string.to_string()));
    println!("Part 2: {}", part_2(input_string.to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input_string = r###"
R8,U5,L5,D3
U7,R6,D4,L4
        "###;

        assert_eq!(part_1(input_string.to_string()), 6);

        let input_string = r###"
R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83
        "###;

        assert_eq!(part_1(input_string.to_string()), 159);

        let input_string = r###"
R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7
        "###;

        assert_eq!(part_1(input_string.to_string()), 135);

        let input_string = include_str!("input.txt");
        assert_eq!(part_1(input_string.to_string()), 1519);
    }

    #[test]
    fn test_part_2() {
        let input_string = r###"
R8,U5,L5,D3
U7,R6,D4,L4
        "###;

        assert_eq!(part_2(input_string.to_string()), 30);

        let input_string = r###"
R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83
        "###;

        assert_eq!(part_2(input_string.to_string()), 610);

        let input_string = r###"
R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7
        "###;

        assert_eq!(part_2(input_string.to_string()), 410);

        let input_string = include_str!("input.txt");
        assert_eq!(part_2(input_string.to_string()), 14358);
    }

    #[test]
    fn test_line_segments_intersection() {
        // intersection

        let line_segment_1 = ((3, 2), (3, 10));
        let line_segment_2 = ((0, 3), (10, 3));

        assert_eq!(
            line_segments_intersection(line_segment_1, line_segment_2),
            Some((3, 3))
        );

        // no intersection

        let line_segment_1 = ((3, 2), (3, 10));
        let line_segment_2 = ((0, 30), (10, 30));

        assert_eq!(
            line_segments_intersection(line_segment_1, line_segment_2),
            None
        );

        // collinear intersection y-axis

        let line_segment_1 = ((3, 2), (3, 10));
        let line_segment_2 = ((3, -10), (3, 20));

        assert_eq!(
            line_segments_intersection(line_segment_1, line_segment_2),
            None
        );

        // collinear intersection x-axis

        let line_segment_1 = ((-10, 3), (20, 3));
        let line_segment_2 = ((0, 3), (10, 3));

        assert_eq!(
            line_segments_intersection(line_segment_1, line_segment_2),
            None
        );
    }
}
