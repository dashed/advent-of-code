// https://adventofcode.com/2023/day/1

fn main() {
    let input_string = include_str!("input.txt");

    // Part 1

    println!("Part 1: {}", part_1(input_string));

    // Part 2

    let result = part_2(input_string);
    println!("Part 2: {}", result);
    assert_eq!(result, 54019);
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

fn parse_number_word(chars: &[char]) -> Option<(i32, i32)> {
    let number_words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for (index, number_word) in number_words.into_iter().enumerate() {
        let number_word_length = number_word.len();
        let number_word = number_word.chars().collect::<Vec<char>>();
        if chars.starts_with(&number_word) {
            let number = index + 1;
            return Some((number as i32, number_word_length as i32));
        }
    }

    None
}

fn part_2(input_string: &str) -> i32 {
    let inputs: Vec<&str> = input_string.trim().lines().collect();
    let mut numbers: Vec<i32> = vec![];

    for input in inputs {
        let mut chars: Vec<char> = input.chars().collect();

        let mut digits: Vec<i32> = vec![];

        loop {
            if chars.is_empty() {
                break;
            }

            let first_char = chars.first().unwrap();
            if first_char.is_ascii_digit() {
                let number = first_char.to_digit(10).unwrap() as i32;
                digits.push(number);
                chars.remove(0);
                continue;
            }

            let result = parse_number_word(&chars);
            if result.is_none() {
                chars.remove(0);
                continue;
            }

            let (number, number_word_length) = result.unwrap();
            digits.push(number);
            // chars.drain(0..number_word_length as usize);
            // Preserve the last character of the number word to cover cases such as eighthree and sevenine.
            // The instructions doesn't make this clear.
            chars.drain(0..(number_word_length - 1) as usize);
        }

        if digits.is_empty() {
            continue;
        }

        let first_calibration_value = digits.first().unwrap();
        let second_calibration_value = digits.last().unwrap();
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

        assert_eq!(part_1(input_string), 142);

        let input_string = r###"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"###;

        assert_eq!(part_2(input_string), 281);

        let input_string = r###"
eighthree
sevenine
"###;

        assert_eq!(part_2(input_string), 83 + 79);
    }
}
