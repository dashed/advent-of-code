// https://adventofcode.com/2019/day/1

fn fuel_required(mass: i32) -> i32 {
    (((mass as f64) / 3.0).floor() - 2.0) as i32
}

fn part_1(inputs: Vec<&str>) -> i32 {
    let total_fuel_requirements: i32 = inputs
        .clone()
        .into_iter()
        .map(|module| -> i32 {
            let module_mass: i32 = module.parse().unwrap();
            fuel_required(module_mass)
        })
        .sum();

    total_fuel_requirements
}

fn fuel_required_part_2(mass: i32) -> i32 {
    let mut sum = fuel_required(mass);
    let mut last_fuel_required = sum;

    loop {
        last_fuel_required = fuel_required(last_fuel_required);
        if last_fuel_required <= 0 {
            return sum;
        }

        sum += last_fuel_required;
    }
}

fn part_2(inputs: Vec<&str>) -> i32 {
    let total_fuel_requirements: i32 = inputs
        .into_iter()
        .map(|module| -> i32 {
            let mass: i32 = module.parse().unwrap();
            fuel_required_part_2(mass)
        })
        .sum();

    total_fuel_requirements
}

fn main() {
    let input_string = include_str!("input.txt");

    // Part 1

    let inputs: Vec<&str> = input_string.split_whitespace().collect();

    println!("Part 1: {}", part_1(inputs.clone()));

    // Part 2

    println!("Part 2: {}", part_2(inputs));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_required() {
        assert_eq!(fuel_required(12), 2);
        assert_eq!(fuel_required(14), 2);
        assert_eq!(fuel_required(1969), 654);
        assert_eq!(fuel_required(100756), 33583);

        let input_string = include_str!("input.txt");
        let inputs: Vec<&str> = input_string.split_whitespace().collect();
        assert_eq!(part_1(inputs), 3249140);
    }

    #[test]
    fn test_fuel_required_part_2() {
        assert_eq!(fuel_required_part_2(14), 2);
        assert_eq!(fuel_required_part_2(1969), 966);
        assert_eq!(fuel_required_part_2(100756), 50346);

        let input_string = include_str!("input.txt");
        let inputs: Vec<&str> = input_string.split_whitespace().collect();
        assert_eq!(part_2(inputs), 4870838);
    }
}
