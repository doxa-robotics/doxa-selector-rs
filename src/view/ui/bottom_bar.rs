use buoyant::{match_view, view::prelude::*};

use crate::view::{
    color, spacing,
    ui::{
        button::{self, ButtonStyle},
        AppState, Screen,
    },
};

pub fn bottom_bar(state: &AppState) -> impl View<color::Color, AppState> {
    HStack::new((
        match_view!(state.screen, {
            Screen::Confirmed => button::button(
                "Change route",
                ButtonStyle::default(),
                |state: &mut AppState| {
                    state.screen = crate::view::ui::Screen::SelectCategory;
                },
            ),
            _ => Text::new("99484A DOXA Robotics", &*crate::view::font::CAPTION)
                .foreground_color(color::M3_ON_SURFACE),
        }),
        Spacer::default(),
        button::button(
            "Recalibrate",
            ButtonStyle::default(),
            |state: &mut AppState| {
                state.calibrate();
            },
        ),
        button::button(
            "Diagnostics",
            ButtonStyle::default(),
            |state: &mut AppState| {
                state.screen = crate::view::ui::Screen::Diagnostics;
            },
        ),
    ))
    .with_spacing(spacing::COMPONENT)
    .flex_infinite_width(HorizontalAlignment::Center)
    .padding(Edges::All, spacing::COMPONENT)
    .background_color(color::M3_SURFACE_CONTAINER_HIGHEST, Capsule)
    .padding(Edges::Horizontal, spacing::EDGE)
    .padding(Edges::Bottom, spacing::EDGE)
}
