fn main() {
    let input_string = include_str!("input.txt");

    // Part 1

    let inputs: Vec<&str> = input_string.trim().split_whitespace().collect();

    let entries: Vec<i32> = inputs
        .into_iter()
        .map(|entry| -> i32 {
            return entry.parse().unwrap();
        })
        .collect();

    println!("Part 1: {}", part_1(entries.clone()));

    // Part 2

    println!("Part 2: {}", part_2(entries.clone()));
}

fn part_1(entries: Vec<i32>) -> i32 {
    return find_candidates_sum(entries, 2020).unwrap();
}

fn part_2(entries: Vec<i32>) -> i32 {
    for (index, entry) in entries.iter().enumerate() {
        let mut rest = entries.clone();
        rest.remove(index);

        let expected_sum = 2020 - entry;

        assert!(expected_sum >= 0);

        let result = find_candidates_sum(rest, expected_sum);

        if result.is_none() {
            continue;
        }

        return result.unwrap() * entry;
    }

    unreachable!();
}

// find two candidates that add up to to sum, and multiply them
fn find_candidates_sum(entries: Vec<i32>, sum: i32) -> Option<i32> {
    for (index, entry) in entries.iter().enumerate() {
        let start_index = index + 1;

        if start_index >= entries.len() {
            return None;
        }

        let rest = &entries[start_index..];
        for other_entry in rest {
            if entry + other_entry == sum {
                return Some(entry * other_entry);
            }
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1() {
        let input_string = include_str!("input.txt");

        let inputs: Vec<&str> = input_string.trim().split_whitespace().collect();

        let entries: Vec<i32> = inputs
            .into_iter()
            .map(|entry| -> i32 {
                return entry.parse().unwrap();
            })
            .collect();

        assert_eq!(part_1(entries.clone()), 864864);
        assert_eq!(part_2(entries.clone()), 281473080);
    }
}
