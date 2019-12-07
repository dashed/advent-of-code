// https://adventofcode.com/2019/day/4

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
}
