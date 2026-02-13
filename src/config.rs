#[derive(Debug, Clone)]
pub struct Config {
    pub step: usize,
    pub font_size: f32,
    pub char_w: usize,
    pub char_h: usize,
}

pub const CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
