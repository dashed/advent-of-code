extern crate image;

fn main() {

    // let input_string = include_str!("input.txt");
    // println!("{:?}", input_string);

    let width = 1000;
    let height = 1000;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::GrayImage::new(width, height);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {

        let color: u8 = (x % 256) as u8;

        *pixel = image::Luma([color]);
    }

    imgbuf.save("fractal.png").unwrap();
}
