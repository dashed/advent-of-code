fn main() {
    let input_string = include_str!("input.txt");

    // Part 1

    println!("Part 1: {}", part_1(input_string));
}

fn part_1(input_string: &str) -> i32 {
    let inputs: Vec<&str> = input_string.trim().lines().collect();

    let mut numbers: Vec<i32> = vec![];

    for input in inputs {
        let digits: String = input.chars().filter(|c| c.is_ascii_digit()).collect();
        let digits: Vec<char> = digits.chars().collect();

        if digits.is_empty() {
            continue;
        }
        let first_calibration_value = digits.first().unwrap().to_digit(10).unwrap() as i32;
        let second_calibration_value = digits.last().unwrap().to_digit(10).unwrap() as i32;
        let calibration_value = format!("{}{}", first_calibration_value, second_calibration_value);
        let number = calibration_value.parse::<i32>().unwrap();
        numbers.push(number);
    }

    numbers.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1() {
        let input_string = r###"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"###;

        assert_eq!(part_1(input_string.clone()), 142);
    }
}
