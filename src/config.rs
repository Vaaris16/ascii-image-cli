#[derive(Debug, Clone)]
pub struct Config {
    pub step: usize,
    pub print_chars: bool,
    pub greyscale: bool,
    pub charset: String,
    pub char_w: usize,
    pub char_h: usize,
}

pub fn get_charset(config: &Config) -> &'static str {
    match config.charset.to_lowercase().as_str() {
        "simple" => "@#%*+=-:.",
        "numbers" => "9876543210",
        "numbers_rev" => "0123456789",
        "binary" => "10",
        "binary_rev" => "01",
        "letters_lower" => "mnhqdykz",
        "letters_upper" => "MNHQDYKZ",
        "symbols" => "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,'\"^`.",
        "matrix" => "01|!@#$%^&*()",
        "extended" => "@%#*+=-:. ",
        "ascii_dense" => "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft",
        _ => "@#%*+=-:.",
    }
}
