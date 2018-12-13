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

fn parse_input_to_star(input: &str) -> Star {

    let input: Vec<&str> = input.trim().split("velocity").map(|x| x.trim()).collect();

    let position: (i32, i32) = {

        let raw_string = input.get(0).unwrap();

        let skip = "position=<".len();
        let end = raw_string.len() - skip - 1;
        let raw_string = substring(raw_string, skip, end);

        let tokens: Vec<i32> = raw_string.split(',').map(|x| -> i32 {
            return x.trim().parse().unwrap();
        }).collect();

        (*tokens.get(0).unwrap(), *tokens.get(1).unwrap())
    };


    let velocity: (i32, i32) = {

        let raw_string = input.get(1).unwrap();

        let skip = "=<".len();
        let end = raw_string.len() - skip - 1;
        let raw_string = substring(raw_string, skip, end);

        let tokens: Vec<i32> = raw_string.split(',').map(|x| -> i32 {
            return x.trim().parse().unwrap();
        }).collect();

        (*tokens.get(0).unwrap(), *tokens.get(1).unwrap())
    };

    Star {
        position,
        velocity
    }
}

fn main() {

    let input_string = include_str!("input.txt");

    let inputs: Vec<_> = input_string.trim().lines().map(|x| parse_input_to_star(x)).collect();

    println!("{:?}", inputs.len());

    let width = 1000;
    let height = 1000;

    let mut img_buffer = image::GrayImage::new(width, height);

    // Iterate over the coordinates and pixels of the image
    // for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {

    //     let color: u8 = (x % 256) as u8;

    //     *pixel = image::Luma([color]);
    // }

    // imgbuf.save("fractal.png").unwrap();
}
