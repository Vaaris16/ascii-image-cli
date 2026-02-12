use image::{Rgb, RgbImage};
use rusttype::{Font, Scale};

use crate::constants::*;

pub fn draw_char(
    font: &Font,
    ch: char,
    color: [u8; 3],
    out: &mut RgbImage,
    base_x: u32,
    base_y: u32,
) {
    let scale = Scale::uniform(FONTSIZE);
    let glyph = font.glyph(ch).scaled(scale).positioned(rusttype::point(
        base_x as f32,
        (base_y + CHARH as u32) as f32,
    ));

    if let Some(bb) = glyph.pixel_bounding_box() {
        glyph.draw(|px, py, v| {
            // convert px/py FIRST
            let x = bb.min.x + px as i32;
            let y = bb.min.y + py as i32;

            // bounds check in signed space
            if x >= 0 && y >= 0 {
                let x = x as u32;
                let y = y as u32;

                if x < out.width() && y < out.height() {
                    out.put_pixel(
                        x,
                        y,
                        Rgb([
                            (color[0] as f32 * v) as u8,
                            (color[1] as f32 * v) as u8,
                            (color[2] as f32 * v) as u8,
                        ]),
                    );
                }
            }
        });
    }
}
