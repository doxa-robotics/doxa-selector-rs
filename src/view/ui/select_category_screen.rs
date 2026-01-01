use buoyant::view::View;

use crate::{
    view::{color, AppState},
    Category,
};

pub fn select_category_screen<C: Category, R>(
    data: &crate::view::AppData<C, R>,
) -> impl View<color::Color, AppState> + use<'_, C, R> {
    super::selector::selector(
        "Select category",
        &data.category_names,
        move |state: &mut AppState, category_index: usize| {
            state.screen = crate::view::ui::Screen::SelectRoute(category_index);
        },
    )
}
