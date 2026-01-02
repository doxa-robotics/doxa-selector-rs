//! Touchscreen-based autonomous route selector for VEX V5 robots.
#![feature(trait_alias)]
#![feature(never_type)]

use std::{cell::RefCell, rc::Rc};

use autons::Selector;
use vexide::{
    display::Display,
    task::{self, Task},
};

mod driver;
mod route;
mod view;

pub use route::*;

/// External state shared between the selector's UI and logic.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
struct ExternalState {
    calibrating: bool,
    show_calibrating: bool,
    show_diagnostics: bool,
    selection: usize,
}

pub trait DoxaSelectInterface {
    fn calibrating_enable(&self) -> bool {
        false
    }
    fn calibrating_calibrate(&mut self) {}
    fn calibrating_calibrating(&self) -> Rc<RefCell<bool>> {
        Rc::new(RefCell::new(false))
    }

    fn diagnostics_enable(&self) -> bool {
        false
    }
    /// Returns a list of key-value pairs representing diagnostics data.
    ///
    /// There is a maximum of 16 entries.
    fn diagnostics_diagnostics(&self) -> Vec<(String, String)> {
        Vec::new()
    }
}

/// Simple touchscreen-based autonomous route selector.
///
/// `SimpleSelect` is a barebones and lightweight autonomous selector that allows picking
/// between up to 16 autonomous routes using the V5 brain's display and touchscreen.
///
/// The selector provides a user interface that mimicks the appearance of other VEXos
/// dashboards, with basic support for color themes through the [`SimpleSelect::new_with_theme`]
/// function.
///
/// This struct implements the [`Selector`] trait and should be used with the [`SelectCompete`]
/// trait if using vexide's competition runtime.
///
/// [`SelectCompete`]: crate::compete::SelectCompete
pub struct DoxaSelect<C: Category, R: 'static> {
    state: Rc<RefCell<ExternalState>>,
    routes: Vec<Route<C, R>>,
    _task: Task<()>,
}

impl<C: Category, R> DoxaSelect<C, R> {
    /// Creates a new selector from a [`Display`] peripheral and array of routes.
    pub fn new<const N: usize>(
        display: Display,
        routes: [Route<C, R>; N],
        interface: impl DoxaSelectInterface + 'static,
    ) -> Self {
        const {
            assert!(N > 0, "DoxaSelect requires at least one route.");
        }

        let categories = {
            let mut cats = routes
                .iter()
                .map(|route| route.category)
                .collect::<Vec<_>>();
            cats.sort_unstable();
            cats.dedup();
            cats
        };

        let state = Rc::new(RefCell::new(ExternalState {
            selection: 0,
            show_diagnostics: interface.diagnostics_enable(),
            show_calibrating: interface.calibrating_enable(),
            calibrating: *interface.calibrating_calibrating().borrow(),
        }));

        Self {
            state: state.clone(),
            routes: routes.to_vec(),
            _task: task::spawn(async move {
                view::run(display, state, interface, routes.to_vec(), categories).await;
            }),
        }
    }

    /// Programmatically selects an autonomous route by index.
    pub fn select(&mut self, index: usize) {
        let mut state = self.state.borrow_mut();
        state.selection = index;
    }
}

impl<C: Category, R> Selector<R> for DoxaSelect<C, R> {
    async fn run(&self, robot: &mut R) {
        {
            let state = self.state.borrow();
            (self.routes[state.selection].callback)(robot)
        }
        .await;
    }
}
