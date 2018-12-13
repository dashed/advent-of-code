// https://adventofcode.com/2018/day/11

use rayon::prelude::*;

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

fn part_1(grid_serial_number: i32, sub_grid_size: i32) -> ((i32, i32), i32) {
    let height = 300;
    let width = 300;

    let x_range: Vec<i32> = (1..=(height - sub_grid_size + 1)).into_iter().collect();
    let y_range: Vec<i32> = (1..=(width - sub_grid_size + 1)).into_iter().collect();

    let best: ((i32, i32), i32) = x_range
        .into_par_iter()
        .map(|x| {
            let best: ((i32, i32), i32) = y_range
                .par_iter()
                .map(|y| {
                    let total =
                        get_total_power_level_of_square(x, *y, sub_grid_size, grid_serial_number);

                    let position: (i32, i32) = (x, *y);

                    return (position, total);
                })
                .max_by_key(|item| {
                    let (_position, total) = item;
                    return total.clone();
                })
                .unwrap();

            return best;
        })
        .max_by_key(|item| {
            let (_position, total) = item;
            return total.clone();
        })
        .unwrap();

    let (position, largest_total_power) = best;

    return (position, largest_total_power);
}

fn part_2(grid_serial_number: i32) {
    let range: Vec<i32> = (1..=300).into_iter().collect();

    let result = range
        .into_par_iter()
        .map(|sub_grid_size: i32| {
            println!("sub_grid_size: {}", sub_grid_size);

            let (position, total) = part_1(grid_serial_number, sub_grid_size);

            return (position, total, sub_grid_size);
        })
        .max_by_key(|x| {
            let (_position, total, _sub_grid_size) = x;
            return total.clone();
        })
        .unwrap();

    let (best_position, _total, best_sub_grid_size) = result;

    println!("Part 2: {:?},{}", best_position, best_sub_grid_size);
}

fn main() {
    let grid_serial_number = 4172;
    let sub_grid_size = 3;

    let (position, _) = part_1(grid_serial_number, sub_grid_size);

    println!("Part 1: {:?}", position);

    part_2(grid_serial_number);
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
