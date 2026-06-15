mod cursor;

use clap::{Parser, ValueEnum};
use cursor::{fmt_move_cursor_by, move_cursor_by};
use std::env;
use std::io::{self, Write};

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const UNBOLD: &str = "\x1b[22m";
const TRUE_BLACK: &str = "\x1b[38;2;0;0;0m";
const TRUE_WHITE: &str = "\x1b[38;2;255;255;255m";

const BOX_SIDE_PADDING: usize = 2;
const BOX_SPACING: usize = 0;

const BOX_WIDTH: usize = "rgb(000, 000, 000)".len() + BOX_SIDE_PADDING * 2;

// Do not change unless fmt_box is changed
// This should hold the number of lines that the box uses
const BOX_HEIGHT: usize = 5;

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum CopyMode {
    RgbShades,
    RgbTints,
    HexShades,
    HexTints,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum CopySeparator {
    Space,
    Newline,
}

#[derive(Parser)]
struct CliArgs {
    #[arg(short, long)]
    color: String,

    #[arg(short, long)]
    percentage: u8,

    #[arg(long, value_enum)]
    copy: Option<CopyMode>,

    #[arg(long, value_enum)]
    copy_separator: Option<CopySeparator>,
}

fn hex_to_rgb(hex: &str) -> [u8; 3] {
    let mut result: [u8; 3] = [0, 0, 0];
    for i in 0..hex.len() / 2 {
        result[i] = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).unwrap();
    }
    result
}

fn rgb_to_hex(rgb: &[u8; 3]) -> String {
    let mut result = String::new();
    for c in rgb {
        let hex = format!("{:02X}", c);
        result += &hex;
    }
    result
}

fn get_term_width() -> usize {
    if let Some((term_width, _)) = term_size::dimensions() {
        term_width
    } else {
        eprintln!("Error getting terminal size.");
        std::process::exit(1);
    }
}

struct Color {
    hex: String,
    rgb: [u8; 3],
    title: String,
}

impl Color {
    pub fn new_hex(hex: &str, title: &str) -> Self {
        let rgb = hex_to_rgb(hex);

        Color {
            hex: String::from(hex),
            rgb,
            title: String::from(title),
        }
    }

    fn new_rgb(rgb: &[u8; 3], title: &str) -> Self {
        let hex = rgb_to_hex(rgb);
        Color {
            hex,
            rgb: *rgb,
            title: String::from(title),
        }
    }

    fn rgb_string(&self) -> String {
        format!(
            "rgb({:03}, {:03}, {:03})",
            self.rgb[0], self.rgb[1], self.rgb[2]
        )
    }

    fn hex_string(&self) -> String {
        format!("#{}", &self.hex)
    }

    fn shade(&self, percentage: u8) -> Self {
        let factor: f64 = 1.0 - percentage as f64 / 100.0;
        let mut out_rgb: [u8; 3] = [0, 0, 0];

        for (i, &val) in self.rgb.iter().enumerate() {
            out_rgb[i] = (factor * val as f64).round() as u8;
        }

        Self::new_rgb(&out_rgb, "Shade")
    }

    fn tint(&self, percentage: u8) -> Self {
        let factor: f64 = percentage as f64 / 100.0;
        let mut out_rgb: [u8; 3] = [0, 0, 0];

        for (i, &val) in self.rgb.iter().enumerate() {
            out_rgb[i] = (val as f64 + ((255.0 - val as f64) * factor)).round() as u8;
        }

        Self::new_rgb(&out_rgb, "Tint")
    }

    fn fmt_box(&self, x: usize) -> String {
        let title_len = self.title.len();
        let mut hex = self.hex_string();
        let mut rgb_str = self.rgb_string();
        let mut title = format!("{}{}{}", BOLD, self.title, UNBOLD);

        let side_padding = " ".repeat(BOX_SIDE_PADDING);

        rgb_str = format!(
            "{}{}{}",
            &side_padding,
            rgb_str,
            " ".repeat(BOX_WIDTH - rgb_str.len() - BOX_SIDE_PADDING)
        );
        title = format!(
            "{}{}{}",
            &side_padding,
            title,
            " ".repeat(BOX_WIDTH - title_len - BOX_SIDE_PADDING)
        );
        hex = format!(
            "{}{}{}",
            &side_padding,
            hex,
            " ".repeat(BOX_WIDTH - hex.len() - BOX_SIDE_PADDING)
        );

        let top_bottom_padding = " ".repeat(BOX_WIDTH);

        let lines = [
            top_bottom_padding.clone(),
            title,
            rgb_str,
            hex,
            top_bottom_padding,
        ];
        let line_separator = format!("\n{}", fmt_move_cursor_by(x as i16, 0));
        let out = lines.join(&line_separator);

        highlight_string(&out, &self.rgb)
    }
}

