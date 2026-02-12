use image::RgbImage;
use rusttype::Font;

pub fn load_image(path: &str) -> RgbImage {
    image::open(path).expect("failed to load image").to_rgb8()
}

pub fn load_font(font_name: &str) -> Font<'static> {
    let font_data = std::fs::read(font_name).expect("failed to read font");
    Font::try_from_vec(font_data).expect("invalid font type")
}
