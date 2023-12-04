type Recipe = i32;

struct Elf {
    position: usize,
}

impl Elf {
    fn get_recipe(&self, scoreboard: &[Recipe]) -> Recipe {
        return *scoreboard.get(self.position).unwrap();
    }

    fn move_to_new_position(&mut self, current_recipe: Recipe, scoreboard: &Vec<Recipe>) {
        let num_of_moves: usize = (1 + current_recipe) as usize;
        self.position = (self.position + num_of_moves) % scoreboard.len();
    }
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

        let recipe_result = recipe_1 + recipe_2;
        assert!(recipe_result <= 18);

        // the resulting recipes are added to the scoreboard

        if recipe_result >= 10 {
            scoreboard.push(recipe_result / 10);
            scoreboard.push(recipe_result % 10);
        } else {
            scoreboard.push(recipe_result);
        }

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
            if fetch_index > start_index {
                return (fetch_index, false);
            }
            return (start_index + 1, false);
        }
    }

    (start_index, true)
}

fn part_2(part_1_input: String) -> usize {
    let needle = part_1_input
        .chars()
        .map(|x| -> i32 { x.to_string().parse().unwrap() })
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

        let recipe_result = recipe_1 + recipe_2;
        assert!(recipe_result <= 18);

        // the resulting recipes are added to the scoreboard

        if recipe_result >= 10 {
            scoreboard.push(recipe_result / 10);
            scoreboard.push(recipe_result % 10);
        } else {
            scoreboard.push(recipe_result);
        }

        first_elf.move_to_new_position(recipe_1, &scoreboard);
        second_elf.move_to_new_position(recipe_2, &scoreboard);

        let (offset, found) = compare_vectors(&needle, &scoreboard, num_of_recipes_to_skip);

        if found {
            return num_of_recipes_to_skip;
        }

        // println!("num_of_recipes_to_skip: {}", offset);
        num_of_recipes_to_skip = offset;
    }
}

fn main() {
    let input = 540391;
    let result = part_1(input);
    println!("Part 1: {}", result);

    let result = part_2(input.to_string());
    println!("Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(9), "5158916779".to_string());
        assert_eq!(part_1(5), "0124515891".to_string());
        assert_eq!(part_1(18), "9251071085".to_string());
        assert_eq!(part_1(2018), "5941429882".to_string());
    }

    #[test]
    fn test_compare_vectors() {
        assert_eq!(
            compare_vectors(&vec![2, 3, 4, 5], &vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 1),
            (1, true)
        );
        assert_eq!(
            compare_vectors(&vec![2, 3, 4, 9], &vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 1),
            (4, false)
        );
        assert_eq!(
            compare_vectors(
                &vec![9, 2, 3, 4, 9],
                &vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                1
            ),
            (2, false)
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("51589".to_string()), 9);
        assert_eq!(part_2("01245".to_string()), 5);
        assert_eq!(part_2("92510".to_string()), 18);
        assert_eq!(part_2("59414".to_string()), 2018);
    }
}
