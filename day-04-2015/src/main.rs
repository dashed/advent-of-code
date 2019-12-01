// https://adventofcode.com/2015/day/4

extern crate md5;

use rayon::prelude::*;

fn general(secret_key: &str, num_of_zeroes: usize) -> i32 {

    let marker: String = String::from_utf8(vec![b'0'; num_of_zeroes]).unwrap();

    let mut current = 0;
    let size = 100000;

    loop {
        let start: i32 = 1 + current * size;
        let end: i32 = size + current * size;

        let lowest_positive_num: Option<i32> = (start..=end)
            .into_par_iter()
            .map(|lowest_positive_num: i32| -> Option<i32> {
                let digest = md5::compute(format!("{}{}", secret_key, lowest_positive_num));
                let digest = format!("{:x}", digest);

                if digest.starts_with(&marker) {
                    return Some(lowest_positive_num);
                }

                return None;
            })
            .fold(
                || None,
                |lowest_positive_num: Option<i32>, candidate: Option<i32>| -> Option<i32> {
                    match candidate {
                        None => {
                            return lowest_positive_num;
                        }
                        Some(maybe_lowest_positive_num) => match lowest_positive_num {
                            None => {
                                return Some(maybe_lowest_positive_num);
                            }
                            Some(lowest_positive_num) => {
                                if maybe_lowest_positive_num < lowest_positive_num {
                                    return Some(maybe_lowest_positive_num);
                                }
                                return Some(lowest_positive_num);
                            }
                        },
                    }
                },
            )
            .reduce(
                || None,
                |lowest_positive_num: Option<i32>, candidate: Option<i32>| -> Option<i32> {
                    match candidate {
                        None => {
                            return lowest_positive_num;
                        }
                        Some(maybe_lowest_positive_num) => match lowest_positive_num {
                            None => {
                                return Some(maybe_lowest_positive_num);
                            }
                            Some(lowest_positive_num) => {
                                if maybe_lowest_positive_num < lowest_positive_num {
                                    return Some(maybe_lowest_positive_num);
                                }
                                return Some(lowest_positive_num);
                            }
                        },
                    }
                },
            );

        if lowest_positive_num.is_none() {
            current = current + 1;
            continue;
        }

        let lowest_positive_num = lowest_positive_num.unwrap();
        return lowest_positive_num;
    }
}

fn part_1(secret_key: &str) -> i32 {
    return general(secret_key, 5);
}

fn part_2(secret_key: &str) -> i32 {
    return general(secret_key, 6);
}

fn main() {
    // Part 1

    let secret_key = "ckczppom";

    println!("Part 1: {}", part_1(secret_key));

    // Part 1

    println!("Part 2: {}", part_2(secret_key));

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("abcdef"), 609043);
        assert_eq!(part_1("pqrstuv"), 1048970);
    }

}
