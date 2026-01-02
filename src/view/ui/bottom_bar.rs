use buoyant::{match_view, view::prelude::*};

use crate::view::{
    color, font, spacing,
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
            _ => Text::new("99484A DOXA Robotics", &*font::MONTSERRAT)
                .with_font_size(font::SIZE_CAPTION)
                .foreground_color(color::M3_ON_SURFACE),
        }),
        Spacer::default(),
        state.interface.calibrating_enable().then(|| {
            button::button(
                "Calibrate",
                ButtonStyle::default(),
                |state: &mut AppState| {
                    state.interface.calibrating_calibrate();
                },
            )
        }),
        state.interface.diagnostics_enable().then(|| {
            button::button(
                match state.screen {
                    Screen::Diagnostics(_) => "Exit diagnostics",
                    _ => "Diagnostics",
                },
                match state.screen {
                    Screen::Diagnostics(_) => ButtonStyle::filled(),
                    _ => ButtonStyle::default(),
                },
                |state: &mut AppState| {
                    state.screen = match &state.screen {
                        Screen::Diagnostics(previous_screen) => {
                            // If already in diagnostics, go back to previous screen
                            *previous_screen.clone()
                        }
                        screen => {
                            // Otherwise, go to diagnostics, saving current screen
                            let screen = screen.clone();
                            state.refresh_diagnostics();
                            crate::view::ui::Screen::Diagnostics(Box::new(screen))
                        }
                    }
                },
            )
        }),
    ))
    .with_spacing(spacing::COMPONENT)
    .flex_infinite_width(HorizontalAlignment::Center)
    .padding(Edges::All, spacing::COMPONENT)
    .background_color(color::M3_SURFACE_CONTAINER_HIGHEST, Capsule)
    .padding(Edges::Horizontal, spacing::EDGE)
    .padding(Edges::Bottom, spacing::EDGE)
    .geometry_group()
}
