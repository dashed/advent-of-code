// https://adventofcode.com/2018/day/5

fn is_same_type(x: char, y: char) -> bool {
    return x.to_lowercase().to_string() == y.to_lowercase().to_string();
}

fn is_opposite_polarity(x: char, y: char) -> bool {
    return (x.is_uppercase() && y.is_lowercase()) || (y.is_uppercase() && x.is_lowercase());
}

fn does_react(x: char, y: char) -> bool {
    return is_same_type(x, y) && is_opposite_polarity(x, y);
}

fn main() {
    let input_string = include_str!("input.txt");

    let mut units: Vec<char> = input_string.chars().collect();

    println!("len: {:?}", units.len());

    // skip first N units known to not react
    let mut skip_n = 0;

    'outer_loop: loop {

        let mut units_iterable = units.iter().enumerate().skip(skip_n).peekable();

        println!("skipping: {} {:?}", skip_n, units_iterable.peek().unwrap());

        while let Some((current_index, current_unit)) = units_iterable.next() {
            if units_iterable.peek().is_none() {
                // no further reactions possible
                break 'outer_loop;
            }

            let (next_index, next_unit) = units_iterable.peek().unwrap();

            if does_react(*current_unit, **next_unit) {
                println!("{} {}{}", current_index, current_unit, next_unit);
                // println!("{},{}", current_index, next_index);

                // remove these items and start from the beginning
                units.drain(current_index..(current_index + 2));
                // units.remove(current_index);
                // units.remove(current_index);

                skip_n = if current_index == 0 {
                    current_index
                } else {
                    current_index - 1
                };

                // println!("new_len: {:?}", units.len());

                break;
            }
        }
    }

    println!("remaining units: {:?}", units.len());
    let final_result: String = units.into_iter().collect();
    println!("final_result: {}", final_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_same_type() {
        assert_eq!(is_same_type('a', 'a'), true);
        assert_eq!(is_same_type('a', 'A'), true);
        assert_eq!(is_same_type('a', 'b'), false);
    }

    #[test]
    fn test_is_opposite_polarity() {
        assert_eq!(is_opposite_polarity('a', 'a'), false);
        assert_eq!(is_opposite_polarity('B', 'B'), false);
        assert_eq!(is_opposite_polarity('a', 'A'), true);
        assert_eq!(is_opposite_polarity('A', 'a'), true);
        assert_eq!(is_opposite_polarity('a', 'B'), true);
    }

    #[test]
    fn test_does_react() {
        assert_eq!(does_react('a', 'a'), false);
        assert_eq!(does_react('A', 'A'), false);
        assert_eq!(does_react('a', 'A'), true);
    }
}
