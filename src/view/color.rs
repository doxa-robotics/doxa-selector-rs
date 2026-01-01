use embedded_graphics::prelude::*;

/// Use this alias instead of directly referring to a specific `embedded_graphics`
/// color type to allow portability between displays
pub type Color = embedded_graphics::pixelcolor::Rgb888;
pub const ACCENT: Color = Color::CSS_LIGHT_SKY_BLUE;
pub const BACKGROUND: Color = Color::BLACK;
pub const BACKGROUND_SECONDARY: Color = Color::CSS_DARK_SLATE_GRAY;
pub const FOREGROUND_SECONDARY: Color = Color::CSS_LIGHT_SLATE_GRAY;
