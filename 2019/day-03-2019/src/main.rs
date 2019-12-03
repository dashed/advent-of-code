// https://adventofcode.com/2019/day/3

type Coordinate = (i32, i32);
// a line segment is defined by two coordinates
type LineSegment = (Coordinate, Coordinate);

// https://math.stackexchange.com/a/139604/10247
type Distance = i32;
fn get_manhattan_distance(start: Coordinate, end: Coordinate) -> Distance {
    let (a, b) = start;
    let (c, d) = end;

    return (a - c).abs() + (b - d).abs();
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

    if (0.0 <= parameter_1 && parameter_1 <= 1.0) && (0.0 <= parameter_2 && parameter_2 <= 1.0) {
        let x = x_1 as f64 + parameter_1 * (x_2 as f64 - x_1 as f64);
        let y = y_1 as f64 + parameter_1 * (y_2 as f64 - y_1 as f64);

        return Some((x as i32, y as i32));
    }

    return None;
}

fn main() {
    let input_string = include_str!("input.txt");

    // Part 1

    // let inputs: Vec<&str> = input_string.trim().split_whitespace().collect();

    println!("{:?}", input_string);
}

#[cfg(test)]
mod tests {
    use super::*;

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
