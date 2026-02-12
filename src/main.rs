use image::{Rgb, RgbImage};
use rusttype::{Font, Scale};

const CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let img = image::open("image.jpg").expect("failed to load image");
    let img = img.to_rgb8();

    let font_data = std::fs::read("JetBrainsMonoNL-Regular.ttf").expect("failed to read font");
    let font = Font::try_from_vec(font_data).expect("invalid font type");
    let scale = Scale::uniform(16.0);
    let step: usize = 8;

    let (w, h) = img.dimensions();

    let width: usize = w as usize;
    let height: usize = h as usize;

    let char_w: usize = 10;
    let char_h: usize = 18;

    let out_w = ((width / step) * char_w) as u32;
    let out_h = ((height / step) * char_h) as u32;

    let mut out = RgbImage::new(out_w as u32, out_h as u32);

    for y in (0..height).step_by(step) {
        for x in (0..width).step_by(step) {
            let pixel = img.get_pixel(x as u32, y as u32);
            let [r, g, b] = pixel.0;

            let brightness =
                ((0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) * 1.5).min(255.0);

            let index = (brightness as usize * (CHARS.len() - 1)) / 255;

            let ch = CHARS.chars().nth(index).unwrap();

            let base_x = ((x / step) * char_w) as u32;
            let base_y = ((y / step) * char_h) as u32;

            let glyph = font.glyph(ch).scaled(scale).positioned(rusttype::point(
                base_x as f32,
                (base_y + char_h as u32) as f32,
            ));

            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|px, py, v| {
                    let x = px + bb.min.x as u32;
                    let y = py + bb.min.y as u32;

                    if x < out.width() && y < out.height() {
                        out.put_pixel(
                            x,
                            y,
                            Rgb([
                                (r as f32 * v) as u8,
                                (g as f32 * v) as u8,
                                (b as f32 * v) as u8,
                            ]),
                        );
                    }
                });
            }
        }
        out.save("ascii_blocks.png").expect("failed to save image");
    }
}
