//! Simple touchscreen-based autonomous route selector.
//!
//! ![Screenshot of the `SimpleSelect` menu showing two routes](https://i.imgur.com/qM9qMsd.png)
//!
//! [`SimpleSelect`] is a barebones and lightweight autonomous selector that allows picking
//! between at most 12 autonomous routes using the V5 Brain's display and touchscreen.
//!
//! The selector provides a user interface that mimicks the appearance of other VEXos
//! dashboards, with basic support for color themes through the [`SimpleSelect::new_with_theme`]
//! function.
//!
//! # Examples
//!
//! Robot with two autonomous routes using [`SelectCompete`](crate::compete::SelectCompete).
//!
//! ```
//! use vexide::prelude::*;
//! use autons::{
//!     prelude::*,
//!     simple::{route, SimpleSelect},
//! };
//!
//! struct Robot {}
//!
//! impl Robot {
//!     async fn route_1(&mut self) {}
//!     async fn route_2(&mut self) {}
//! }
//!
//! impl SelectCompete for Robot {
//!     async fn driver(&mut self) {
//!         // ...
//!     }
//! }
//!
//! #[vexide::main]
//! async fn main(peripherals: Peripherals) {
//!     let robot = Robot {};
//!
//!     robot
//!         .compete(SimpleSelect::new(
//!             peripherals.display,
//!             [
//!                 route!("Route 1", Robot::route_1),
//!                 route!("Route 2", Robot::route_2),
//!             ],
//!         ))
//!         .await;
//! }
//! ```
#![feature(trait_alias)]

use std::{cell::RefCell, rc::Rc};

use autons::Selector;
use buoyant::{
    color,
    render_target::{EmbeddedGraphicsRenderTarget, RenderTarget},
    view::AsDrawable,
};
use embedded_graphics::{
    geometry::OriginDimensions as _, pixelcolor::Rgb888, prelude::RgbColor, Drawable,
};
use vexide::{
    display::Display,
    task::{self, Task},
};

mod route;
mod view;

pub use route::*;

struct SelectorState<C: Category, R: 'static, const N: usize> {
    routes: [Route<C, R>; N],
    categories: Vec<C>,
    selection: usize,
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
pub struct DoxaSelect<C: Category, R: 'static, const N: usize> {
    state: Rc<RefCell<SelectorState<C, R, N>>>,
    _task: Task<()>,
}

impl<C: Category, R, const N: usize> DoxaSelect<C, R, N> {
    /// Creates a new selector from a [`Display`] peripheral and array of routes.
    pub fn new(display: Display, routes: [Route<C, R>; N]) -> Self {
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

        let state = Rc::new(RefCell::new(SelectorState {
            routes,
            categories,
            selection: 0,
        }));

        Self {
            state: state.clone(),
            _task: task::spawn(async move {
                view::run(display).await;
            }),
        }
    }

    /// Programatically selects an autonomous route by index.
    pub fn select(&mut self, index: usize) {
        assert!(index < N, "Invalid route selection index.");
        let mut state = self.state.borrow_mut();
        state.selection = index;
    }
}

impl<C: Category, R, const N: usize> Selector<R> for DoxaSelect<C, R, N> {
    async fn run(&self, robot: &mut R) {
        {
            let state = self.state.borrow();
            (state.routes[state.selection].callback)(robot)
        }
        .await;
    }
}
