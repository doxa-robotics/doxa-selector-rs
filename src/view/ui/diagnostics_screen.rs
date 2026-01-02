use buoyant::view::{
    prelude::ViewModifier, scroll_view::ScrollDirection, shape::RoundedRectangle, ForEach, HStack,
    ScrollView, Spacer, Text, VStack, View,
};

use crate::view::{
    color, spacing,
    ui::button::{self, ButtonStyle},
    AppState,
};

// This screen is very inefficient and bleeds memory, but it should be okay since
// diagnostics mode is only intended for short-term use not during a match.
//
// This is because we must return a static lifetime from the function, but the
// diagnostics data is dynamic.

pub fn diagnostics_screen(state: &crate::view::AppState) -> impl View<color::Color, AppState> {
    let diagnostics = state
        .diagnostics
        .clone()
        .expect("Diagnostics should be Some when in diagnostics screen");
    assert!(
        diagnostics.len() <= 16,
        "Diagnostics should be less or equal to than 16 lines"
    );

    let len = diagnostics.len();
    let compact = state.interface.diagnostics_compact();

    ScrollView::new(
        VStack::new((
            HStack::new((
                Text::new("Diagnostics", &*crate::view::font::HEADING)
                    .foreground_color(color::M3_ON_SURFACE),
                Spacer::default(),
                button::button("Refresh", ButtonStyle::default(), |state: &mut AppState| {
                    state.refresh_diagnostics();
                }),
            ))
            .with_spacing(spacing::ELEMENT),
            ForEach::<16>::new_vertical(
                // ForEach requires a static lifetime for items
                &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15][0..len],
                move |i| {
                    let (key, value) = diagnostics[*i as usize].clone();
                    HStack::new((
                        Text::new(
                            key,
                            if compact {
                                &*crate::view::font::CAPTION
                            } else {
                                &*crate::view::font::BODY
                            },
                        )
                        .foreground_color(color::M3_ON_SURFACE),
                        Spacer::default(),
                        Text::new(
                            value,
                            if compact {
                                &*crate::view::font::CAPTION
                            } else {
                                &*crate::view::font::BODY
                            },
                        )
                        .foreground_color(color::M3_ON_SURFACE_VARIANT),
                    ))
                    .padding(
                        buoyant::view::prelude::Edges::All,
                        if compact { 4 } else { spacing::ELEMENT },
                    )
                    .background_color(
                        color::M3_SURFACE_CONTAINER_HIGHEST,
                        RoundedRectangle::new(if compact { 8 } else { 12 }),
                    )
                },
            )
            .with_spacing(if compact { 2 } else { spacing::LIST_ITEM }),
        ))
        .with_spacing(spacing::ELEMENT)
        .padding(buoyant::view::prelude::Edges::All, spacing::SECTION_MARGIN)
        .flex_frame()
        .with_alignment(buoyant::layout::Alignment::Center),
    )
    .with_direction(ScrollDirection::Vertical)
    .with_overlapping_bar(true)
    .with_bar_visibility(if len > if compact { 5 } else { 3 } {
        buoyant::view::scroll_view::ScrollBarVisibility::Always
    } else {
        buoyant::view::scroll_view::ScrollBarVisibility::Never
    })
}
