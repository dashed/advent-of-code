// https://adventofcode.com/2018/day/11

use rayon::prelude::*;

fn get_row_major_order_idx(x: usize, y: usize, width: usize) -> usize {
    width * y + x
}

struct SummedAreaTable {
    grid_size: usize,
    inner_array: Vec<i32>,
}

// https://en.wikipedia.org/wiki/Summed-area_table
impl SummedAreaTable {
    fn new(grid_size: usize, grid_serial_number: i32) -> SummedAreaTable {
        assert!(grid_size > 0);

        // this will be a square summed-area table

        let area = (grid_size * grid_size) as usize;

        // linear array: https://en.wikipedia.org/wiki/Row-_and_column-major_order
        let mut inner_array = vec![0; area];

        // pre-populate the array with their corresponding power levels
        for x in 0..grid_size {
            for y in 0..grid_size {
                let normalized_x = (x + 1) as i32;
                let normalized_y = (y + 1) as i32;

                let power_level = get_power_level(normalized_x, normalized_y, grid_serial_number);

                let index = get_row_major_order_idx(x, y, grid_size);
                inner_array[index] = power_level;
            }
        }

        // for each fuel cell (item in the array), the sum of all the values above
        // and to the left of (x, y), inclusive
        for x in 0..grid_size {
            for y in 0..grid_size {
                // sum itself
                let mut sum: i32 = inner_array[get_row_major_order_idx(x, y, grid_size)];

                // to the left
                if x > 0 {
                    sum += inner_array[get_row_major_order_idx(x - 1, y, grid_size)];
                }

                // above
                if y > 0 {
                    sum += inner_array[get_row_major_order_idx(x, y - 1, grid_size)];
                }

                // above and to the left
                if x > 0 && y > 0 {
                    sum -= inner_array[get_row_major_order_idx(x - 1, y - 1, grid_size)];
                }

                inner_array[get_row_major_order_idx(x, y, grid_size)] = sum;
            }
        }

        SummedAreaTable {
            grid_size,
            inner_array,
        }
    }

    fn get_spanned_square(&self, x: usize, y: usize, sub_grid_size: usize) -> i32 {
        assert!(x < self.grid_size);
        assert!(y < self.grid_size);
        assert!(sub_grid_size <= self.grid_size);
        assert!(x + sub_grid_size <= self.grid_size);

        // top-left corner
        let top_left = if x > 0 && y > 0 {
            self.inner_array[get_row_major_order_idx(x - 1, y - 1, self.grid_size)]
        } else {
            0
        };

        // invariant: end_x - x + 1 = sub_grid_size
        let end_x = sub_grid_size + x - 1;
        // invariant: end_y - y + 1 = sub_grid_size
        let end_y = sub_grid_size + y - 1;

        // top-right corner
        let top_right = if y > 0 {
            self.inner_array[get_row_major_order_idx(end_x, y - 1, self.grid_size)]
        } else {
            0
        };

        // bottom-left corner
        let bottom_left = if x > 0 {
            self.inner_array[get_row_major_order_idx(x - 1, end_y, self.grid_size)]
        } else {
            0
        };

        // bottom-right corner
        let bottom_right = self.inner_array[get_row_major_order_idx(end_x, end_y, self.grid_size)];

        bottom_right + top_left - top_right - bottom_left
    }
}

fn get_power_level(x: i32, y: i32, grid_serial_number: i32) -> i32 {
    assert!(x > 0);
    assert!(y > 0);

    let rack_id = x + 10;
    let power_level = rack_id * y;
    let power_level = power_level + grid_serial_number;
    let power_level = power_level * rack_id;

    let power_level = power_level.to_string();

    assert!(power_level.len() >= 3);

    let skip_n = power_level.len() - 3;

    let power_level: i32 = power_level
        .chars()
        .nth(skip_n)
        .unwrap()
        .to_string()
        .parse()
        .unwrap();

    power_level - 5
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

    let x_range: Vec<i32> = (start_x..=end_x).into_iter().collect();
    let y_range: Vec<i32> = (start_y..=end_y).into_iter().collect();

    let total: i32 = x_range
        .into_par_iter()
        .map(|x| -> i32 {
            let result: i32 = y_range
                .par_iter()
                .map(|y| -> i32 { get_power_level(x, *y, grid_serial_number) })
                .sum();

            result
        })
        .sum();

    total
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

                    (position, total)
                })
                .max_by_key(|item| {
                    let (_position, total) = item;
                    *total
                })
                .unwrap();

            best
        })
        .max_by_key(|item| {
            let (_position, total) = item;
            *total
        })
        .unwrap();

    let (position, largest_total_power) = best;

    (position, largest_total_power)
}

