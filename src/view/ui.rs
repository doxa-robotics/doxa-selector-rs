use buoyant::{match_view, view::prelude::*};
use embedded_graphics::prelude::WebColors as _;

use crate::view::{color, font, spacing};

mod bottom_bar;
mod brew_tab;
mod button;
mod settings_tab;
mod toggle;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(super) struct AppState {
    pub screen: Screen,
    pub stop_on_weight: bool,
    pub auto_off: bool,
    pub auto_brew: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub(super) enum Screen {
    #[default]
    Brew,
    Clean,
    Settings,
}

pub(super) fn root_view(
    state: &AppState,
) -> impl View<crate::view::color::Color, AppState> + use<> {
    VStack::new((
        match_view!(state.screen, {
            Screen::Brew => {
                brew_tab::brew_tab(state)
            },
            Screen::Clean => {
                Text::new("Clean", &*font::BODY)
                    .foreground_color(color::Color::CSS_ORANGE_RED)
                    .padding(Edges::All, spacing::SECTION_MARGIN)
            },
            Screen::Settings => {
                settings_tab::settings_tab(state)
            },
        }),
        bottom_bar::bottom_bar(state),
    ))
}
