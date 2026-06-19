use crate::color::Color;
use crate::constants::*;
use crate::cursor::move_cursor_by;
use crate::OutputFormat;
use std::io::{self, Write};

pub struct ColorGroup {
    pub original: Color,
    pub shaded: Color,
    pub tinted: Color,
}

impl ColorGroup {
    fn print_colors_full(&self, term_width: usize) {
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

    fn print_colors_basic(&self) {
        let colors = [&self.original, &self.tinted, &self.shaded];
        for color in colors {
            println!("{}: {}, {}", &color.title, color.hex_string(), color.rgb_string());
        }
    }

    pub fn print_colors(&self, output_format: OutputFormat, term_width: usize) {
        match output_format {
            OutputFormat::Full => self.print_colors_full(term_width),
            OutputFormat::Basic => self.print_colors_basic()
        }
    }
}
