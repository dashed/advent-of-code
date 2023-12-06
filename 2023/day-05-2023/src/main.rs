use std::{cmp, collections::HashMap};

#[derive(Debug, Clone)]
struct CategoryRange {
    category: String,
    range_start: i64,
    range_end: i64,
    range_length: i64,
}

impl CategoryRange {
    fn is_valid(&self) -> bool {
        let proper_range = self.range_start <= self.range_end;
        let expected_length = self.range_end - self.range_start + 1;
        let proper_length = self.range_length == expected_length && expected_length > 0;

        proper_range && proper_length
    }

    fn is_in_source_range(&self, category_range: &CategoryRange) -> bool {
        if self.range_start < category_range.range_start {
            return false;
        }

        if self.range_end > category_range.range_end {
            return false;
        }

        true
    }
}

#[derive(Debug)]
enum MappingResult {
    Overlap(CategoryRange),
    NoOverlap(CategoryRange),
}

#[derive(Debug)]
struct Range {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}

impl Range {
    fn is_in_source_range(&self, source: i64) -> bool {
        if source < self.source_range_start {
            return false;
        }
        // invariant: source_range_end - self.source_range_start + 1 = self.range_length
        let source_range_end = self.source_range_start + self.range_length - 1;
        if source > source_range_end {
            return false;
        }
        true
    }

    fn get_destination(&self, source: i64) -> i64 {
        if !self.is_in_source_range(source) {
            // Any source numbers that aren't mapped correspond to the same destination number.
            return source;
        }

        let zero_indexed_source = source - self.source_range_start;

        self.destination_range_start + zero_indexed_source
    }

    fn get_destination_ranges(
        &self,
        destination_category: String,
        input_range: CategoryRange,
    ) -> Vec<MappingResult> {
        assert!(input_range.category != destination_category);
        let mut mapping_results: Vec<MappingResult> = Vec::new();

        // check range overlap
        let source_range_end = self.source_range_start + self.range_length - 1;
        let has_overlap = self.source_range_start <= input_range.range_end
            && input_range.range_start <= source_range_end;

        if has_overlap {
            let new_range_start = cmp::min(self.source_range_start, input_range.range_start);

            let new_range_end = cmp::max(source_range_end, input_range.range_end);

            let destination_overlap_range_start =
                cmp::max(self.source_range_start, input_range.range_start);
            let destination_overlap_range_end = cmp::min(source_range_end, input_range.range_end);

            let left_non_overlap = CategoryRange {
                category: destination_category.clone(),
                range_start: new_range_start,
                range_end: destination_overlap_range_start - 1,
                range_length: destination_overlap_range_start - new_range_start,
            };

            let overlap = CategoryRange {
                category: destination_category.clone(),
                // range_start: self.get_destination(destination_overlap_range_start),
                // range_end: self.get_destination(destination_overlap_range_end),
                range_start: destination_overlap_range_start,
                range_end: destination_overlap_range_end,
                range_length: destination_overlap_range_end - destination_overlap_range_start + 1,
            };

            let right_non_overlap = CategoryRange {
                category: destination_category.clone(),
                range_start: destination_overlap_range_end + 1,
                range_end: new_range_end,
                range_length: new_range_end - destination_overlap_range_end,
            };

            if left_non_overlap.is_valid() && left_non_overlap.is_in_source_range(&input_range) {
                mapping_results.push(MappingResult::NoOverlap(left_non_overlap));
            }
            if overlap.is_valid() {
                let mut overlap = overlap;
                assert!(overlap.is_in_source_range(&input_range));

                overlap.range_start = self.get_destination(destination_overlap_range_start);
                overlap.range_end = self.get_destination(destination_overlap_range_end);

                mapping_results.push(MappingResult::Overlap(overlap));
            }
            if right_non_overlap.is_valid() && right_non_overlap.is_in_source_range(&input_range) {
                mapping_results.push(MappingResult::NoOverlap(right_non_overlap));
            }
        } else {
            // no overlap
            // let destination_range = CategoryRange {
            //     category: destination_category.clone(),
            //     range_start: input_range.range_start,
            //     range_end: input_range.range_end,
            //     range_length: input_range.range_length,
            // };

            // mapping_results.push(MappingResult::NoOverlap(destination_range));
        }

        mapping_results
    }
}

