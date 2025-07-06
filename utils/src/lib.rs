mod terminal_utils;
mod color_utils;
mod color_config;

pub use terminal_utils::clear_terminal_screen;
pub use terminal_utils::create_loading_bar;
pub use terminal_utils::time_to_string;
pub use terminal_utils::number_to_string;
pub use terminal_utils::bytes_to_string;
pub use color_utils::Labels;
pub use color_utils::Colors;
pub use color_utils::PieceColors;
pub use color_utils::heat_color;
pub use color_utils::lerp_color;
pub use color_config::*;