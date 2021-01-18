extern crate image;

const sigma: f64 = 10.0;
const rho: f64 = 28.0;
const beta: f64 = 8.0 / 3.0;

const dt: f64 = 0.0001;

const background: u8 = 30;

fn main() {
    // use some random data
    let mut x: f64 = 1.0;
    let mut y: f64 = 0.0;
    let mut z: f64 = 0.0;

    let imgx = 1920;
    let imgy = 1080;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([background, background, background]);
    }

    let mut r: u8 = 100;
    let mut g: u8 = 100;
    let mut b: u8 = 100;

    for _n in 1..900000 {
        let dx = (sigma * (y - x)) * dt;
        let dy = (x * (rho - z) - y) * dt;
        let dz = (x * y - beta * z) * dt;
        x = dx + x;
        y = dy + y;
        z = dz + z;

        let pixel = imgbuf.get_pixel_mut((x * 19.0 + 960.0) as u32, (y * 19.0 + 540.0) as u32);
        *pixel = image::Rgb([r, g, b]);

        // if r > 0 && b == 55 {
        //     r = r - 1;
        //     g = g + 1;
        // }

        // if g > 0 && r == 55 {
        //     g = g - 1;
        //     b = b + 1;
        // }

        // if b > 0 && g == 55 {
        //     r = r + 1;
        //     b = b - 1;
        // }
    }

    imgbuf.save("lorenz_system.png").unwrap()
}