#[derive(Debug)]
struct Mapping {
    source_category: String,
    destination_category: String,
    ranges: Vec<Range>,
}

impl Mapping {
    fn get_destination(&self, source: i64) -> i64 {
        for range in self.ranges.iter() {
            if range.is_in_source_range(source) {
                return range.get_destination(source);
            }
        }

        // Any source numbers that aren't mapped correspond to the same destination number.
        source
    }

    fn get_destination_ranges(&self, source_ranges: &[CategoryRange]) -> Vec<CategoryRange> {
        let mut destination_ranges: Vec<CategoryRange> = Vec::new();
        let mut source_ranges: Vec<CategoryRange> = source_ranges.to_owned();

        'source_ranges_loop: loop {
            if source_ranges.is_empty() {
                break;
            }

            let source_range = source_ranges.remove(0);

            assert!(source_range.category == self.source_category);

            'ranges_for_loop: for range in self.ranges.iter() {
                let mapping_result = range.get_destination_ranges(
                    self.destination_category.clone(),
                    source_range.clone(),
                );

                if mapping_result.is_empty() {
                    // no overlap found. try to find an overlap in the next range in self.ranges.
                    continue 'ranges_for_loop;
                }

                if mapping_result.len() == 1 {
                    match &mapping_result[0] {
                        MappingResult::Overlap(range) => {
                            destination_ranges.push(range.clone());
                            continue 'source_ranges_loop;
                        }
                        MappingResult::NoOverlap(_range) => {
                            continue 'ranges_for_loop;
                        }
                    }
                }

                assert!(mapping_result.len() <= 3);
                let has_overlap = mapping_result.iter().any(|x| match x {
                    MappingResult::Overlap(_) => true,
                    MappingResult::NoOverlap(_) => false,
                });
                assert!(has_overlap);

                for result in mapping_result.into_iter() {
                    match result {
                        MappingResult::Overlap(range) => {
                            destination_ranges.push(range);
                        }
                        MappingResult::NoOverlap(range) => {
                            let mut range = range;
                            range.category = self.source_category.clone();
                            source_ranges.push(range);
                        }
                    }
                }
                continue 'source_ranges_loop;
            }

            // Any source numbers that aren't mapped correspond to the same destination number.
            let range = CategoryRange {
                category: self.destination_category.clone(),
                range_start: source_range.range_start,
                range_end: source_range.range_end,
                range_length: source_range.range_length,
            };
            destination_ranges.push(range);
        }

        destination_ranges
    }
}

fn find_location_for_seed(maps: &HashMap<String, Mapping>, seed: i64) -> i64 {
    let mut source = seed;
    let mut destination;
    let mut current_map = maps.get(&"seed".to_string()).unwrap();

    loop {
        destination = current_map.get_destination(source);
        if current_map.destination_category == "location" {
            break;
        }

        source = destination;
        current_map = maps.get(&current_map.destination_category).unwrap();
    }

    destination
}

fn part_1(input_string: &str) -> i64 {
    let mut inputs: Vec<&str> = input_string.trim().lines().collect();

    // first line is the seed input
    let seeds = inputs.remove(0).trim();
    assert!(seeds.starts_with("seeds: "));

    let seeds = {
        let seeds = seeds.trim_start_matches("seeds: ");
        seeds
            .split(' ')
            .map(|s| s.trim())
            .filter(|x| !x.is_empty())
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
    };

    let maps = generate_maps(inputs.iter().map(|s| s.to_string()).collect());

    let mut lowest_location_number: Option<i64> = None;

    for seed in seeds.into_iter() {
        let location_number = find_location_for_seed(&maps, seed);
        if lowest_location_number.is_none() {
            lowest_location_number = Some(location_number);
            continue;
        }
        lowest_location_number = cmp::min(lowest_location_number, Some(location_number));
    }

    lowest_location_number.unwrap()
}

