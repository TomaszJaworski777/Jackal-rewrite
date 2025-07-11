mod color_config;
mod color_utils;
mod terminal_utils;

pub use color_config::*;
pub use color_utils::heat_color;
pub use color_utils::lerp_color;
pub use color_utils::Colors;
pub use color_utils::Labels;
pub use color_utils::PieceColors;
pub use color_utils::CustomColor;
pub use terminal_utils::bytes_to_string;
pub use terminal_utils::clear_terminal_screen;
pub use terminal_utils::create_loading_bar;
pub use terminal_utils::miliseconds_to_string;
pub use terminal_utils::number_to_string;
pub use terminal_utils::seconds_to_string;
