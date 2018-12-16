struct Elf {
    position: usize,
}

fn generate_recipes_from_digits(sum: i32) -> Vec<i32> {
    return sum
        .to_string()
        .chars()
        .map(|x| -> i32 {
            return x.to_string().parse().unwrap();
        })
        .collect();
}

fn main() {
    let recipes = vec![3, 7];

    let first_elf = Elf { position: 0 };

    let second_elf = Elf { position: 1 };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_recipes_from_digits() {
        assert_eq!(generate_recipes_from_digits(1234), vec![1, 2, 3, 4]);
        assert_eq!(generate_recipes_from_digits(0), vec![0]);
        assert_eq!(generate_recipes_from_digits(1), vec![1]);
    }
}
