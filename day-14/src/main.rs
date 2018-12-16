type Recipe = i32;

struct Elf {
    position: usize,
}

impl Elf {
    fn get_recipe(&self, scoreboard: &Vec<Recipe>) -> Recipe {
        return *scoreboard.get(self.position).unwrap();
    }

    fn move_to_new_position(&mut self, current_recipe: Recipe, scoreboard: &Vec<Recipe>) {
        let num_of_moves: usize = (1 + current_recipe) as usize;
        self.position = (self.position + num_of_moves) % scoreboard.len();
    }
}

fn generate_recipes_from_digits(mixed_recipe: Recipe) -> Vec<Recipe> {
    return mixed_recipe
        .to_string()
        .chars()
        .map(|x| -> i32 {
            return x.to_string().parse().unwrap();
        })
        .collect();
}

fn part_1(part_1_input: usize) -> String {
    let part_1_guard = part_1_input + 10;

    let mut scoreboard = vec![3, 7];

    let mut first_elf = Elf { position: 0 };
    let mut second_elf = Elf { position: 1 };

    loop {
        // each elf gets a recipe from the scoreboard

        let recipe_1 = first_elf.get_recipe(&scoreboard);
        let recipe_2 = second_elf.get_recipe(&scoreboard);

        // each elf combine their current recipes

        let result = generate_recipes_from_digits(recipe_1 + recipe_2);

        // the resulting recipes are added to the scoreboard

        scoreboard.extend(result);

        first_elf.move_to_new_position(recipe_1, &scoreboard);
        second_elf.move_to_new_position(recipe_2, &scoreboard);

        if scoreboard.len() >= part_1_guard {
            let result: String = scoreboard
                .iter()
                .skip(part_1_input)
                .take(10)
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("");

            return result;
        }
    }
}

fn compare_vectors(
    needle: &Vec<Recipe>,
    scoreboard: &Vec<Recipe>,
    start_index: usize,
) -> (usize, bool) {
    assert!(start_index < scoreboard.len());

    // compare needle to recipes within scoreboard starting at index start_index

    if needle.len() > scoreboard.len() {
        return (start_index, false);
    }

    if start_index + (needle.len() - 1) >= scoreboard.len() {
        return (start_index, false);
    }

    for (index, recipe_needle) in needle.iter().enumerate() {
        let fetch_index = start_index + index;

        if fetch_index >= scoreboard.len() {
            return (start_index, false);
        }

        let recipe = scoreboard.get(fetch_index).unwrap();

        if recipe != recipe_needle {
            return (start_index + 1, false);
        }
    }

    return (start_index, true);
}

fn part_2(part_1_input: String) -> usize {
    let needle = part_1_input
        .chars()
        .map(|x| -> i32 {
            return x.to_string().parse().unwrap();
        })
        .collect();

    let mut num_of_recipes_to_skip = 0;

    let mut scoreboard = vec![3, 7];

    let mut first_elf = Elf { position: 0 };
    let mut second_elf = Elf { position: 1 };

    loop {
        // each elf gets a recipe from the scoreboard

        let recipe_1 = first_elf.get_recipe(&scoreboard);
        let recipe_2 = second_elf.get_recipe(&scoreboard);

        // each elf combine their current recipes

        let result = generate_recipes_from_digits(recipe_1 + recipe_2);

        // the resulting recipes are added to the scoreboard

        scoreboard.extend(result);

        first_elf.move_to_new_position(recipe_1, &scoreboard);
        second_elf.move_to_new_position(recipe_2, &scoreboard);

        let (offset, found) = compare_vectors(&needle, &scoreboard, num_of_recipes_to_skip);

        if found {
            return num_of_recipes_to_skip;
        }

        num_of_recipes_to_skip = offset;
    }
}

fn main() {
    let result = part_1(540391);
    println!("Part 1: {}", result);

    let result = part_2("540391".to_string());
    println!("Part 2: {}", result);
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

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(9), "5158916779".to_string());
        assert_eq!(part_1(5), "0124515891".to_string());
        assert_eq!(part_1(18), "9251071085".to_string());
        assert_eq!(part_1(2018), "5941429882".to_string());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("51589".to_string()), 9);
        assert_eq!(part_2("01245".to_string()), 5);
        assert_eq!(part_2("92510".to_string()), 18);
        assert_eq!(part_2("59414".to_string()), 2018);
    }
}
