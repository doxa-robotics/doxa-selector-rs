use buoyant::view::{prelude::ViewModifier, EmptyView, VStack, View, ZStack};

use crate::view::{
    color,
    ui::button::{self, ButtonStyle},
    AppState,
};

pub fn select_route_screen(
    data: &crate::view::AppData,
    category_index: usize,
) -> impl View<color::Color, AppState> + use<'_> {
    let routes = &data.route_names_map[&category_index];

    ZStack::new((
        VStack::new((
            button::button("Back", ButtonStyle::default(), |state: &mut AppState| {
                state.screen = crate::view::ui::Screen::SelectCategory
            })
            .padding(buoyant::view::prelude::Edges::All, 8),
            EmptyView,
        ))
        .flex_frame()
        .with_infinite_max_height()
        .with_infinite_max_width()
        .with_alignment(buoyant::layout::Alignment::TopLeading),
        super::selector::selector(
            "Select route",
            routes,
            move |state: &mut AppState, route_index: usize| {
                let global_route_index = routes[route_index].2;
                state.external.borrow_mut().selection = global_route_index;
                state.screen = crate::view::ui::Screen::ConfirmSelection;
            },
        ),
    ))
}
