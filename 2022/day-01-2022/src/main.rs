// https://adventofcode.com/2022/day/1

type Calories = i32;

#[derive(Debug, Clone)]
struct Elf {
    food: Vec<Calories>,
}

impl Elf {
    fn get_total_calories(&self) -> i32 {
        self.food.iter().sum()
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    // Part 1

    let inputs: Vec<&str> = input_string.trim().split('\n').collect();

    let mut elves: Vec<Elf> = vec![];

    let mut current_elf = Elf { food: vec![] };

    for input in inputs {
        let input = input.trim();
        if input.is_empty() {
            elves.push(current_elf.clone());
            current_elf = Elf { food: vec![] };
            continue;
        }
        let calories = input.parse::<i32>().unwrap();
        current_elf.food.push(calories);
    }

    // Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
    elves.sort_by_key(|elf| -> i32 { elf.get_total_calories() });
    elves.reverse();
    let elve_carrying_most_calories = elves.first().unwrap();

    let total_calories: i32 = elve_carrying_most_calories.get_total_calories();
    println!("Part 1: {}", total_calories);
    assert_eq!(total_calories, 70509);

    assert!(elves.len() >= 3);

    let total_calories_first_3_elves: i32 = elves
        .iter()
        .take(3)
        .map(|elf| elf.get_total_calories())
        .sum();

    assert_eq!(total_calories_first_3_elves, 208567);
    println!("Part 2: {}", total_calories_first_3_elves);
}
