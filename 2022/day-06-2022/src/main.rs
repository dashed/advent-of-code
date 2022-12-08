// https://adventofcode.com/2022/day/6

fn marker_finder(input_string: String, marker_size: usize) -> Option<usize> {
    let mut buffer: Vec<char> = vec![];
    for (index, char) in input_string.chars().enumerate() {
        // invariant: buffer contains up to 3 unique characters

        if let Some(index) = buffer.iter().position(|&c| c == char) {
            buffer = buffer.iter().skip(index + 1).copied().collect();
        }

        buffer.push(char);

        if buffer.len() == marker_size {
            return Some(index + 1);
        }
    }

    None
}

fn part_1(input_string: String) -> Option<usize> {
    marker_finder(input_string, 4)
}

fn part_2(input_string: String) -> Option<usize> {
    marker_finder(input_string, 14)
}

fn main() {
    let input_string = include_str!("input.txt");
    let chars_seen = part_1(input_string.to_string()).unwrap();
    println!("Part 1: {}", chars_seen);
    assert_eq!(chars_seen, 1142);

    let chars_seen = part_2(input_string.to_string()).unwrap();
    println!("Part 2: {}", chars_seen);
    assert_eq!(chars_seen, 2803);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()),
            Some(7)
        );

        assert_eq!(part_1("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), Some(5));

        assert_eq!(part_1("nppdvjthqldpwncqszvftbrmjlhg".to_string()), Some(6));

        assert_eq!(
            part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()),
            Some(10)
        );

        assert_eq!(
            part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()),
            Some(11)
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()),
            Some(19)
        );

        assert_eq!(part_2("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), Some(23));

        assert_eq!(part_2("nppdvjthqldpwncqszvftbrmjlhg".to_string()), Some(23));

        assert_eq!(
            part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()),
            Some(29)
        );

        assert_eq!(
            part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()),
            Some(26)
        );
    }
}
