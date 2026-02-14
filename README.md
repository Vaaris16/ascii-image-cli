# [RUST] ASCII IMAGE GENERATOR

a fast rust-based cli tool that converts images into ascii art and renders
the result as an image.

## Requirements

- rust (latest stable)

## Installation

```bash
cargo install ascii-image
```

## Usage

```bash
ascii-image --input  [options]
```

# Options

| option                    | short | description       | default     |
| ------------------------- | ----- | ----------------- | ----------- |
| `--input <input>`         | `-i`  | input image file  | -           |
| `--output <output>`       | `-o`  | output ascii file | `ascii.png` |
| `--step <step>`           | `-s`  | pixel step size   | `8`         |
| `--font-size <font_size>` | `-f`  | font size         | `15`        |
| `--charset <charset>`     | `-c`  | character set     | `simple`    |
| `--print-chars`           |       | print to terminal | false       |
| `--greyscale`             |       | greyscale mode    | false       |
| `--help`                  | `-h`  | show help         | -           |

# Charset Presets

| Preset        | Characters                                                              |
| ------------- | ----------------------------------------------------------------------- |
| simple        | `@#%*+=-:.`                                                             |
| numbers       | `9876543210`                                                            |
| numbers_rev   | `0123456789`                                                            |
| binary        | `10`                                                                    |
| binary_rev    | `01`                                                                    |
| letters_lower | `mnhqdykz`                                                              |
| letters_upper | `MNHQDYKZ`                                                              |
| symbols       | `$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,'\"^.` |
| matrix        | `` `01 ``                                                               |
| extended      | `@%#*+=-:.`                                                             |
| ascii_dense   | `$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft`                              |

## Charset Examples

| Charset         | Example                                                            |
| --------------- | ------------------------------------------------------------------ |
| `simple`        | <img src="examples/simple/simple.png" width="550" />               |
| `numbers`       | <img src="examples/numbers/numbers.png" width="550" />             |
| `numbers_rev`   | <img src="examples/numbers_rev/numbers_rev.png" width="550" />     |
| `binary`        | <img src="examples/binary/binary.png" width="550" />               |
| `binary_rev`    | <img src="examples/binary_rev/binary_rev.png" width="550" />       |
| `letters_lower` | <img src="examples/letters_lower/letter_lower.png" width="550" />  |
| `letters_upper` | <img src="examples/letters_upper/letters_upper.png" width="550" /> |
| `matrix`        | <img src="examples/matrix/matrix.png" width="550" />               |
| `extended`      | <img src="examples/extended/extended.png" width="550" />           |
| `ascii_dense`   | <img src="examples/ascii_dense/ascii_dense.png" width="550" />     |
| `symbols`       | <img src="examples/symbols/symbols.png" width="550" />             |
