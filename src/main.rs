extern crate image;

use image::{ImageBuffer, Rgba};

const SIGMA: f64 = 10.0;
const RHO: f64 = 28.0;
const BETA: f64 = 8.0 / 3.0;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

const BACKGROUND: [u8; 4] = [30, 30, 30, 255];

const SCALE: f64 = 20.0;

fn main() {
    // use some random data
    let mut x: f64 = 1.0;
    let mut y: f64 = 0.0;
    let mut z: f64 = 0.0;
    let dt = 0.0001;

    let r: u8 = 100;
    let g: u8 = 100;
    let b: u8 = 100;

    // Create a new ImgBuf
    let mut imgbuf: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(WIDTH, HEIGHT);

    // Fill with background
    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = Rgba(BACKGROUND);
    }

    for _n in 1..600000 {
        // Lorenz system
        x = (SIGMA * (y - x)) * dt + x;
        y = (x * (RHO - z) - y) * dt + y;
        z = (x * y - BETA * z) * dt + z;

        let coordinate_x = (x * SCALE + WIDTH as f64 / 2.0) as u32;
        let coordinate_y = (y * SCALE + HEIGHT as f64 / 2.0) as u32;

        // skip out of bounds coordinates
        if coordinate_x < WIDTH && coordinate_y < HEIGHT {
            let pixel = imgbuf.get_pixel_mut(coordinate_x, HEIGHT - coordinate_y);
            *pixel = Rgba([r, g, b, 255]);
        }
    }

    imgbuf.save("lorenz_system.png").unwrap()
}
