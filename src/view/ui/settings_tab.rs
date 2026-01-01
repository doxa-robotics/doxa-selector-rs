use std::time::Duration;

use buoyant::view::prelude::*;

use crate::view::{
    color, spacing,
    ui::{toggle::toggle_text, AppState},
};

pub fn settings_tab(state: &AppState) -> impl View<color::Color, AppState> + use<> {
    ScrollView::new(
        VStack::new((
            toggle_text(
                "Auto brew",
                state.auto_brew,
                "Automatically brew coffee at 7am",
                true,
                |state: &mut AppState| {
                    state.auto_brew = !state.auto_brew;
                },
            ),
            toggle_text(
                "Stop on weight",
                state.stop_on_weight,
                "Stop the machine automatically when the target weight is reached",
                false,
                |state: &mut AppState| {
                    state.stop_on_weight = !state.stop_on_weight;
                },
            ),
            toggle_text(
                "Auto off",
                state.auto_off,
                "The display will go to sleep after 5 minutes of inactivity",
                true,
                |state: &mut AppState| {
                    state.auto_off = !state.auto_off;
                },
            ),
        ))
        .with_spacing(spacing::COMPONENT)
        .with_alignment(HorizontalAlignment::Trailing)
        .padding(Edges::All, spacing::SECTION_MARGIN)
        .animated(Animation::linear(Duration::from_millis(200)), state.clone()),
    )
    .with_overlapping_bar(true) // we already applied padding
}
