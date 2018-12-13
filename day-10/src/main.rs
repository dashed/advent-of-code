extern crate image;

struct Star {
    position: (i32, i32),
    velocity: (i32, i32),
}

fn parse_input_to_star(input: &str) {

}

fn main() {

    let input_string = include_str!("input.txt");
    // println!("{:?}", input_string);

    let width = 10000;
    let height = 10000;

    // Create a new ImgBuf with width: imgx and height: imgy
    // let mut imgbuf = image::GrayImage::new(width, height);

    // Iterate over the coordinates and pixels of the image
    // for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {

    //     let color: u8 = (x % 256) as u8;

    //     *pixel = image::Luma([color]);
    // }

    // imgbuf.save("fractal.png").unwrap();
}
