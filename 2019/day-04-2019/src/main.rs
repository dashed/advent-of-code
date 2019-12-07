// https://adventofcode.com/2019/day/4

use std::collections::HashMap;

use rayon::prelude::*;

fn is_valid_password(password: String) -> bool {
    if password.len() != 6 {
        return false;
    }

    let chars: Vec<u8> = password
        .chars()
        .map(|digit| -> u8 {
            return digit.to_digit(10).unwrap() as u8;
        })
        .collect();

    let mut equal_adjacent_digits = false;
    let mut prev_digit: Option<u8> = None;

    for digit in chars {
        match prev_digit {
            None => {
                prev_digit = Some(digit);
                continue;
            }
            Some(last_digit) => {
                // Two adjacent digits are the same
                if !equal_adjacent_digits {
                    if digit == last_digit {
                        equal_adjacent_digits = true;
                    }
                }

                // println!("{} <= {} {}", last_digit, digit, digit >= last_digit);

                if digit >= last_digit {
                    prev_digit = Some(digit);
                    continue;
                }

                return false;
            }
        }
    }

    if !equal_adjacent_digits {
        return false;
    }

    true
}

fn is_valid_password_part_2(password: String) -> bool {
    if password.len() != 6 {
        return false;
    }

    let chars: Vec<u8> = password
        .chars()
        .map(|digit| -> u8 {
            return digit.to_digit(10).unwrap() as u8;
        })
        .collect();

    // the two adjacent matching digits are not part of a larger group of matching digits.
    let mut digit_counter: HashMap<u8, u32> = HashMap::new();
    let mut prev_digit: Option<u8> = None;

    for digit in chars {
        match prev_digit {
            None => {

                let entry = digit_counter.entry(digit).or_insert(0);
                *entry += 1;

                prev_digit = Some(digit);
                continue;
            }
            Some(last_digit) => {

                // if !digit_counter.contains_key(&digit) {
                //     digit_counter.insert(digit, 1);
                // } else {
                //     let count = digit_counter.get(&digit).unwrap();
                //     digit_counter.insert(digit, count + 1);
                // }

                let entry = digit_counter.entry(digit).or_insert(0);
                *entry += 1;

                if digit >= last_digit {
                    prev_digit = Some(digit);
                    continue;
                }

                return false;
            }
        }
    }

    for (_digit, count) in &digit_counter {
        if *count == 2 {
            return true;
        }
    }

    return false;
}

fn main() {
    // Part 1

    let num_of_valid_passwords: u32 = (193651..=649729)
        .into_par_iter()
        .map(|current_pass: i32| -> u32 {
            if is_valid_password(format!("{}", current_pass)) {
                return 1;
            }
            return 0;
        })
        .sum();

    // 1605
    println!("Part 1: {}", num_of_valid_passwords);

    let num_of_valid_passwords_part_2: u32 = (193651..=649729)
        .into_par_iter()
        .map(|current_pass: i32| -> u32 {
            if is_valid_password_part_2(format!("{}", current_pass)) {
                return 1;
            }
            return 0;
        })
        .sum();

    println!("Part 2: {}", num_of_valid_passwords_part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_password() {
        assert_eq!(is_valid_password("111111".to_string()), true);
        assert_eq!(is_valid_password("122345".to_string()), true);
        assert_eq!(is_valid_password("111123".to_string()), true);
        assert_eq!(is_valid_password("135679".to_string()), false);

        assert_eq!(is_valid_password("223450".to_string()), false);
        assert_eq!(is_valid_password("123789".to_string()), false);
    }

    #[test]
    fn test_is_valid_password_part_2() {
        assert_eq!(is_valid_password_part_2("111111".to_string()), false);
        assert_eq!(is_valid_password_part_2("122345".to_string()), true);
        assert_eq!(is_valid_password_part_2("111123".to_string()), false);
        assert_eq!(is_valid_password_part_2("135679".to_string()), false);

        assert_eq!(is_valid_password_part_2("223450".to_string()), false);
        assert_eq!(is_valid_password_part_2("123789".to_string()), false);
        assert_eq!(is_valid_password_part_2("112233".to_string()), true);
        assert_eq!(is_valid_password_part_2("123444".to_string()), false);
        assert_eq!(is_valid_password_part_2("111122".to_string()), true);
        assert_eq!(is_valid_password_part_2("112345".to_string()), true);
    }
}
