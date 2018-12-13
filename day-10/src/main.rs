// https://adventofcode.com/2018/day/10

// imports

extern crate image;

// helpers

fn substring(this: &str, start: usize, len: usize) -> String {
    this.chars().skip(start).take(len).collect()
}

struct Star {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Star {
    fn wait_for(&mut self, seconds: i32) {
        let (x, y) = self.position;
        let (x_velocity, y_velocity) = self.velocity;

        self.position = (x + x_velocity * seconds, y + y_velocity * seconds);
    }
}

fn parse_input_to_star(input: &str) -> Star {
    let input: Vec<&str> = input.trim().split("velocity").map(|x| x.trim()).collect();

    let position: (i32, i32) = {
        let raw_string = input.get(0).unwrap();

        let skip = "position=<".len();
        let end = raw_string.len() - skip - 1;
        let raw_string = substring(raw_string, skip, end);

        let tokens: Vec<i32> = raw_string
            .split(',')
            .map(|x| -> i32 {
                return x.trim().parse().unwrap();
            })
            .collect();

        (*tokens.get(0).unwrap(), *tokens.get(1).unwrap())
    };

    let velocity: (i32, i32) = {
        let raw_string = input.get(1).unwrap();

        let skip = "=<".len();
        let end = raw_string.len() - skip - 1;
        let raw_string = substring(raw_string, skip, end);

        let tokens: Vec<i32> = raw_string
            .split(',')
            .map(|x| -> i32 {
                return x.trim().parse().unwrap();
            })
            .collect();

        (*tokens.get(0).unwrap(), *tokens.get(1).unwrap())
    };

    Star { position, velocity }
}

fn main() {
    let wait_for = 10124;

    let input_string = include_str!("input.txt");

    let inputs: Vec<Star> = input_string
        .trim()
        .lines()
        .map(|x| parse_input_to_star(x))
        .map(|mut x| {
            x.wait_for(wait_for);
            x
        })
        .collect();

    let max_x = inputs.iter().map(|star| star.position.0).max().unwrap();
    let min_x = inputs.iter().map(|star| star.position.0).min().unwrap();
    let max_y = inputs.iter().map(|star| star.position.1).max().unwrap();
    let min_y = inputs.iter().map(|star| star.position.1).min().unwrap();

    println!("min_x: {}", min_x);
    println!("min_y: {}", min_y);

    println!("max_x: {}", max_x);
    println!("max_y: {}", max_y);

    let margin_gap = 50;

    let x_adjustment = if min_x < 0 {
        min_x.abs() + margin_gap
    } else {
        0
    };

    let y_adjustment = if min_y < 0 {
        min_y.abs() + margin_gap
    } else {
        0
    };

    let width = max_x + x_adjustment + 1 + margin_gap * 2;
    let height = max_y + y_adjustment + 1 + margin_gap * 2;

    println!("width: {}", width);
    println!("height: {}", height);
    println!("area: {}", height * width);

    let mut img_buffer = image::GrayImage::new(width as u32, height as u32);

    for star in inputs.iter() {
        let (x, y) = star.position;

        let x = x_adjustment + x;
        let y = y_adjustment + y;

        assert!(x < width);
        assert!(x >= 0);
        assert!(y < height);
        assert!(y >= 0);

        let pixel = img_buffer.get_pixel_mut(x as u32, y as u32);

        let color: u8 = 255u8;
        *pixel = image::Luma([color]);
    }

    img_buffer.save("day_10_result.png").unwrap();
}
