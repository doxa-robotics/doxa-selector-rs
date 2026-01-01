use std::sync::LazyLock;

use embedded_ttf::{FontTextStyle, FontTextStyleBuilder};

use super::color;

pub static MONTSERRAT_FONT: LazyLock<rusttype::Font<'static>> = LazyLock::new(|| {
    // Include the font data at compile time
    // Note that Montserrat-Regular.ttf is actually a trimmed version of the
    // full font to reduce binary size
    rusttype::Font::try_from_bytes(include_bytes!("../../assets/Montserrat-Regular.ttf")).unwrap()
});

/// Font for body text
pub static BODY: LazyLock<FontTextStyle<color::Color>> = LazyLock::new(|| {
    FontTextStyleBuilder::new((*MONTSERRAT_FONT).clone())
        .font_size(24)
        .build()
});
/// Font for captions and smaller text
pub static CAPTION: LazyLock<FontTextStyle<color::Color>> = LazyLock::new(|| {
    FontTextStyleBuilder::new((*MONTSERRAT_FONT).clone())
        .font_size(18)
        .build()
});
/// Font for headings
pub static HEADING: LazyLock<FontTextStyle<color::Color>> = LazyLock::new(|| {
    FontTextStyleBuilder::new((*MONTSERRAT_FONT).clone())
        .font_size(32)
        .build()
});
