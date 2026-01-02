use buoyant::view::{prelude::ViewModifier, shape::RoundedRectangle, HStack, Text, VStack, View};

use crate::view::{
    color, spacing,
    ui::button::{self, ButtonStyle},
    AppState,
};

pub fn confirm_selection_screen(
    data: &crate::view::AppData,
    route_index: usize,
) -> impl View<color::Color, AppState> + use<'_> {
    let (route_name, route_description) = data.routes[route_index];

    VStack::new((
        Text::new("Confirm selection", &*crate::view::font::HEADING)
            .foreground_color(color::M3_ON_SURFACE),
        VStack::new((
            Text::new(route_name, &*crate::view::font::BODY)
                .multiline_text_alignment(buoyant::view::HorizontalTextAlignment::Center)
                .foreground_color(color::M3_ON_SURFACE),
            Text::new(route_description, &*crate::view::font::CAPTION)
                .multiline_text_alignment(buoyant::view::HorizontalTextAlignment::Center)
                .foreground_color(color::M3_ON_SURFACE_VARIANT),
        ))
        .padding(buoyant::view::prelude::Edges::All, 12)
        .flex_infinite_height(buoyant::layout::VerticalAlignment::Center)
        .background_color(
            color::M3_SURFACE_CONTAINER_HIGHEST,
            RoundedRectangle::new(16),
        ),
        HStack::new((
            button::button("Cancel", ButtonStyle::large(), |state: &mut AppState| {
                state.screen = crate::view::ui::Screen::SelectCategory;
            }),
            button::button(
                "Confirm",
                ButtonStyle::filled_large(),
                |state: &mut AppState| {
                    state.screen = crate::view::ui::Screen::Confirmed;
                },
            ),
        ))
        .with_spacing(spacing::ELEMENT),
    ))
    .with_spacing(8)
    .padding(buoyant::view::prelude::Edges::All, spacing::ELEMENT)
    .flex_frame()
    .with_alignment(buoyant::layout::Alignment::Center)
}
