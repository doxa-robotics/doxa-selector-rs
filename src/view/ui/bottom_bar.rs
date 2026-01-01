use std::time::Duration;

use buoyant::view::prelude::*;

use crate::view::{
    color, spacing,
    ui::{
        button::{self, ButtonStyle},
        AppState,
    },
};

pub fn bottom_bar<C: crate::Category, R: 'static>(
    state: &AppState<C, R>,
) -> impl View<color::Color, AppState<C, R>> {
    HStack::new((
        Text::new("99484A DOXA Robotics", &*crate::view::font::CAPTION)
            .foreground_color(color::M3_ON_SURFACE),
        Spacer::default(),
        button::button(
            "Recalibrate",
            ButtonStyle::default(),
            |state: &mut AppState<C, R>| {
                state.calibrate();
            },
        ),
        button::button(
            "Diagnostics",
            ButtonStyle::default(),
            |state: &mut AppState<C, R>| {
                state.screen = crate::view::ui::Screen::SelectRoute;
            },
        ),
    ))
    .with_spacing(spacing::COMPONENT)
    .flex_infinite_width(HorizontalAlignment::Center)
    .padding(Edges::All, spacing::COMPONENT)
    .background_color(color::M3_SURFACE_CONTAINER_HIGHEST, Capsule)
    .padding(Edges::Horizontal, spacing::EDGE)
    .padding(Edges::Bottom, spacing::EDGE)
    .animated(
        Animation::linear(Duration::from_millis(800)),
        state.external.borrow().calibrating,
    )
}
