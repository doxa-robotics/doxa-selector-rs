use std::time::Duration;

use buoyant::view::prelude::*;

use crate::view::{
    color, spacing,
    ui::{
        button::{self, ButtonStyle},
        toggle::toggle_text,
        AppState,
    },
};

pub fn bottom_bar(state: &AppState) -> impl View<color::Color, AppState> + use<> {
    HStack::new((
        Text::new("99484A DOXA Robotics", &*crate::view::font::CAPTION)
            .foreground_color(color::M3_ON_SURFACE),
        Text::new("Calibrating...", &*crate::view::font::CAPTION)
            .foreground_color(color::M3_ON_SURFACE),
        Spacer::default(),
        button::button("Brew", ButtonStyle::default(), |state: &mut AppState| {
            state.screen = crate::view::ui::Screen::Brew;
        }),
    ))
    .with_spacing(spacing::COMPONENT)
    .flex_infinite_width(HorizontalAlignment::Center)
    .padding(Edges::All, spacing::COMPONENT)
    .background_color(color::M3_SURFACE_CONTAINER_HIGHEST, Capsule)
    .padding(Edges::Horizontal, spacing::EDGE)
    .padding(Edges::Bottom, spacing::EDGE)
    .animated(Animation::linear(Duration::from_millis(200)), state.clone())
}
