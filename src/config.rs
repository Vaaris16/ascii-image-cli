#[derive(Debug, Clone)]
pub struct Config {
    pub step: usize,
    pub char_w: usize,
    pub char_h: usize,
}

pub const CHARS: &str = "@#W$9876543210?!abc;:+=-,._ ";
