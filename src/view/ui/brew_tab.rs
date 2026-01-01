use buoyant::view::{prelude::*, scroll_view::ScrollDirection};
use embedded_graphics::prelude::RgbColor as _;

use crate::view::{color, font, spacing, ui::AppState};

pub fn brew_tab<C>(_state: &AppState) -> impl View<color::Color, C> + use<C> {
    ScrollView::new(
        VStack::new((
            Text::new("Good morning", &*font::HEADING),
            Text::new(
                "You can't brew coffee in a simulator, but you can pretend.",
                &*font::BODY,
            )
            .multiline_text_alignment(HorizontalTextAlignment::Center),
        ))
        .with_spacing(spacing::COMPONENT)
        .with_alignment(HorizontalAlignment::Center)
        .flex_infinite_width(HorizontalAlignment::Center)
        .padding(Edges::All, spacing::SECTION_MARGIN)
        .foreground_color(color::Color::WHITE),
    )
    .with_direction(ScrollDirection::Both)
}
