use buoyant::view::View;

use crate::view::{color, AppState};

pub fn select_route_screen(
    data: &crate::view::AppData,
    category_index: usize,
) -> impl View<color::Color, AppState> + use<'_> {
    let routes = &data.route_names_map[&category_index];
    super::selector::selector(
        "Select route",
        routes,
        move |state: &mut AppState, route_index: usize| {
            let global_route_index = routes[route_index].2;
            println!("Selected route index: {}", global_route_index);
            state.external.borrow_mut().selection = global_route_index;
            state.screen = crate::view::ui::Screen::ConfirmSelection;
        },
    )
}
