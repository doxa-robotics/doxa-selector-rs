use std::time::Duration;

use buoyant::{transition::Move, view::prelude::*};

use crate::view::{color, font, spacing, ui::AppState};

pub fn calibrating_overlay(state: &AppState) -> impl View<color::Color, AppState> {
    let calibrating = state.external.borrow().calibrating;
    calibrating
        // If calibrating is true, then render
        .then(|| {
            RoundedRectangle::new(32)
                .foreground_color(color::M3_ERROR_CONTAINER)
                .padding(Edges::All, 48)
                .overlay(
                    Alignment::Center,
                    VStack::new((
                        Text::new("Calibrating...", &*font::MONTSERRAT)
                            .with_font_size(font::SIZE_HEADING)
                            .foreground_color(color::M3_ON_ERROR_CONTAINER)
                            .hint_background_color(color::M3_ERROR_CONTAINER),
                        Text::new("Please do not move the robot.", &*font::MONTSERRAT)
                            .with_font_size(font::SIZE_BODY)
                            .foreground_color(color::M3_ON_SURFACE_VARIANT)
                            .hint_background_color(color::M3_ERROR_CONTAINER),
                    ))
                    .with_spacing(spacing::ELEMENT),
                )
                .transition(Move::bottom())
        })
        .animated(
            Animation::ease_in_out(Duration::from_millis(800)),
            calibrating,
        )
}
