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

    let sub_grid_size = 3;
    let height = 300;
    let width = 300;

    let mut position: Option<(i32, i32)> = None;
    let mut largest_total_power = 0;

    for x in 1..=(width - sub_grid_size) {
        for y in 1..=(height - sub_grid_size) {
            let total = get_total_power_level_of_square(x, y, sub_grid_size, grid_serial_number);

            if total > largest_total_power {
                largest_total_power = total;
                position = Some((x, y));
            }
        }
    }

    println!("Part 1: {:?}", position);
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

    #[test]
    fn test_get_total_power_level_of_square() {
        assert_eq!(get_total_power_level_of_square(33, 45, 3, 18), 29);
        assert_eq!(get_total_power_level_of_square(21, 61, 3, 42), 30);
    }

}
