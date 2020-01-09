struct Color(u8, u8, u8);

fn main() {
    let width = 200;
    let height = 100;

    write_dummy_ppm(width, height, |u: u32, v: u32| -> Color {
        let r = u as f32 / width as f32;
        let g = v as f32 / height as f32;
        let b = 0.2f32;

        let ir = (255.99 * r) as u8;
        let ig = (255.99 * g) as u8;
        let ib = (255.99 * b) as u8;

        Color(ir, ig, ib)
    })
}

fn write_dummy_ppm<F>(width: u32, height: u32, pixels: F)
where
    F: Fn(u32, u32) -> Color,
{
    println!("P3\n{} {}\n255", width, height);
    for j in (0u32..height).rev() {
        for i in 0u32..width {
            let color = pixels(i, j);

            println!("{} {} {}", color.0, color.1, color.2);
        }
    }
}
