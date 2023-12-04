// https://adventofcode.com/2023/day/2

fn main() {
    let input_string = include_str!("input.txt");

    // Part 1

    let answer = part_1(input_string);
    println!("Part 1: {}", answer);
    assert_eq!(answer, 2006);
}

#[derive(Debug)]
struct Revealed {
    red_cubes: i32,
    blue_cubes: i32,
    green_cubes: i32,
}
#[derive(Debug)]
struct Game {
    id: i32,
    reveals: Vec<Revealed>,
}

impl Game {
    fn is_valid(&self, max_red_cubes: i32, max_blue_cubes: i32, max_green_cubes: i32) -> bool {
        for reveal in &self.reveals {
            if !(reveal.red_cubes <= max_red_cubes
                && reveal.blue_cubes <= max_blue_cubes
                && reveal.green_cubes <= max_green_cubes)
            {
                return false;
            }
        }

        true
    }
}

fn part_1(input_string: &str) -> i32 {
    let inputs: Vec<&str> = input_string.trim().lines().collect();

    let mut games: Vec<Game> = vec![];

    for raw_input in inputs {
        let input: Vec<&str> = raw_input.trim().split(':').collect();
        let game_id: i32 = input[0]
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<i32>()
            .unwrap();

        let raw_reveals: Vec<&str> = input[1].trim().split(';').collect();
        let mut reveals: Vec<Revealed> = vec![];
        for reveal in raw_reveals {
            let reveal: Vec<&str> = reveal.trim().split(',').collect();
            let mut red_cubes = 0;
            let mut blue_cubes = 0;
            let mut green_cubes = 0;

            for cube in reveal {
                let cube: String = cube.trim().to_string();
                let num_of_cubes: i32 = cube
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();
                if cube.ends_with("red") {
                    red_cubes = num_of_cubes;
                } else if cube.ends_with("blue") {
                    blue_cubes = num_of_cubes;
                } else if cube.ends_with("green") {
                    green_cubes = num_of_cubes;
                } else {
                    panic!("invalid cube: {:?}", cube);
                }
            }

            let reveal = Revealed {
                red_cubes,
                blue_cubes,
                green_cubes,
            };
            reveals.push(reveal);
        }

        let game = Game {
            // raw_input: raw_input.to_string(),
            id: game_id,
            reveals,
        };
        games.push(game);
    }

    let max_red_cubes = 12;
    let max_green_cubes = 13;
    let max_blue_cubes = 14;

    games
        .into_iter()
        .filter(|x| x.is_valid(max_red_cubes, max_blue_cubes, max_green_cubes))
        .map(|x| x.id)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_2() {
        let input_string = r###"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"###;

        assert_eq!(part_1(input_string), 8);
    }
}
