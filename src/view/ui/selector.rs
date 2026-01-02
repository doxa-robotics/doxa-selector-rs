use buoyant::{
    match_view,
    view::{prelude::*, scroll_view::ScrollDirection},
};
use embedded_graphics::prelude::RgbColor as _;

use crate::view::{color, font, spacing};

const CARD_HEIGHT: u32 = 56;

pub fn selector<'a, C: 'a, OnSelectFn>(
    title: &'a str,
    items: &'a [(usize, String, usize)],
    on_select: OnSelectFn,
) -> impl View<color::Color, C> + use<'a, C, OnSelectFn>
where
    OnSelectFn: Fn(&mut C, usize) + 'a + Copy,
{
    ScrollView::new(
        VStack::new((
            Text::new(title, &*font::MONTSERRAT).with_font_size(font::SIZE_HEADING),
            ForEach::<12>::new_vertical(
                // It's a bit hacky, but it works.
                // To split the categories into two columns, we tell buoyant that
                // there are only half the number of categories, and then we index
                // into the original list to get the correct category names.
                &items[0..(items.len().div_ceil(2))],
                move |(index, _, _): &(usize, String, usize)| {
                    let category_name_1 = &items[*index * 2].1;
                    // We're not guaranteed to have a second category if the number of categories is odd
                    let category_name_2 = items.get(*index * 2 + 1).map(|x| &x.1);

                    HStack::new((
                        crate::view::ui::card::card(
                            category_name_1,
                            crate::view::ui::card::CardStyle::default(),
                            move |state: &mut C| {
                                on_select(state, *index * 2);
                            },
                        )
                        .flex_frame()
                        .with_ideal_height(CARD_HEIGHT),
                        match_view!(category_name_2, {
                            Some(category_name_2) => {
                                let card_2 = crate::view::ui::card::card(
                                    category_name_2,
                                    crate::view::ui::card::CardStyle::default(),
                                    move |state: &mut C| {
                                        on_select(state, *index * 2 + 1);
                                    },
                                )
                                .flex_frame()
                                .with_ideal_height(CARD_HEIGHT);
                                card_2
                            },
                            None => {
                                EmptyView
                            },
                        }),
                    ))
                    .with_spacing(spacing::COMPONENT)
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
    .with_bar_visibility(if items.len() > 4 {
        buoyant::view::scroll_view::ScrollBarVisibility::Always
    } else {
        buoyant::view::scroll_view::ScrollBarVisibility::Never
    })
}
