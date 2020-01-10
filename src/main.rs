mod color;
mod ppm;

use color::Color;

fn main() {
    let width = 200;
    let height = 100;

    let output = ppm::get_file_content(width, height, |u: u32, v: u32| -> Color {
        let r = u as f32 / width as f32;
        let g = v as f32 / height as f32;
        let b = 0.2f32;

        let ir = (255.99 * r) as u8;
        let ig = (255.99 * g) as u8;
        let ib = (255.99 * b) as u8;

        Color::new(ir, ig, ib)
    });

    print!("{}", output);
}
