# Tint and Shade

A minimal CLI for generating tints and shades from any color.

## Features

- Generate tints (lighter variants) and shades (darker variants) for multiple colors at once
- Hex and rgb input and output format
- Automatically copy the results to the clipboard
- Truecolor output to see generated colors right away
- See [Usage](#usage) for all options

## Usage

```
A minimal CLI for generating tints and shades from any color.

Usage: tint-and-shade [OPTIONS] --percentage <PERCENTAGE> <COLORS>...

Arguments:
  <COLORS>...  List of colors in hex or rgb format. E.g. #FFFFFF, FFFFFF, rgb(255, 255, 255)

Options:
  -p, --percentage <PERCENTAGE>
          Percentage to tint and shade by
      --copy <COPY>
          Format for clipboard copying. Omit to copy nothing [possible values: rgb-shades, rgb-tints, hex-shades, hex-tints]
      --copy-separator <COPY_SEPARATOR>
          Delimiter used to separate copied items [possible values: space, newline]
  -o, --output-format <OUTPUT_FORMAT>
          Output format [default: full] [possible values: full, basic]
  -h, --help
          Print help
  -V, --version
          Print version
```

## Building

```sh
git clone 'https://github.com/SirEthanator/tint-and-shade.git'
cd tint-and-shade
cargo build --release
# Alternatively, use 'cargo install --path .' to install
```

## Credits

- Big thanks to [edelstone](https://github.com/edelstone)'s project, [tints-and-shades](https://github.com/edelstone/tints-and-shades), a website found at [maketintsandshades.com](https://maketintsandshades.com). This provided the key formulas and inspiration for the project.
