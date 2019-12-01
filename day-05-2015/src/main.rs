// https://adventofcode.com/2015/day/5

fn is_vowel(c: char) -> bool {
    // c is one of a, e, i, o, or u
    return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u';
}

fn is_nice(input: String) -> bool {
    let chars: Vec<char> = input.chars().collect();

    // a string is nice if it contains at least 3 vowels

    let mut num_of_vowels = 0;

    for c in chars.clone() {
        if is_vowel(c) {
            num_of_vowels = num_of_vowels + 1;
        }

        if num_of_vowels >= 3 {
            break;
        }
    }

    if num_of_vowels < 3 {
        return false;
    }

    // invariant: input.len() >= 3

    // a string is nice if it contains at least one letter that appears twice in a row

    let mut contains_letter_appearing_twice = false;

    for index in 0..=(input.len() - 2) {
        let first_letter: char = chars[index];
        let second_letter: char = chars[index + 1];

        if first_letter == second_letter {
            contains_letter_appearing_twice = true;
            break;
        }
    }

    if !contains_letter_appearing_twice {
        return false;
    }

    // a string is not nice if it contains any of these bad strings

    let bad_strings = vec!["ab", "cd", "pq", "xy"];
    for bad_string in bad_strings {
        if input.contains(bad_string) {
            return false;
        }
    }

    return true;
}

fn part_1(input_string: String) -> usize {
    let inputs: Vec<&str> = input_string.trim().split_whitespace().collect();

    let nice_strings: Vec<&str> = inputs
        .into_iter()
        .filter(|input| {
            return is_nice(input.to_string());
        })
        .collect();

    return nice_strings.len();
}

fn main() {
    let input_string = include_str!("input.txt");

    println!("Part 1: {}", part_1(input_string.to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_nice() {
        assert_eq!(is_nice("ugknbfddgicrmopn".to_string()), true);
        assert_eq!(is_nice("aaa".to_string()), true);
        assert_eq!(is_nice("jchzalrnumimnmhp".to_string()), false);
        assert_eq!(is_nice("haegwjzuvuyypxyu".to_string()), false);
        assert_eq!(is_nice("dvszwmarrgswjxmb".to_string()), false);
    }
}
