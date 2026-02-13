use clap::Parser;
use image::RgbImage;

mod ascii;
mod config;
mod load;
mod render;

use crate::{
    ascii::ascii_char,
    config::Config,
    load::{load_font, load_image},
    render::draw_char,
};

#[derive(Parser)]
#[command(name = "ascii-image")]
#[command(about = "Convert images into ASCII art")]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long, default_value = "ascii.png")]
    output: String,

    #[arg(short, long, default_value_t = 8)]
    step: usize,

    #[arg(short, long, default_value_t = 15.0)]
    font_size: f32,
}

fn main() {
    let args = Args::parse();

    let config = Config {
        step: args.step,
        font_size: args.font_size,
        char_w: (args.font_size * 0.6) as usize,
        char_h: (args.font_size * 1.2) as usize,
    };

    let img = load_image(&args.input);
    let font = load_font();

    let (w, h) = img.dimensions();
    let width = w as usize;
    let height = h as usize;

    let mut out = RgbImage::new(width as u32, height as u32);

    for y in (0..height).step_by(config.step) {
        for x in (0..width).step_by(config.step) {
            let pixel = img.get_pixel(x as u32, y as u32);
            let [r, g, b] = pixel.0;

            let ch = ascii_char(r, g, b);

            let base_x = x as u32;
            let base_y = y as u32;

            draw_char(&font, ch, [r, g, b], &mut out, base_x, base_y, &config);
        }
    }

    out.save(&args.output).expect("failed to save image");
}
