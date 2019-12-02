// https://adventofcode.com/2018/day/10

// imports

extern crate image;

// helpers

fn substring(this: &str, start: usize, len: usize) -> String {
    this.chars().skip(start).take(len).collect()
}

#[derive(Debug, Clone)]
struct Star {
    position: (i64, i64),
    velocity: (i64, i64),
}

impl Star {
    fn wait_for(&mut self, seconds: i64) {
        let (x, y) = self.position;
        let (x_velocity, y_velocity) = self.velocity;

        self.position = (x + x_velocity * seconds, y + y_velocity * seconds);
    }
}

fn parse_input_to_star(input: &str) -> Star {
    let input: Vec<&str> = input.trim().split("velocity").map(|x| x.trim()).collect();

    let position: (i64, i64) = {
        let raw_string = input.get(0).unwrap();

        let skip = "position=<".len();
        let end = raw_string.len() - skip - 1;
        let raw_string = substring(raw_string, skip, end);

        let tokens: Vec<i64> = raw_string
            .split(',')
            .map(|x| -> i64 {
                return x.trim().parse().unwrap();
            })
            .collect();

        (*tokens.get(0).unwrap(), *tokens.get(1).unwrap())
    };

    let velocity: (i64, i64) = {
        let raw_string = input.get(1).unwrap();

        let skip = "=<".len();
        let end = raw_string.len() - skip - 1;
        let raw_string = substring(raw_string, skip, end);

        let tokens: Vec<i64> = raw_string
            .split(',')
            .map(|x| -> i64 {
                return x.trim().parse().unwrap();
            })
            .collect();

        (*tokens.get(0).unwrap(), *tokens.get(1).unwrap())
    };

    Star { position, velocity }
}

fn find_smallest_area(stars: Vec<Star>) -> Vec<Star> {
    // let wait_for = 10124;
    let mut wait_for = 10000;

    let mut best_stars: Vec<Star> = stars
        .into_iter()
        .map(|mut star| {
            star.wait_for(wait_for);
            star
        })
        .collect();

    let mut smallest_area = None;

    loop {
        let stars: Vec<Star> = best_stars
            .iter()
            .map(|star| {
                let mut star = star.clone();
                star.wait_for(1);
                star
            })
            .collect();

        let max_x = stars.iter().map(|star| star.position.0).max().unwrap();
        let min_x = stars.iter().map(|star| star.position.0).min().unwrap();
        let max_y = stars.iter().map(|star| star.position.1).max().unwrap();
        let min_y = stars.iter().map(|star| star.position.1).min().unwrap();

        let width = max_x - min_x + 1;
        let height = max_y - min_y + 1;

        assert!(width >= 0);
        assert!(height >= 0);
        let area: i64 = height * width;

        match smallest_area {
            None => {
                smallest_area = Some(area);
            }
            Some(best_area) => {
                if area > best_area {
                    println!("The stars align if you wait for {} seconds.", wait_for - 1);
                    return best_stars;
                }

                best_stars = stars;
                smallest_area = Some(area);
            }
        }

        wait_for += 1;
    }
}

fn main() {
    // let wait_for = 10124;

    let input_string = include_str!("input.txt");

    let inputs: Vec<Star> = {
        let initial_stars: Vec<Star> = input_string
            .trim()
            .lines()
            .map(|x| parse_input_to_star(x))
            .collect();

        find_smallest_area(initial_stars)
    };

    let max_x = inputs.iter().map(|star| star.position.0).max().unwrap();
    let min_x = inputs.iter().map(|star| star.position.0).min().unwrap();
    let max_y = inputs.iter().map(|star| star.position.1).max().unwrap();
    let min_y = inputs.iter().map(|star| star.position.1).min().unwrap();

    println!("min_x: {}", min_x);
    println!("min_y: {}", min_y);

    println!("max_x: {}", max_x);
    println!("max_y: {}", max_y);

    let margin_gap = 10;

    let x_adjustment = if min_x < 0 { min_x.abs() } else { -min_x };

    let y_adjustment = if min_y < 0 { min_y.abs() } else { -min_y };

    let width = (max_x - min_x + 1) + margin_gap * 2;
    let height = (max_y - min_y + 1) + margin_gap * 2;

    println!("width: {}", width);
    println!("height: {}", height);
    println!("area: {}", height * width);

    let mut img_buffer = image::GrayImage::new(width as u32, height as u32);

    for star in inputs.iter() {
        let (x, y) = star.position;

        let x = x_adjustment + x + margin_gap;
        let y = y_adjustment + y + margin_gap;

        assert!(x < width);
        assert!(x >= 0);
        assert!(y < height);
        assert!(y >= 0);

        let pixel = img_buffer.get_pixel_mut(x as u32, y as u32);

        let color: u8 = 255u8;
        *pixel = image::Luma([color]);
    }

    img_buffer.save("day-10/day_10_result.png").unwrap();
}
