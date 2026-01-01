use std::{cell::RefCell, rc::Rc, time::Duration};

use buoyant::{match_view, view::prelude::*};

use crate::{ExternalState, Route};

mod bottom_bar;
mod button;
mod calibrating_overlay;
mod card;
mod select_screen;

#[derive(Debug, Clone)]
pub(super) struct AppData<C: crate::route::Category, R: 'static> {
    routes: Vec<Route<C, R>>,
    categories: Vec<C>,
    category_names: Vec<(usize, String)>,
}

impl<C: crate::route::Category, R: 'static> AppData<C, R> {
    pub fn new(routes: Vec<Route<C, R>>, categories: Vec<C>) -> Self {
        let category_names = categories
            .iter()
            .enumerate()
            .map(|(i, c)| (i, c.to_string()))
            .collect();
        Self {
            routes,
            categories,
            category_names,
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct AppState {
    screen: Screen,

    pub external: Rc<RefCell<ExternalState>>,
}

impl AppState {
    pub fn new(external: Rc<RefCell<ExternalState>>) -> Self {
        Self {
            screen: Screen::default(),
            external,
        }
    }
}

impl AppState {
    pub fn calibrate(&mut self) {
        let self_external = self.external.clone();
        vexide::task::spawn(async move {
            self_external.borrow_mut().calibrating = true;
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
    SelectRoute(usize),
}

pub(super) fn root_view<'a, C: crate::route::Category, R: 'static>(
    state: &AppState,
    data: &'a AppData<C, R>,
) -> impl View<crate::view::color::Color, AppState> + use<'a, C, R> {
    ZStack::new((
        VStack::new((
            match_view!(state.screen, {
                Screen::SelectCategory => {
                    select_screen::select_screen(state, data)
                },
                Screen::SelectRoute(_category_index) => {
                    EmptyView
                }
            })
            .animated(
                Animation::ease_in_out(Duration::from_millis(400)),
                state.screen,
            ),
            bottom_bar::bottom_bar(state),
        )),
        calibrating_overlay::calibrating_overlay(state),
    ))
}
