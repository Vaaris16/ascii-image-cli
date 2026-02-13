use image::RgbImage;
use rusttype::Font;

pub fn load_image(path: &str) -> RgbImage {
    image::open(path).expect("failed to load image").to_rgb8()
}

pub fn load_font() -> Font<'static> {
    let font_data: &[u8] = include_bytes!("../JetBrainsMonoNL-Regular.ttf");
    Font::try_from_bytes(font_data).expect("invalid font")
}
