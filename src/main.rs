use image::RgbImage;

mod ascii;
mod constants;
mod load;
mod render;

use crate::{
    ascii::ascii_char,
    constants::*,
    load::{load_font, load_image},
    render::draw_char,
};

fn main() {
    let img = load_image("image.jpg");

    let font = load_font("JetBrainsMonoNL-Regular.ttf");

    let (w, h) = img.dimensions();

    let width: usize = w as usize;
    let height: usize = h as usize;

    let out_w = ((width / STEP) * CHARW) as u32;
    let out_h = ((height / STEP) * CHARH) as u32;

    let mut out = RgbImage::new(out_w as u32, out_h as u32);

    for y in (0..height).step_by(STEP) {
        for x in (0..width).step_by(STEP) {
            let pixel = img.get_pixel(x as u32, y as u32);
            let [r, g, b] = pixel.0;

            let ch = ascii_char(r, g, b);

            let base_x = ((x / STEP) * CHARW) as u32;
            let base_y = ((y / STEP) * CHARH) as u32;

            draw_char(&font, ch, [r, g, b], &mut out, base_x, base_y);
        }
        out.save("ascii_blocks.png").expect("failed to save image");
    }
}
