use crate::constants::CHARS;

pub fn ascii_char(r: u8, g: u8, b: u8) -> char {
    let brightness = ((0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) * 1.5).min(255.0);

    let index = (brightness as usize * (CHARS.len() - 1)) / 255;

    CHARS.chars().nth(index).unwrap()
}
