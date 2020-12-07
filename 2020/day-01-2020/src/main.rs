fn main() {
    let input_string = include_str!("input.txt");

    // println!("{:?}", input_string);

    // Part 1

    let inputs: Vec<&str> = input_string.trim().split_whitespace().collect();

    let entries: Vec<i32> = inputs
        .into_iter()
        .map(|entry| -> i32 {
            return entry.parse().unwrap();
        })
        .collect();

    println!("Part 1: {}", part_1(entries.clone()));
}

fn part_1(entries: Vec<i32>) -> i32 {
    for (index, entry) in entries.iter().enumerate() {
        let start_index = index + 1;

        if start_index >= entries.len() {
            // no entries found that sum to 2020 :(
            break;
        }

        let rest = &entries[start_index..];
        for other_entry in rest {
            if entry + other_entry == 2020 {
                return entry * other_entry;
            }
        }
    }

    unreachable!();
}
