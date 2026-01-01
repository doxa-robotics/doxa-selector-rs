use buoyant::{
    match_view,
    view::{prelude::*, scroll_view::ScrollDirection},
};
use embedded_graphics::prelude::RgbColor as _;

use crate::view::{
    color, font, spacing,
    ui::{AppData, AppState},
};

const CARD_HEIGHT: u32 = 56;

pub fn select_screen<'b, C: crate::Category, R: 'static>(
    _state: &AppState,
    data: &'b AppData<C, R>,
) -> impl View<color::Color, AppState> + use<'b, C, R> {
    ScrollView::new(
        VStack::new((
            Text::new("Select category", &*font::HEADING),
            ForEach::<12>::new_vertical(
                // It's a bit hacky, but it works.
                // To split the categories into two columns, we tell buoyant that
                // there are only half the number of categories, and then we index
                // into the original list to get the correct category names.
                &data.category_names[0..(data.category_names.len().div_ceil(2))],
                move |(index, _): &(usize, String)| {
                    let category_name_1 = &data.category_names[*index * 2].1;
                    // We're not guaranteed to have a second category if the number of categories is odd
                    let category_name_2 = data
                        .category_names
                        .get(*index * 2 + 1)
                        .map(|(_, name)| name);

                    HStack::new((
                        crate::view::ui::card::card(
                            category_name_1,
                            crate::view::ui::card::CardStyle::default(),
                            move |state: &mut AppState| {
                                state.screen = crate::view::ui::Screen::SelectRoute(*index * 2);
                            },
                        )
                        .flex_frame()
                        .with_ideal_height(CARD_HEIGHT), 
                        match_view!(category_name_2, {
                            Some(category_name_2) => {
                                let card_2 = crate::view::ui::card::card(
                                    category_name_2,
                                    crate::view::ui::card::CardStyle::default(),
                                    move |state: &mut AppState| {
                                        state.screen = crate::view::ui::Screen::SelectRoute(*index * 2 + 1);
                                    },
                                )
                                .flex_frame()
                                .with_ideal_height(CARD_HEIGHT);
                                card_2
                            },
                            None => {
                                EmptyView
                            },
                        })
                    )).with_spacing(spacing::COMPONENT)
                },
            )
            .with_spacing(spacing::COMPONENT),
        ))
        .with_spacing(spacing::COMPONENT)
        .with_alignment(HorizontalAlignment::Center)
        .padding(Edges::All, spacing::SECTION_MARGIN)
        .foreground_color(color::Color::WHITE),
    )
    .with_direction(ScrollDirection::Vertical)
    .with_overlapping_bar(true)
    .with_bar_visibility(if data.category_names.len() > 4 {
        buoyant::view::scroll_view::ScrollBarVisibility::Always
    } else {
        buoyant::view::scroll_view::ScrollBarVisibility::Never
    })
}
