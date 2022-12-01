// https://adventofcode.com/2015/day/5

fn is_vowel(c: char) -> bool {
    // c is one of a, e, i, o, or u
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

fn is_nice(input: String) -> bool {
    let chars: Vec<char> = input.chars().collect();

    // a string is nice if it contains at least 3 vowels

    let mut num_of_vowels = 0;

    for c in chars.clone() {
        if is_vowel(c) {
            num_of_vowels += 1;
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

    true
}

fn part_1(input_string: String) -> usize {
    let inputs: Vec<&str> = input_string.split_whitespace().collect();

    let nice_strings: Vec<&str> = inputs
        .into_iter()
        .filter(|input| {
            is_nice(input.to_string())
        })
        .collect();

    nice_strings.len()
}

fn is_nice_part_2(input_string: String) -> bool {
    if input_string.len() < 3 {
        return false;
    }

    let chars: Vec<char> = input_string.chars().collect();

    // It contains at least one letter which repeats with exactly one letter between them
    let mut has_second_rule = false;

    for index in 0..=(input_string.len() - 3) {
        let first_letter: char = chars[index];
        // let second_letter: char = chars[index + 1];
        let third_letter: char = chars[index + 2];

        if first_letter == third_letter {
            has_second_rule = true;
            break;
        }
    }

    if !has_second_rule {
        return false;
    }

    // It contains a pair of any two letters that appears at least twice in the string without overlapping
    let mut has_first_rule = false;

    for index in 0..=(input_string.len() - 2) {
        let first_letter: char = chars[index];
        let second_letter: char = chars[index + 1];

        let pair: String = format!("{}{}", first_letter, second_letter);
        let sub_string: String = input_string.chars().skip(index + 2).collect();

        if sub_string.contains(&pair) {
            has_first_rule = true;
            break;
        }
    }

    has_first_rule
}

fn part_2(input_string: String) -> usize {
    let inputs: Vec<&str> = input_string.split_whitespace().collect();

    let nice_strings: Vec<&str> = inputs
        .into_iter()
        .filter(|input| {
            is_nice_part_2(input.to_string())
        })
        .collect();

    nice_strings.len()
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
    fn test_is_nice() {
        assert_eq!(is_nice("ugknbfddgicrmopn".to_string()), true);
        assert_eq!(is_nice("aaa".to_string()), true);
        assert_eq!(is_nice("jchzalrnumimnmhp".to_string()), false);
        assert_eq!(is_nice("haegwjzuvuyypxyu".to_string()), false);
        assert_eq!(is_nice("dvszwmarrgswjxmb".to_string()), false);
    }

    #[test]
    fn test_is_nice_part_2() {
        assert_eq!(is_nice_part_2("qjhvhtzxzqqjkmpb".to_string()), true);
        assert_eq!(is_nice_part_2("xxyxx".to_string()), true);
        assert_eq!(is_nice_part_2("uurcxstgmygtbstg".to_string()), false);
        assert_eq!(is_nice_part_2("ieodomkazucvgmuy".to_string()), false);
    }
}
