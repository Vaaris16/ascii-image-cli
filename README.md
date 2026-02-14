# [RUST] ASCII IMAGE GENERATOR

A fast Rust-based CLI tool that converts images into ASCII art and renders
the result as an image.

## Requirements

- Rust (latest stable)

## Installation

```bash
cargo install ascii-image
```

## Usage

```bash
ascii-image --input  [OPTIONS]
```

# Options

| Option                    | Short | Description                                    | Default     |
| ------------------------- | ----- | ---------------------------------------------- | ----------- |
| `--input <INPUT>`         | `-i`  | Path to the input image file (required)        | -           |
| `--output <OUTPUT>`       | `-o`  | Path to save the ASCII art output              | `ascii.png` |
| `--step <STEP>`           | `-s`  | Pixel sampling step size (lower = more detail) | `8`         |
| `--font-size <FONT_SIZE>` | `-f`  | Font size for the ASCII characters             | `15`        |
| `--help`                  | `-h`  | Print help information                         | -           |

<div align="center">
  <img src="./examples/image-1-complex-paris/paris.png" alt="Description"/>
</div>
