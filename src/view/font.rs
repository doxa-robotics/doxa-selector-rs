use std::sync::LazyLock;

pub static MONTSERRAT: LazyLock<rusttype::Font<'static>> = LazyLock::new(|| {
    // Include the font data at compile time
    // Note that Montserrat-Regular.ttf is actually a trimmed version of the
    // full font to reduce binary size
    rusttype::Font::try_from_bytes(include_bytes!("../../assets/Montserrat-Regular.ttf")).unwrap()
});

pub const SIZE_BODY: u32 = 24;
pub const SIZE_CAPTION: u32 = 18;
pub const SIZE_HEADING: u32 = 32;
