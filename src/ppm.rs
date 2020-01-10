use crate::color::Color;

pub fn get_file_content<F>(width: u32, height: u32, pixels: F) -> String
where
    F: Fn(u32, u32) -> Color,
{
    let mut content = String::new();
    content.push_str(&format!("P3\n{} {}\n255\n", width, height));

    for y in (0u32..height).rev() {
        for x in 0u32..width {
            let color = pixels(x, y);
            content.push_str(&color.to_string());
            content.push_str("\n");
        }
    }

    content
}
