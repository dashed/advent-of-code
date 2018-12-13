// https://adventofcode.com/2018/day/11

fn get_power_level(x: i32, y: i32, grid_serial_number: i32) -> i32 {
    let rack_id = x + 10;
    let power_level = rack_id * y;
    let power_level = power_level + grid_serial_number;
    let power_level = power_level * rack_id;

    let power_level = power_level.to_string();

    assert!(power_level.len() >= 3);

    let skip_n = power_level.len() - 3;

    let power_level: i32 = power_level
        .chars()
        .skip(skip_n)
        .next()
        .unwrap()
        .to_string()
        .parse()
        .unwrap();

    return power_level - 5;
}

fn get_total_power_level_of_square(
    start_x: i32,
    start_y: i32,
    size: i32,
    grid_serial_number: i32,
) -> i32 {
    let size = size - 1;

    // end_y - start_x + 1 = size
    let end_x = size + start_x - 1;
    // similarly for end_y
    let end_y = size + start_y - 1;

    let mut total = 0;

    for x in start_x..=end_x {
        for y in start_y..=end_y {
            total += get_power_level(x, y, grid_serial_number);
        }
    }

    return total;
}

fn main() {
    let grid_serial_number = 4172;

    let height = 300;
    let width = 300;

    get_power_level(3, 5, 8);

    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_power_level() {
        assert_eq!(get_power_level(3, 5, 8), 4);
        assert_eq!(get_power_level(122, 79, 57), -5);
        assert_eq!(get_power_level(217, 196, 39), 0);
        assert_eq!(get_power_level(101, 153, 71), 4);
    }

}
