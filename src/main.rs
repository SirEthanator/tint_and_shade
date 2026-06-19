mod constants;
mod cursor;
mod log;

mod color;
use color::Color;

mod color_group;
use color_group::ColorGroup;

use clap::{Parser, ValueEnum};
use std::env;
use std::process::ExitCode;

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

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum OutputFormat {
    Full,
    Basic,
}

#[derive(Parser)]
struct CliArgs {
    /// List of colors in hex or rgb format. E.g. #FFFFFF, FFFFFF, rgb(255, 255, 255)
    #[arg(required = true)]
    colors: Vec<String>,

    /// Percentage to tint and shade by.
    #[arg(short, long, value_parser=clap::value_parser!(u8).range(0..=100))]
    percentage: u8,

    /// Format for clipboard copying. Omit to copy nothing.
    #[arg(long, value_enum)]
    copy: Option<CopyMode>,

    /// Delimiter used to separate copied items.
    #[arg(long, value_enum)]
    copy_separator: Option<CopySeparator>,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Full)]
    output_format: OutputFormat,
}

fn main() -> ExitCode {
    let term_supports_truecolor = match env::var("COLORTERM") {
        Ok(val) => val == "truecolor" || val == "24bit",
        Err(_) => false,
    };

    let mut args = CliArgs::parse();

    if !term_supports_truecolor && args.output_format != OutputFormat::Basic {
        log::warn("Terminal does not support truecolor. Falling back to basic output.");
        args.output_format = OutputFormat::Basic;
    }

    if args.copy_separator.is_some() && args.copy.is_none() {
        log::warn("Specified --copy-separator but not --copy. This does nothing.");
    }

    let mut clipboard_items: Vec<String> = Vec::new();

    let term_width = get_term_width().unwrap_or_else(|| {
        log::warn("Failed to get terminal size: Falling back to basic ouput mode");
        args.output_format = OutputFormat::Basic;
        0
    });

    let mut iter = args.colors.iter().peekable();
    while let Some(color_string) = iter.next() {
        let color: Color;

        if let Some(hex) = Color::parse_hex(color_string) {
            color = Color::from_hex(&hex, "Original");
        } else if let Some(rgb) = Color::parse_rgb(color_string) {
            color = Color::from_rgb(&rgb, "Original");
        } else {
            log::warn(&format!("Skipping invalid color: \"{}\"", color_string));
            continue;
        }

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

        group.print_colors(args.output_format, term_width);

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
            log::error("Failed to access system clipboard");
        } else {
            // Needed to make clipboard contents stay after program exits on Wayland
            std::thread::sleep(std::time::Duration::from_millis(15));
        }
    }

    ExitCode::SUCCESS
}

fn get_term_width() -> Option<usize> {
    if let Some((term_width, _)) = term_size::dimensions() {
        return Some(term_width);
    }
    None
}
