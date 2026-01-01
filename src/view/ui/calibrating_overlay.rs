use std::time::Duration;

use buoyant::{render::Rect, view::prelude::*};
use vexide::prelude::Display;

use crate::view::{
    color, spacing,
    ui::{
        button::{self, ButtonStyle},
        AppState,
    },
};

pub fn calibrating_overlay(state: &AppState) -> impl View<color::Color, AppState> {
    // TODO: we always render this, even when not calibrating. Optimize?
    let calibrating = state.external.borrow().calibrating;
    RoundedRectangle::new(32)
        .foreground_color(color::M3_ERROR_CONTAINER)
        .padding(Edges::All, 48)
        .overlay(
            Alignment::Center,
            VStack::new((
                Text::new("Calibrating...", &*crate::view::font::HEADING)
                    .foreground_color(color::M3_ON_ERROR_CONTAINER)
                    .hint_background_color(color::M3_ERROR_CONTAINER),
                Text::new("Please do not move the robot.", &*crate::view::font::BODY)
                    .foreground_color(color::M3_ON_SURFACE_VARIANT)
                    .hint_background_color(color::M3_ERROR_CONTAINER),
            ))
            .with_spacing(spacing::ELEMENT),
        )
        .offset(
            0,
            if calibrating {
                0
            } else {
                Display::VERTICAL_RESOLUTION as i32
            },
        )
        .animated(
            Animation::ease_in_out(Duration::from_millis(800)),
            calibrating,
        )
}
