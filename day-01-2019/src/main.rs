// https://adventofcode.com/2019/day/1

fn fuel_required(mass: i32) -> i32 {
    return (((mass as f64) / 3.0).floor() - 2.0) as i32;
}

fn fuel_required_part_2(mass: i32) -> i32 {
    let mut sum = fuel_required(mass);
    let mut last_fuel_required = sum;

    loop {
        last_fuel_required = fuel_required(last_fuel_required);
        if last_fuel_required <= 0 {
            return sum;
        }

        sum = sum + last_fuel_required;
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    // Part 1

    let inputs: Vec<&str> = input_string.trim().split_whitespace().collect();

    let total_fuel_requirements_part_1: i32 = inputs.clone()
        .into_iter()
        .map(|module| -> i32 {
            let module_mass: i32 = module.parse().unwrap();
            return fuel_required(module_mass);
        })
        .sum();

    println!("Part 1: {}", total_fuel_requirements_part_1);

    // Part 2

    let total_fuel_requirements_part_2: i32 = inputs
        .into_iter()
        .map(|module| -> i32 {
            let module_mass: i32 = module.parse().unwrap();
            return fuel_required_part_2(module_mass);
        })
        .sum();

    println!("Part 2: {}", total_fuel_requirements_part_2);
}
