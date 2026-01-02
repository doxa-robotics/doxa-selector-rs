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
    selection: usize,
}

pub trait DoxaSelectInterface {
    /// Whether the calibration feature is enabled.
    ///
    /// If true, a "Calibrate" button will be shown in the UI. You must implement
    /// the calibration logic in `calibrating_calibrate` and `calibrating_calibrating`
    /// if this returns true.
    fn calibrating_enable(&self) -> bool {
        false
    }
    /// Starts the calibration process.
    fn calibrating_calibrate(&mut self) {
        panic!("You must implement calibrating_calibrate if calibrating_enable returns true.");
    }
    /// Returns a Rc<RefCell<bool>> that indicates whether calibration is in progress.
    fn calibrating_calibrating(&self) -> Rc<RefCell<bool>> {
        panic!("You must implement calibrating_calibrating if calibrating_enable returns true.");
    }

    /// Whether the diagnostics screen is enabled.
    ///
    /// If true, a "Diagnostics" button will be shown in the UI. You must implement
    /// the diagnostics_diagnostics method if this returns true.
    fn diagnostics_enable(&self) -> bool {
        false
    }
    /// Returns a list of key-value pairs representing diagnostics data.
    ///
    /// There is a maximum of 16 entries.
    fn diagnostics_diagnostics(&self) -> Vec<(String, String)> {
        panic!("You must implement diagnostics_diagnostics if diagnostics_enable returns true.");
    }
    /// Whether the diagnostics screen should use a compact layout.
    fn diagnostics_compact(&self) -> bool {
        false
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
            calibrating: if interface.calibrating_enable() {
                *interface.calibrating_calibrating().borrow()
            } else {
                false
            },
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
