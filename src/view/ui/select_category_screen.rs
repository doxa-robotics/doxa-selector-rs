use buoyant::view::View;

use crate::view::{color, AppState};

pub fn select_category_screen(
    data: &crate::view::AppData,
) -> impl View<color::Color, AppState> + use<'_> {
    super::selector::selector(
        "Select category",
        &data.category_names,
        move |state: &mut AppState, category_index: usize| {
            state.screen = crate::view::ui::Screen::SelectRoute(category_index);
        },
    )
}
