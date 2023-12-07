use core::panic;

fn quadratic_formula(a: f64, b: f64, c: f64) -> (f64, f64) {
    // ax^2 + bx + c = 0
    let discriminant = b.powi(2) - 4.0 * a * c;
    if discriminant < 0.0 {
        panic!("No real solutions");
    }
    let sqrt_discriminant = discriminant.sqrt();
    let x1 = (-b + sqrt_discriminant) / (2.0 * a);
    let x2 = (-b - sqrt_discriminant) / (2.0 * a);
    (x1, x2)
}

#[derive(Debug)]
struct Race {
    // For each race, how far can the toy boat travel within time_allowed?
    // milliseconds
    time_allowed: i64,
    // millimeters
    best_distance: i64,
}

impl Race {
    fn ways_to_win(&self) -> i64 {
        // Your toy boat has a starting speed of zero millimeters per millisecond. For each whole millisecond you
        // spend at the beginning of the race holding down the button, the boat's speed increases by
        // one millimeter per millisecond.

        //  Scratch work:
        // let charge_speed = 1;
        // let charge_time = charge_speed;
        // let remaining_time = self.time_allowed - charge_time;
        // let distance_travelled = charge_speed * remaining_time;

        // Assume charge_speed is a positive integer.
        // charge_time = charge_speed
        // remaining_time = self.time_allowed - charge_time;
        //
        // distance_travelled > self.best_distance
        // charge_speed * remaining_time > self.best_distance
        // charge_speed * (self.time_allowed - charge_time) > self.best_distance
        // charge_time * (self.time_allowed - charge_time) > self.best_distance
        // charge_time * self.time_allowed - charge_time^2 > self.best_distance
        // c * t - c^2 > d
        // Get solutions for: charge_time * self.time_allowed - charge_time^2 - self.best_distance = 0
        // There should be two zeros, and we want the number of integers between these two zeros.
        // The number of integers between two zeros is the number of ways to win.

        // Assume ax^2 + bx + c = 0
        // -c^2 + t * c - d = 0
        // -charge_time^2 + self.time_allowed * charge_time  - self.best_distance = 0
        let (left, right) =
            quadratic_formula(-1.0, self.time_allowed as f64, -self.best_distance as f64);
        let charge_time_minimum = left.floor() as i64 + 1;
        let charge_time_maximum = right.ceil() as i64 - 1;
        assert!(charge_time_minimum <= charge_time_maximum);
        assert!(charge_time_minimum > 0);

        charge_time_maximum - charge_time_minimum + 1
    }
}

fn part_1(input_string: &str) -> i64 {
    let inputs: Vec<&str> = input_string.trim().lines().collect();

    let mut time_allowed_vec: Vec<i64> = Vec::new();
    let mut best_distance_vec: Vec<i64> = Vec::new();

    for input in inputs.into_iter() {
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input.starts_with("Time:") {
            time_allowed_vec = input
                .trim_start_matches("Time:")
                .trim()
                .split(' ')
                .map(|x| x.trim())
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i64>().unwrap())
                .collect();

            continue;
        }

        if input.starts_with("Distance:") {
            best_distance_vec = input
                .trim_start_matches("Distance:")
                .trim()
                .split(' ')
                .map(|x| x.trim())
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i64>().unwrap())
                .collect();

            println!("{:?}", input);
            continue;
        }
        panic!("Unknown input: {}", input);
    }

    assert!(time_allowed_vec.len() == best_distance_vec.len());

    println!("time_allowed_vec: {:?}", time_allowed_vec);
    println!("best_distance_vec: {:?}", best_distance_vec);

    let races: Vec<Race> = time_allowed_vec
        .into_iter()
        .enumerate()
        .map(|(index, x)| Race {
            time_allowed: x,
            best_distance: best_distance_vec[index],
        })
        .collect();

    races.into_iter().map(|x| x.ways_to_win()).product::<i64>()
}

fn part_2(input_string: &str) -> i64 {
    let inputs: Vec<&str> = input_string.trim().lines().collect();

    let mut time_allowed_vec: Vec<i64> = Vec::new();
    let mut best_distance_vec: Vec<i64> = Vec::new();

    for input in inputs.into_iter() {
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input.starts_with("Time:") {
            time_allowed_vec = input
                .trim_start_matches("Time:")
                .trim()
                .split(' ')
                .map(|x| x.trim())
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i64>().unwrap())
                .collect();

            continue;
        }

        if input.starts_with("Distance:") {
            best_distance_vec = input
                .trim_start_matches("Distance:")
                .trim()
                .split(' ')
                .map(|x| x.trim())
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i64>().unwrap())
                .collect();

            println!("{:?}", input);
            continue;
        }
        panic!("Unknown input: {}", input);
    }

    let time_allowed: String = time_allowed_vec
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");
    let best_distance: String = best_distance_vec
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");

    let race = Race {
        time_allowed: time_allowed.parse::<i64>().unwrap(),
        best_distance: best_distance.parse::<i64>().unwrap(),
    };
    race.ways_to_win()
}

fn main() {
    let input_string = include_str!("input.txt");

    // Part 1

    let answer = part_1(input_string);
    println!("Part 1: {}", answer);
    assert_eq!(answer, 2065338);

    // Part 2

    let answer = part_2(input_string);
    println!("Part 2: {}", answer);
    assert_eq!(answer, 34934171);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle() {
        let input_string = r###"
Time:      7  15   30
Distance:  9  40  200
"###;

        assert_eq!(part_1(input_string), 288);
        assert_eq!(part_2(input_string), 71503);
    }
}
