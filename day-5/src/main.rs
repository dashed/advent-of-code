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

    println!("{}", input_string);
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
        assert_eq!(is_opposite_polarity('a', 'B'), true);
    }

    #[test]
    fn test_does_react() {
        assert_eq!(does_react('a', 'a'), false);
        assert_eq!(does_react('A', 'A'), false);
        assert_eq!(does_react('a', 'A'), true);
    }
}
