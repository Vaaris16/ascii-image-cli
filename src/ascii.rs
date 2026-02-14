use crate::config::*;

fn luminance(r: u8, g: u8, b: u8) -> f32 {
    0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32
}

fn gamma_correct(l: f32) -> f32 {
    (l / 255.0).powf(2.2) * 255.0
}

pub fn ascii_char(chars: &str, r: u8, g: u8, b: u8) -> char {
    let brightness = luminance(r, g, b);
    let brightness = gamma_correct(brightness);

    let t = (brightness / 255.0).clamp(0.0, 1.0);
    let index = (t * (chars.len() - 1) as f32).round() as usize;

    chars.chars().nth(index).unwrap()
}