struct ColorGroup {
    original: Color,
    shaded: Color,
    tinted: Color,
}

impl ColorGroup {
    fn print_colors(&self, term_width: usize) {
        let colors = [&self.tinted, &self.original, &self.shaded];

        let mut cursor_x = 0;

        for (i, color) in colors.iter().enumerate() {
            print!("{}", color.fmt_box(cursor_x));
            let _ = io::stdout().flush();

            // Stop early on last box, nothing more needs to be done
            if i == colors.len() - 1 {
                break;
            }

            let dx = BOX_WIDTH + BOX_SPACING;
            let new_cursor_x = cursor_x + dx;

            if new_cursor_x + BOX_WIDTH > term_width {
                print!("\n\n");
                cursor_x = 0;
            } else {
                move_cursor_by(BOX_SPACING as i16, -(BOX_HEIGHT as i16) + 1);
                cursor_x = new_cursor_x;
            }
        }

        println!();
    }
}

fn text_color(bg: &[u8; 3]) -> &'static str {
    // https://www.w3.org/TR/AERT/#color-contrast
    let brightness: f64 =
        ((bg[0] as f64 * 299.0) + (bg[1] as f64 * 587.0) + (bg[2] as f64 * 114.0)) / 1000.0;
    if brightness > 125.0 {
        TRUE_BLACK
    } else {
        TRUE_WHITE
    }
}

fn highlight_string(str: &str, bg: &[u8; 3]) -> String {
    let bg_str = format!("\x1b[48;2;{};{};{}m", bg[0], bg[1], bg[2]);
    let fg_str = text_color(bg);
    format!("{}{}{}{}", bg_str, fg_str, str, RESET)
}

fn main() {
    let term_supports_truecolor = match env::var("COLORTERM") {
        Ok(val) => val == "truecolor" || val == "24bit",
        Err(_) => false,
    };

    if !term_supports_truecolor {
        eprintln!("Warning: Terminal does not support truecolor. Output will not look correct.");
    }

    let args = CliArgs::parse();

    let hex_codes = args.color.replace("#", "");
    let hex_codes = hex_codes.split(" ");

    let mut clipboard_items: Vec<String> = Vec::new();

    println!();

    let mut iter = hex_codes.peekable();
    while let Some(hex) = iter.next() {
        let color = Color::new_hex(hex, "Original");
        let shaded = color.shade(args.percentage);
        let tinted = color.tint(args.percentage);

        match args.copy {
            None => {}
            Some(CopyMode::RgbShades) => clipboard_items.push(shaded.rgb_string()),
            Some(CopyMode::RgbTints) => clipboard_items.push(tinted.rgb_string()),
            Some(CopyMode::HexShades) => clipboard_items.push(shaded.hex_string()),
            Some(CopyMode::HexTints) => clipboard_items.push(tinted.hex_string()),
        }

        let group = ColorGroup {
            original: color,
            shaded,
            tinted,
        };

        let term_width = get_term_width();

        group.print_colors(term_width);

        if iter.peek().is_some() {
            println!();
        }
    }

    if !clipboard_items.is_empty() {
        let clipboard_separator_str: &str = match args.copy_separator {
            None => " ",
            Some(CopySeparator::Space) => " ",
            Some(CopySeparator::Newline) => "\n",
        };

        let clipboard_string = clipboard_items.join(clipboard_separator_str);

        let copy_result = cli_clipboard::set_contents(clipboard_string);
        if copy_result.is_err() {
            eprintln!("Error: Failed to copy to clipboard");
        } else {
            // Needed to make clipboard contents stay after program exits on Wayland
            std::thread::sleep(std::time::Duration::from_millis(15));
        }
    }
}
