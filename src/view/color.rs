#![allow(dead_code)]

/// Use this alias instead of directly referring to a specific `embedded_graphics`
/// color type to allow portability between displays
pub type Color = embedded_graphics::pixelcolor::Rgb888;

// M3 Color Palette
pub const M3_PRIMARY: Color = Color::new(0xB4, 0xC5, 0xFF);
pub const M3_SURFACE_TINT: Color = Color::new(0xB4, 0xC5, 0xFF);
pub const M3_ON_PRIMARY: Color = Color::new(0x1B, 0x2D, 0x60);
pub const M3_PRIMARY_CONTAINER: Color = Color::new(0x33, 0x44, 0x78);
pub const M3_ON_PRIMARY_CONTAINER: Color = Color::new(0xDB, 0xE1, 0xFF);
pub const M3_SECONDARY: Color = Color::new(0xC1, 0xC5, 0xDD);
pub const M3_ON_SECONDARY: Color = Color::new(0x2B, 0x30, 0x42);
pub const M3_SECONDARY_CONTAINER: Color = Color::new(0x41, 0x46, 0x59);
pub const M3_ON_SECONDARY_CONTAINER: Color = Color::new(0xDD, 0xE1, 0xF9);
pub const M3_TERTIARY: Color = Color::new(0xE2, 0xBB, 0xDB);
pub const M3_ON_TERTIARY: Color = Color::new(0x42, 0x27, 0x40);
pub const M3_TERTIARY_CONTAINER: Color = Color::new(0x5B, 0x3D, 0x58);
pub const M3_ON_TERTIARY_CONTAINER: Color = Color::new(0xFF, 0xD6, 0xF7);
pub const M3_ERROR: Color = Color::new(0xFF, 0xB4, 0xAB);
pub const M3_ON_ERROR: Color = Color::new(0x69, 0x00, 0x05);
pub const M3_ERROR_CONTAINER: Color = Color::new(0x93, 0x00, 0x0A);
pub const M3_ON_ERROR_CONTAINER: Color = Color::new(0xFF, 0xDA, 0xD6);
pub const M3_BACKGROUND: Color = Color::new(0x12, 0x13, 0x18);
pub const M3_ON_BACKGROUND: Color = Color::new(0xE3, 0xE2, 0xE9);
pub const M3_SURFACE: Color = Color::new(0x12, 0x13, 0x18);
pub const M3_ON_SURFACE: Color = Color::new(0xE3, 0xE2, 0xE9);
pub const M3_SURFACE_VARIANT: Color = Color::new(0x45, 0x46, 0x4F);
pub const M3_ON_SURFACE_VARIANT: Color = Color::new(0xC5, 0xC6, 0xD0);
pub const M3_OUTLINE: Color = Color::new(0x8F, 0x90, 0x9A);
pub const M3_OUTLINE_VARIANT: Color = Color::new(0x45, 0x46, 0x4F);
pub const M3_SHADOW: Color = Color::new(0x00, 0x00, 0x00);
pub const M3_SCRIM: Color = Color::new(0x00, 0x00, 0x00);
pub const M3_INVERSE_SURFACE: Color = Color::new(0xE3, 0xE2, 0xE9);
pub const M3_INVERSE_ON_SURFACE: Color = Color::new(0x2F, 0x30, 0x36);
pub const M3_INVERSE_PRIMARY: Color = Color::new(0x4B, 0x5C, 0x92);
pub const M3_PRIMARY_FIXED: Color = Color::new(0xDB, 0xE1, 0xFF);
pub const M3_ON_PRIMARY_FIXED: Color = Color::new(0x01, 0x17, 0x4B);
pub const M3_PRIMARY_FIXED_DIM: Color = Color::new(0xB4, 0xC5, 0xFF);
pub const M3_ON_PRIMARY_FIXED_VARIANT: Color = Color::new(0x33, 0x44, 0x78);
pub const M3_SECONDARY_FIXED: Color = Color::new(0xDD, 0xE1, 0xF9);
pub const M3_ON_SECONDARY_FIXED: Color = Color::new(0x16, 0x1B, 0x2C);
pub const M3_SECONDARY_FIXED_DIM: Color = Color::new(0xC1, 0xC5, 0xDD);
pub const M3_ON_SECONDARY_FIXED_VARIANT: Color = Color::new(0x41, 0x46, 0x59);
pub const M3_TERTIARY_FIXED: Color = Color::new(0xFF, 0xD6, 0xF7);
pub const M3_ON_TERTIARY_FIXED: Color = Color::new(0x2B, 0x12, 0x2A);
pub const M3_TERTIARY_FIXED_DIM: Color = Color::new(0xE2, 0xBB, 0xDB);
pub const M3_ON_TERTIARY_FIXED_VARIANT: Color = Color::new(0x5B, 0x3D, 0x58);
pub const M3_SURFACE_DIM: Color = Color::new(0x12, 0x13, 0x18);
pub const M3_SURFACE_BRIGHT: Color = Color::new(0x38, 0x39, 0x3F);
pub const M3_SURFACE_CONTAINER_LOWEST: Color = Color::new(0x0D, 0x0E, 0x13);
pub const M3_SURFACE_CONTAINER_LOW: Color = Color::new(0x1A, 0x1B, 0x21);
pub const M3_SURFACE_CONTAINER: Color = Color::new(0x1E, 0x1F, 0x25);
pub const M3_SURFACE_CONTAINER_HIGH: Color = Color::new(0x29, 0x2A, 0x2F);
pub const M3_SURFACE_CONTAINER_HIGHEST: Color = Color::new(0x34, 0x34, 0x3A);
