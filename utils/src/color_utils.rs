use colored::*;

use crate::color_config;

pub trait Labels {
    fn label(&self) -> String;
    fn highlight(&self) -> String;
    fn highlight_alt(&self) -> String;
}

impl Labels for String {
    fn label(&self) -> Self {
        apply_color(&self, color_config::LABEL_COLOR)
    }

    fn highlight(&self) -> Self {
        apply_color(&self, color_config::HIGHLIGHT_COLOR)
    }

    fn highlight_alt(&self) -> Self {
        apply_color(&self, color_config::HIGHLIGHT_ALT_COLOR)
    }
}

impl Labels for &str {
    fn label(&self) -> String {
        apply_color(&self, color_config::LABEL_COLOR)
    }

    fn highlight(&self) -> String {
        apply_color(&self, color_config::HIGHLIGHT_COLOR)
    }

    fn highlight_alt(&self) -> String {
        apply_color(&self, color_config::HIGHLIGHT_ALT_COLOR)
    }
}

pub trait Colors {
    fn green(&self) -> String;
    fn yellow(&self) -> String;
    fn red(&self) -> String;
    fn blue(&self) -> String;
    fn dark_blue(&self) -> String;
    fn black(&self) -> String;
    fn white(&self) -> String;
    fn dark_white(&self) -> String;
}

impl Colors for String {
    fn green(&self) -> Self {
        apply_color(&self, color_config::GREEN)
    }

    fn yellow(&self) -> Self {
        apply_color(&self, color_config::YELLOW)
    }

    fn red(&self) -> Self {
        apply_color(&self, color_config::RED)
    }

    fn blue(&self) -> Self {
        apply_color(&self, color_config::BLUE)
    }

    fn dark_blue(&self) -> Self {
        apply_color(&self, color_config::DARK_BLUE)
    }

    fn black(&self) -> Self {
        apply_color(&self, color_config::BLACK)
    }

    fn white(&self) -> Self {
        apply_color(&self, color_config::WHITE)
    }

    fn dark_white(&self) -> Self {
        apply_color(&self, color_config::DARK_WHITE)
    }
}

impl Colors for &str {
    fn green(&self) -> String {
        apply_color(&self, color_config::GREEN)
    }

    fn yellow(&self) -> String {
        apply_color(&self, color_config::YELLOW)
    }

    fn red(&self) -> String {
        apply_color(&self, color_config::RED)
    }

    fn blue(&self) -> String {
        apply_color(&self, color_config::BLUE)
    }

    fn dark_blue(&self) -> String {
        apply_color(&self, color_config::DARK_BLUE)
    }

    fn black(&self) -> String {
        apply_color(&self, color_config::BLACK)
    }

    fn white(&self) -> String {
        apply_color(&self, color_config::WHITE)
    }

    fn dark_white(&self) -> String {
        apply_color(&self, color_config::DARK_WHITE)
    }
}

pub trait PieceColors {
    fn white_pieces(&self) -> String;
    fn black_pieces(&self) -> String;
}

impl PieceColors for String {
    fn white_pieces(&self) -> Self {
        apply_color(&self, color_config::WHITE_PIECES)
    }

    fn black_pieces(&self) -> Self {
        apply_color(&self, color_config::BLACK_PIECES)
    }
}

impl PieceColors for &str {
    fn white_pieces(&self) -> String {
        apply_color(&self, color_config::WHITE_PIECES)
    }

    fn black_pieces(&self) -> String {
        apply_color(&self, color_config::BLACK_PIECES)
    }
}

pub fn heat_color(content: &str, value: f32, min_value: f32, max_value: f32) -> String {
    let difference = max_value - min_value;
    let min_value = min_value + difference * 0.1;
    let max_value = max_value - difference * 0.2;

    let value = value.max(min_value).min(max_value);
    let scalar = if min_value == max_value {
        0.5
    } else {
        (value - min_value) / (max_value - min_value)
    };

    if scalar >= 0.5 {
        lerp_color(
            content,
            color_config::DRAW_COLOR,
            color_config::WIN_COLOR,
            (scalar - 0.5) * 2.0,
        )
    } else {
        lerp_color(
            content,
            color_config::LOSE_COLOR,
            color_config::DRAW_COLOR,
            scalar * 2.0,
        )
    }
}

pub fn lerp_color(content: &str, a: (u8, u8, u8), b: (u8, u8, u8), value: f32) -> String {
    let (r, g, b) = lerp_color_internal(a, b, value);
    content.truecolor(r, g, b).to_string()
}

fn lerp_color_internal(a: (u8, u8, u8), b: (u8, u8, u8), value: f32) -> (u8, u8, u8) {
    let result_r = a.0 as i16 + ((b.0 as i16 - a.0 as i16) as f32 * value) as i16;
    let result_g = a.1 as i16 + ((b.1 as i16 - a.1 as i16) as f32 * value) as i16;
    let result_b = a.2 as i16 + ((b.2 as i16 - a.2 as i16) as f32 * value) as i16;
    (result_r as u8, result_g as u8, result_b as u8)
}

fn apply_color(content: &str, color: (u8, u8, u8)) -> String {
    content.truecolor(color.0, color.1, color.2).to_string()
}
