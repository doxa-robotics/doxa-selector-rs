use std::{cell::RefCell, collections::HashMap, rc::Rc, time::Duration};

use buoyant::{
    transition::{Move, Slide},
    view::prelude::*,
};

use crate::{view::image, ExternalState, Route};

mod bottom_bar;
mod button;
mod calibrating_overlay;
mod card;
mod confirm_selection_screen;
mod diagnostics_screen;
mod select_category_screen;
mod select_route_screen;
mod selector;

/// Application data shared across views.
///
/// Will not change after initialization and is intended to be created once and
/// shared by reference for the duration of the program.
///
/// `Clone` is intentionally not implemented to make this shared-ownership model
/// explicit and to avoid accidentally duplicating the application data, which
/// is costly in terms of memory.
#[derive(Debug)]
pub(super) struct AppData {
    /// Vec<(category_index, category_name, category_index)>
    category_names: Vec<(usize, String, usize)>,
    /// (category_index) -> Vec<(route_index, route_name, global_route_index)>
    route_names_map: HashMap<usize, Vec<(usize, String, usize)>>,
    routes: Vec<(&'static str, &'static str)>,
}

impl AppData {
    pub fn new<C: crate::route::Category, R: 'static>(
        routes: Vec<Route<C, R>>,
        categories: Vec<C>,
    ) -> Self {
        let category_names = categories
            .iter()
            .enumerate()
            .map(|(i, c)| (i, c.to_string(), i))
            .collect();
        let mut route_names_map: HashMap<usize, Vec<(usize, String, usize)>> = HashMap::new();
        for (i, route) in routes.iter().enumerate() {
            let category_index = categories
                .iter()
                .position(|c| *c == route.category)
                .expect("Route category not found in categories list.");
            let entry = route_names_map.entry(category_index).or_default();
            entry.push((entry.len(), route.name.to_string(), i));
        }
        Self {
            category_names,
            route_names_map,
            routes: routes.iter().map(|r| (r.name, r.description)).collect(),
        }
    }
}

pub(super) struct AppState {
    /// Current screen
    pub screen: Screen,
    /// Cached diagnostics data
    diagnostics: Option<Vec<(String, String)>>,

    /// External state shared with the main DoxaSelect struct
    pub external: Rc<RefCell<ExternalState>>,
    /// Interface to the crate user
    pub interface: Box<dyn crate::DoxaSelectInterface>,
}

impl AppState {
    pub fn new(
        external: Rc<RefCell<ExternalState>>,
        interface: impl crate::DoxaSelectInterface + 'static,
    ) -> Self {
        Self {
            screen: Screen::default(),
            external,
            interface: Box::new(interface),
            diagnostics: None,
        }
    }

    fn refresh_diagnostics(&mut self) {
        let interface = &*self.interface;
        if interface.diagnostics_enable() {
            self.diagnostics = Some(interface.diagnostics_diagnostics());
        } else {
            self.diagnostics = None;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(super) enum Screen {
    #[default]
    SelectCategory,
    SelectRoute(usize),
    ConfirmSelection,
    Confirmed,
    Diagnostics(Box<Screen>),
}

pub(super) fn root_view<'a>(
    state: &AppState,
    data: &'a AppData,
) -> impl View<crate::view::color::Color, AppState> + use<'a> {
    ZStack::new((
        image::LOGO_CROPPED.as_ref().map(|img| {
            Image::new(img)
                .flex_frame()
                .with_alignment(Alignment::BottomTrailing)
                .with_infinite_max_height()
                .with_infinite_max_width()
        }),
        if matches!(state.screen, Screen::Confirmed) {
            image::SELECTED_BACKGROUND
                .as_ref()
                .map(|img| Image::new(img).flex_frame())
        } else {
            None
        }
        .transition(Slide::top()),
        VStack::new((
            // In principle, it would be better to use a match_view! here, but
            // because buoyant's implementation of OneOfN doesn't implement
            // animations correctly, we just stack everything and use
            // conditional rendering.
            ZStack::new((
                matches!(state.screen, Screen::SelectCategory).then(|| {
                    select_category_screen::select_category_screen(data).transition(Move::leading())
                }),
                match state.screen {
                    Screen::SelectRoute(category_index) => Some(
                        select_route_screen::select_route_screen(data, category_index)
                            .transition(Move::trailing()),
                    ),
                    _ => None,
                },
                matches!(state.screen, Screen::ConfirmSelection).then(|| {
                    confirm_selection_screen::confirm_selection_screen(
                        data,
                        state.external.borrow().selection,
                    )
                    .transition(Move::top())
                }),
                matches!(state.screen, Screen::Confirmed).then(|| {
                    EmptyView
                        .flex_frame()
                        .with_infinite_max_height()
                        .with_infinite_max_width()
                }),
                matches!(state.screen, Screen::Diagnostics(_)).then(|| {
                    diagnostics_screen::diagnostics_screen(state).transition(Move::bottom())
                }),
            )),
            bottom_bar::bottom_bar(state),
        )),
        calibrating_overlay::calibrating_overlay(state),
    ))
    .animated(
        Animation::ease_in_out(Duration::from_millis(400)),
        match state.screen {
            Screen::SelectCategory => 0,
            Screen::SelectRoute(_) => 1,
            Screen::ConfirmSelection => 2,
            Screen::Confirmed => 3,
            Screen::Diagnostics(_) => 4,
        },
    )
}