fn part_1_optimized(
    summed_area_table: &SummedAreaTable,
    sub_grid_size: usize,
) -> ((usize, usize), i32) {
    let width = summed_area_table.grid_size;
    let height = summed_area_table.grid_size;

    let mut largest_total_power = 0;
    let mut best_position = None;

    for x in 1..=(width - sub_grid_size + 1) {
        for y in 1..=(height - sub_grid_size + 1) {
            let total = summed_area_table.get_spanned_square(x - 1, y - 1, sub_grid_size);
            if total > largest_total_power {
                largest_total_power = total;
                best_position = Some((x, y));
            }
        }
    }

    (best_position.unwrap(), largest_total_power)
}

#[allow(dead_code)]
fn part_2(grid_serial_number: i32) {
    let range: Vec<i32> = (1..=300).into_iter().collect();

    let result = range
        .into_par_iter()
        .map(|sub_grid_size: i32| {
            println!("sub_grid_size: {}", sub_grid_size);

            let (position, total) = part_1(grid_serial_number, sub_grid_size);

            (position, total, sub_grid_size)
        })
        .max_by_key(|x| {
            let (_position, total, _sub_grid_size) = x;
            *total
        })
        .unwrap();

    let (best_position, _total, best_sub_grid_size) = result;

    println!("Part 2: {:?},{}", best_position, best_sub_grid_size);
}

fn part_2_optimized(summed_area_table: &SummedAreaTable) -> ((usize, usize), usize) {
    let width = summed_area_table.grid_size;
    let height = summed_area_table.grid_size;

    let sub_grid_size_range: Vec<usize> = (1..=summed_area_table.grid_size).into_iter().collect();

    let result = sub_grid_size_range
        .into_par_iter()
        .map(|sub_grid_size: usize| {
            let mut largest_total_power = 0;
            let mut best_position = None;

            for x in 1..=(width - sub_grid_size + 1) {
                for y in 1..=(height - sub_grid_size + 1) {
                    let total = summed_area_table.get_spanned_square(x - 1, y - 1, sub_grid_size);
                    if total > largest_total_power {
                        largest_total_power = total;
                        best_position = Some((x, y));
                    }
                }
            }

            (best_position, largest_total_power, sub_grid_size)
        })
        .max_by_key(|x| {
            let (_position, total, _sub_grid_size) = x;
            *total
        })
        .unwrap();

    let (best_position, _total, best_sub_grid_size) = result;

    (best_position.unwrap(), best_sub_grid_size)

    // Naive version
    // let mut largest_total_power = 0;
    // let mut best_position = None;
    // let mut best_grid_size = 0;
    // for sub_grid_size in 1..=grid_size {
    //     for x in 1..=(width - sub_grid_size + 1) {
    //         for y in 1..=(height - sub_grid_size + 1) {
    //             let total = summed_area_table.get_spanned_square(x - 1, y - 1, sub_grid_size);
    //             if total > largest_total_power {
    //                 largest_total_power = total;
    //                 best_position = Some((x, y));
    //                 best_grid_size = sub_grid_size
    //             }
    //         }
    //     }
    // }

    // return (best_position.unwrap(), best_grid_size);
}

fn main() {
    let grid_size = 300;
    let grid_serial_number = 4172;
    let sub_grid_size: usize = 3;

    let (position, power) = part_1(grid_serial_number, sub_grid_size as i32);
    println!("Part 1: {:?} {}", position, power);

    // TODO: this is slow af.
    // use this: https://en.wikipedia.org/wiki/Summed-area_table
    // part_2(grid_serial_number);

    let summed_area_table = SummedAreaTable::new(grid_size, grid_serial_number);

    let (position, power) = part_1_optimized(&summed_area_table, sub_grid_size);

    println!("Part 1 optimized: {:?} {}", position, power);

    let (position, grid_size) = part_2_optimized(&summed_area_table);
    println!("Part 2 optimized: {:?} {}", position, grid_size);
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
