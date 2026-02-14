# [rust] ascii image generator

a fast rust-based cli tool that converts images into ascii art and renders
the result as an image.

## requirements

- rust (latest stable)

## installation

```bash
cargo install ascii-image
```

## usage

```bash
ascii-image --input  [options]
```

# options

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
| `simple`        | <img src="examples/simple/simple.png" width="260" />               |
| `numbers`       | <img src="examples/numbers/numbers.png" width="260" />             |
| `numbers_rev`   | <img src="examples/numbers_rev/numbers_rev.png" width="260" />     |
| `binary`        | <img src="examples/binary/binary.png" width="260" />               |
| `binary_rev`    | <img src="examples/binary_rev/binary_rev.png" width="260" />       |
| `letters_lower` | <img src="examples/letters_lower/letter_lower.png" width="260" />  |
| `letters_upper` | <img src="examples/letters_upper/letters_upper.png" width="260" /> |
| `matrix`        | <img src="examples/matrix/matrix.png" width="260" />               |
| `extended`      | <img src="examples/extended/extended.png" width="260" />           |
| `ascii_dense`   | <img src="examples/ascii_dense/ascii_dense.png" width="260" />     |
| `symbols`       | <img src="examples/symbols/symbols.png" width="260" />             |