fn generate_maps(inputs: Vec<String>) -> HashMap<String, Mapping> {
    let mut inputs = inputs;

    let mut maps: HashMap<String, Mapping> = HashMap::new();

    let mut current_map: Option<Mapping> = None;

    loop {
        if inputs.is_empty() {
            if current_map.is_some() {
                let map = current_map.unwrap();
                maps.insert(map.source_category.clone(), map);
            }

            break;
        }

        let current_line = inputs.remove(0).trim().to_string();

        if current_line.is_empty() {
            if current_map.is_some() {
                let map = current_map.unwrap();
                maps.insert(map.source_category.clone(), map);
                current_map = None;
            }

            continue;
        }

        if current_line.ends_with("map:") {
            assert!(current_map.is_none());

            let categories: Vec<String> = current_line
                .trim_end_matches("map:")
                .split("-to-")
                .map(|s| s.trim().to_string())
                .collect();
            assert!(categories.len() == 2);
            let source_category = categories[0].clone();
            let destination_category = categories[1].clone();

            current_map = Some(Mapping {
                source_category,
                destination_category,
                ranges: Vec::new(),
            });
            continue;
        }

        assert!(current_map.is_some());

        let map = current_map.as_mut().unwrap();

        let range_input = current_line
            .split(' ')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        assert!(range_input.len() == 3);

        map.ranges.push(Range {
            destination_range_start: range_input[0],
            source_range_start: range_input[1],
            range_length: range_input[2],
        });
    }

    maps
}

fn find_lowest_location_for_seed_range(
    maps: &HashMap<String, Mapping>,
    seed_range: &CategoryRange,
) -> i64 {
    let mut source_ranges: Vec<CategoryRange> = vec![seed_range.clone()];

    let mut current_map = maps.get(&"seed".to_string()).unwrap();

    let mut destination_ranges: Vec<CategoryRange>;

    loop {
        destination_ranges = current_map.get_destination_ranges(&source_ranges);

        for destination_range in destination_ranges.iter() {
            assert!(destination_range.category == current_map.destination_category);
        }

        if current_map.destination_category == "location" {
            break;
        }

        source_ranges = destination_ranges;
        current_map = maps.get(&current_map.destination_category).unwrap();
    }

    destination_ranges
        .iter()
        .map(|r| r.range_start)
        .min()
        .unwrap()
}

fn part_2(input_string: &str) -> i64 {
    let mut inputs: Vec<&str> = input_string.trim().lines().collect();

    // first line is the seed input
    let seeds = inputs.remove(0).trim();
    assert!(seeds.starts_with("seeds: "));

    let seed_ranges: Vec<CategoryRange> = {
        let seeds = seeds.trim_start_matches("seeds: ");
        let mut numbers = seeds
            .split(' ')
            .map(|s| s.trim())
            .filter(|x| !x.is_empty())
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let mut seed_ranges: Vec<CategoryRange> = Vec::new();

        loop {
            if numbers.is_empty() {
                break;
            }

            assert!(numbers.len() >= 2);

            let range_start = numbers.remove(0);
            let range_length = numbers.remove(0);

            let seed_range = CategoryRange {
                category: "seed".to_string(),
                range_start,
                range_end: range_start + range_length - 1,
                range_length,
            };

            seed_ranges.push(seed_range);
        }

        seed_ranges
    };

    let maps = generate_maps(inputs.iter().map(|s| s.to_string()).collect());

    let mut lowest_location_number: Option<i64> = None;

    for seed_range in seed_ranges.iter() {
        let location_number = find_lowest_location_for_seed_range(&maps, seed_range);

        if lowest_location_number.is_none() {
            lowest_location_number = Some(location_number);
            continue;
        }
        lowest_location_number = cmp::min(lowest_location_number, Some(location_number));
    }

    lowest_location_number.unwrap()
}

fn main() {
    let input_string = include_str!("input.txt");

    // Part 1

    let answer = part_1(input_string);
    println!("Part 1: {}", answer);
    assert_eq!(answer, 178159714);

    // Part 2

    let answer = part_2(input_string);
    println!("Part 2: {}", answer);
    assert_eq!(answer, 100165128);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle() {
        let input_string = r###"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"###;

        assert_eq!(part_1(input_string), 35);
        assert_eq!(part_2(input_string), 46);
    }
}
