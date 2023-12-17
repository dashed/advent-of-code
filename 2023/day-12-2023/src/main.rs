use core::panic;
use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
    Damaged,
    Operational,
    Unknown,
}

#[derive(Debug, Clone)]
struct Row {
    springs: Vec<Spring>,
    damage_report: Vec<usize>,
}

impl Row {
    #[allow(dead_code)]
    fn to_str(&self) -> String {
        let mut string = String::new();

        for spring in self.springs.iter() {
            match spring {
                Spring::Damaged => {
                    string.push('#');
                }
                Spring::Operational => {
                    string.push('.');
                }
                Spring::Unknown => {
                    string.push('?');
                }
            }
        }

        format!("{} {:?}", string, self.damage_report)
    }
}

fn part_1(input_string: &str) -> usize {
    let inputs: Vec<&str> = input_string.trim().lines().collect();

    let mut rows: Vec<Row> = Vec::new();

    for line in inputs {
        let line = line.trim();
        let split = line.split(' ').map(|x| x.trim()).collect::<Vec<&str>>();
        assert!(split.len() == 2);

        let row: &str = split[0];
        let damage_report = split[1];

        let springs = row
            .chars()
            .map(|x| match x {
                '#' => Spring::Damaged,
                '.' => Spring::Operational,
                '?' => Spring::Unknown,
                _ => panic!("Unexpected character: {}", x),
            })
            .collect::<Vec<Spring>>();

        let damage_report = damage_report
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>();

        let row = Row {
            springs,
            damage_report,
        };

        rows.push(row);
    }

    rows.into_par_iter()
        .map(|row| -> usize { count_possible_arangements(row) })
        .sum()
}

fn count_possible_arangements(row: Row) -> usize {
    let springs = row.springs;
    let counts = row.damage_report;
    let mut cache = HashMap::new();

    count_possible_arangements_inner(&springs, &counts, &mut cache)
}

fn count_possible_arangements_inner(
    springs: &Vec<Spring>,
    counts: &Vec<usize>,
    cache: &mut HashMap<(Vec<usize>, Vec<Spring>), usize>,
) -> usize {
    if springs.is_empty() {
        if counts.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }

    match springs[0] {
        Spring::Operational => {
            // We can skip this spring, and continue processing the rest
            count_possible_arangements_inner(&springs[1..].to_vec(), counts, cache)
        }
        Spring::Damaged => process_damaged_group(springs, counts, cache),
        Spring::Unknown => {
            let mut possible_arrrangements = 0;

            // We assume current spring is operational, and skip it to process the rest.
            possible_arrrangements +=
                count_possible_arangements_inner(&springs[1..].to_vec(), counts, cache);

            // We assume current spring is damaged, and process the rest.
            possible_arrrangements += process_damaged_group(springs, counts, cache);

            possible_arrrangements
        }
    }
}

fn process_damaged_group(
    springs: &Vec<Spring>,
    counts: &Vec<usize>,
    cache: &mut HashMap<(Vec<usize>, Vec<Spring>), usize>,
) -> usize {
    // invariant: assume springs[0] is damaged

    if counts.is_empty() {
        return 0;
    }

    if springs.is_empty() {
        return 0;
    }

    if let Some(&result) = cache.get(&(counts.clone(), springs.clone())) {
        return result;
    }

    let current_group_size = counts[0];
    if springs.len() < current_group_size {
        return 0;
    }

    if springs[..current_group_size].contains(&Spring::Operational) {
        return 0;
    }

    // assume springs[..current_group_size] are all damaged

    if springs.len() == current_group_size {
        if counts.len() == 1 {
            return 1;
        }
        // springs are not long enough to satisfy the next set of counts
        return 0;
    }

    if springs[current_group_size] == Spring::Damaged {
        // This group is one damaged spring too long.
        return 0;
    }

    // invariant: springs[current_group_size] is Operational or Unknown. If it is Unknown, then we still assume it is
    // Operational, since otherwise it would be part of this group.

    let result = count_possible_arangements_inner(
        &springs[(current_group_size + 1)..].to_vec(),
        &counts[1..].to_vec(),
        cache,
    );

    cache.insert((counts.clone(), springs.clone()), result);

    result
}

fn part_2(input_string: &str) -> usize {
    let inputs: Vec<&str> = input_string.trim().lines().collect();

    let mut rows: Vec<Row> = Vec::new();

    for line in inputs {
        let line = line.trim();
        let split = line.split(' ').map(|x| x.trim()).collect::<Vec<&str>>();
        assert!(split.len() == 2);

        let row: &str = split[0];
        let damage_report = split[1];

        let springs = row
            .chars()
            .map(|x| match x {
                '#' => Spring::Damaged,
                '.' => Spring::Operational,
                '?' => Spring::Unknown,
                _ => panic!("Unexpected character: {}", x),
            })
            .collect::<Vec<Spring>>();

        let damage_report = damage_report
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>();

        let row = Row {
            springs,
            damage_report,
        };

        rows.push(row);
    }

    rows.into_par_iter()
        .map(|row| -> usize {
            let springs: Vec<Spring> = row
                .springs
                .iter()
                .copied()
                .chain([Spring::Unknown])
                .cycle()
                .take(row.springs.len() * 5 + 4)
                .collect();

            let damage_report: Vec<usize> = row
                .damage_report
                .iter()
                .copied()
                .cycle()
                .take(row.damage_report.len() * 5)
                .collect();

            let row = Row {
                springs,
                damage_report,
            };

            count_possible_arangements(row)
        })
        .sum()
}

fn main() {
    let input_string = include_str!("input.txt");

    // Part 1

    let answer = part_1(input_string);
    println!("Part 1: {}", answer);
    assert_eq!(answer, 7204);

    // Part 2

    let answer = part_2(input_string);
    println!("Part 2: {}", answer);
    assert_eq!(answer, 1672318386674);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle() {
        let input_string = r###"
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"###;

        assert_eq!(part_1(input_string), 21);
        assert_eq!(part_2(input_string), 525152);
    }
}
