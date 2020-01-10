use weekend_raytracer::{color, ppm};

#[test]
fn test_test() {
    const EXPECTED: &str = "P3\n1 1\n255\n0 0 0\n";
    let ppm_string = ppm::get_file_content(1, 1, |_, _| color::Color::new(0, 0, 0));
    assert_eq!(EXPECTED, ppm_string);
}
