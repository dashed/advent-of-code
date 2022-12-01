// https://adventofcode.com/2022/day/1

type Calories = i32;

#[derive(Debug, Clone)]
struct Elf {
    food: Vec<Calories>,
}

fn main() {
    let input_string = include_str!("input.txt");

    // Part 1

    let inputs: Vec<&str> = input_string.trim().split('\n').collect();

    let mut elves: Vec<Elf> = vec![];

    let mut current_elf = Elf { food: vec![] };

    for input in inputs {
        let input = input.trim();
        if input.len() <= 0 {
            elves.push(current_elf.clone());
            current_elf = Elf { food: vec![] };
            continue;
        }
        let calories = input.parse::<i32>().unwrap();
        current_elf.food.push(calories);

        println!("{}", calories);
    }

    // Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
    let elve_carrying_most_calories = elves
        .iter()
        .max_by_key(|elf| -> i32 { elf.food.iter().sum() })
        .unwrap();

    let total_calories: i32 = elve_carrying_most_calories.food.iter().sum();
    println!("Part 1: {}", total_calories);
}
