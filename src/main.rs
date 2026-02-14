use clap::Parser;
use image::RgbImage;
use std::io::{self, Write};

mod ascii;
mod config;
mod load;
mod render;

use crate::{
    ascii::ascii_char,
    config::{Config, get_charset},
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

    #[arg[long]]
    print_chars: bool,

    #[arg[long]]
    greyscale: bool,

    #[arg(short, long)]
    charset: String,
}

fn main() {
    let args = Args::parse();

    let config = Config {
        step: args.step,
        print_chars: args.print_chars,
        greyscale: args.greyscale,
        charset: args.charset,
        char_w: (args.font_size * 0.6) as usize,
        char_h: (args.font_size * 1.2) as usize,
    };

    let img = load_image(&args.input);
    let font = load_font();

    let (w, h) = img.dimensions();
    let width = w as usize;
    let height = h as usize;

    let out_w = ((width / config.step) * config.char_w) as u32;
    let out_h = ((height / config.step) * config.char_h) as u32;
    let mut out = RgbImage::new(out_w as u32, out_h as u32);

    let ascii_ch = get_charset(&config);

    for y in (0..height).step_by(config.step) {
        for x in (0..width).step_by(config.step) {
            let pixel = img.get_pixel(x as u32, y as u32);
            let [r, g, b] = pixel.0;

            let ch = ascii_char(ascii_ch, r, g, b);

            let base_x = ((x / config.step) * config.char_w) as u32;
            let base_y = ((y / config.step) * config.char_h) as u32;

            draw_char(
                &font,
                ch,
                [r, g, b],
                &mut out,
                base_x,
                base_y,
                &config,
                config.greyscale,
            );

            if config.print_chars {
                if config.greyscale {
                    print!("{}", ch);
                } else {
                    print!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, ch);
                }
            }
        }

        if config.print_chars {
            println!();
            io::stdout().flush().unwrap();
        }
    }

    out.save(&args.output).expect("failed to save image");
}
