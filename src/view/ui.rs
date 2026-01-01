use std::{cell::RefCell, rc::Rc};

use buoyant::{match_view, view::prelude::*};
use embedded_graphics::prelude::WebColors as _;

use crate::view::{color, font, spacing};

mod bottom_bar;
mod button;
mod select_screen;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(super) struct AppState {
    pub screen: Screen,

    pub external: Rc<RefCell<ExternalAppState>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ExternalAppState {
    pub calibrating: bool,
}

impl AppState {
    pub fn calibrate(&mut self) {
        self.external.borrow_mut().calibrating = true;
        let self_external = self.external.clone();
        vexide::task::spawn(async move {
            vexide::time::sleep(std::time::Duration::from_secs(2)).await;
            self_external.borrow_mut().calibrating = false;
        })
        .detach();
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub(super) enum Screen {
    #[default]
    SelectCategory,
    SelectRoute,
}

pub(super) fn root_view(
    state: &AppState,
) -> impl View<crate::view::color::Color, AppState> + use<> {
    VStack::new((
        match_view!(state.screen, {
            Screen::SelectCategory => {
                select_screen::select_screen(state)
            },
            Screen::SelectRoute => {
                select_screen::select_screen(state)
            }
        }),
        bottom_bar::bottom_bar(state),
    ))
}
