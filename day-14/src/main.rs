type Recipe = i32;

struct Elf {
    position: usize,
}

impl Elf {
    fn get_recipe(&self, receipes: &Vec<Recipe>) -> Recipe {
        return *receipes.get(self.position).unwrap();
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

fn main() {
    let mut scoreboard = vec![3, 7];

    let first_elf = Elf { position: 0 };
    let second_elf = Elf { position: 1 };

    // each elf gets a recipe from the scoreboard

    let recipe_1 = first_elf.get_recipe(&scoreboard);
    let recipe_2 = second_elf.get_recipe(&scoreboard);

    // each elf combine their current recipes

    let result = generate_recipes_from_digits(recipe_1 + recipe_2);

    // the resulting recipes are added to the scoreboard

    scoreboard.extend(result);

    println!("{:?}", scoreboard);
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
