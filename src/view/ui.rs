use std::{cell::RefCell, rc::Rc};

use buoyant::{match_view, view::prelude::*};

use crate::ExternalState;

mod bottom_bar;
mod button;
mod calibrating_overlay;
mod select_screen;

#[derive(Debug, Clone)]
pub(super) struct AppState<C: crate::route::Category, R: 'static> {
    pub screen: Screen,

    pub external: Rc<RefCell<ExternalState<C, R>>>,
}

impl<C: crate::route::Category, R> AppState<C, R> {
    pub fn new(external: Rc<RefCell<ExternalState<C, R>>>) -> Self {
        Self {
            screen: Screen::default(),
            external,
        }
    }
}

impl<C: crate::route::Category, R> AppState<C, R> {
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

pub(super) fn root_view<C: crate::route::Category, R>(
    state: &AppState<C, R>,
) -> impl View<crate::view::color::Color, AppState<C, R>> {
    ZStack::new((
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
        )),
        calibrating_overlay::calibrating_overlay(state),
    ))
}
