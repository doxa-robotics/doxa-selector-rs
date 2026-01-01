use buoyant::view::{prelude::*, scroll_view::ScrollDirection};
use embedded_graphics::prelude::RgbColor as _;

use crate::view::{
    color, font, spacing,
    ui::{AppData, AppState},
};

pub fn select_screen<'b, C: crate::Category, R: 'static>(
    state: &AppState,
    data: &'b AppData<C, R>,
) -> impl View<color::Color, AppState> + use<'b, C, R> {
    ScrollView::new(
        VStack::new((
            Text::new("Select category", &*font::HEADING),
            ForEach::<12>::new_vertical(
                &data.category_names,
                |(index, category_name): &(usize, String)| {
                    crate::view::ui::card::card(
                        category_name,
                        crate::view::ui::card::CardStyle::default(),
                        move |state: &mut AppState| {
                            // TODO: we need to use a Lens to correctly update the state to the right category
                            state.screen = crate::view::ui::Screen::SelectRoute;
                        },
                    )
                },
            ),
        ))
        .with_spacing(spacing::COMPONENT)
        .with_alignment(HorizontalAlignment::Center)
        .padding(Edges::All, spacing::SECTION_MARGIN)
        .foreground_color(color::Color::WHITE),
    )
    .with_direction(ScrollDirection::Vertical)
    .with_overlapping_bar(true)
}
