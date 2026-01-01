use buoyant::{match_view, view::prelude::*};
use embedded_graphics::prelude::WebColors as _;

use crate::view::{color, font, spacing};

mod brew_tab;
mod settings_tab;
mod tab_bar;
mod toggle;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(super) struct AppState {
    pub tab: Tab,
    pub stop_on_weight: bool,
    pub auto_off: bool,
    pub auto_brew: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub(super) enum Tab {
    #[default]
    Brew,
    Clean,
    Settings,
}

pub(super) fn root_view(
    state: &AppState,
) -> impl View<crate::view::color::Color, AppState> + use<> {
    VStack::new((
        Lens::new(tab_bar::tab_bar(state.tab), |state: &mut AppState| {
            &mut state.tab
        }),
        match_view!(state.tab, {
            Tab::Brew => {
                brew_tab::brew_tab(state)
            },
            Tab::Clean => {
                Text::new("Clean", &*font::BODY)
                    .foreground_color(color::Color::CSS_ORANGE_RED)
                    .padding(Edges::All, spacing::SECTION_MARGIN)
            },
            Tab::Settings => {
                settings_tab::settings_tab(state)
            },
        }),
    ))
}
