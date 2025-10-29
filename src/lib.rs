#![no_main]
#![no_std]
#![feature(never_type)]

extern crate alloc;

mod controller;
#[cfg(feature = "ui")]
mod platform;

#[cfg(feature = "ui")]
use alloc::string::ToString;
use alloc::{boxed::Box, collections::btree_map::BTreeMap, string::String, vec::Vec};
#[cfg(feature = "ui")]
use core::time::Duration;
use core::{
    fmt::{Debug, Display},
    ops::ControlFlow,
};
#[cfg(feature = "ui")]
use std::time::Instant;

use async_trait::async_trait;
#[cfg(feature = "ui")]
use platform::slint_platform::SelectorV5Platform;
#[cfg(feature = "ui")]
use slint::VecModel;
use vexide::{competition::CompetitionRuntime, display::Display as VexideDisplay};

#[cfg(feature = "ui")]
slint::include_modules!();

#[async_trait]
pub trait AutonRoutine<CompeteT>: 'static {
    type Return;

    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    async fn run(&self, context: &mut CompeteT) -> Self::Return;
}

/// A set of tasks to run when the competition is in a particular mode.
#[allow(async_fn_in_trait, clippy::unused_async)]
pub trait CompeteWithSelector: Sized {
    type Category: Display + Debug + Clone;
    type Return;

    /// Runs when the competition system is connected.
    ///
    /// See [`Compete::connected`] for more information.
    async fn connected(&mut self) {}

    /// Runs when the competition system is disconnected.
    ///
    /// See [`Compete::disconnected`] for more information.
    async fn disconnected(&mut self) {}

    /// Returns a map of autonomous routines to run.
    ///
    /// This map must not change over the lifetime of the program.
    fn autonomous_routes<'a, 'b>(
        &'b self,
    ) -> BTreeMap<Self::Category, impl AsRef<[&'a dyn AutonRoutine<Self, Return = Self::Return>]>>
    where
        Self: 'a;

    /// Runs when the robot is put into driver control mode.
    ///
    /// See [`Compete::driver`] for more information.
    async fn driver(&mut self);

    /// Calibrates the gyro. Called when the user requests a calibration.
    fn calibrate_gyro(&mut self) {}

    /// Returns whether the gyro is currently calibrating.
    fn is_gyro_calibrating(&self) -> bool {
        false
    }

    /// Returns a list of diagnostics to display in the UI.
    fn diagnostics(&self) -> Vec<(String, String)> {
        Vec::new()
    }

    /// Runs when an autonomous routine is started.
    fn autonomous_route_started(&mut self, _route: &dyn AutonRoutine<Self, Return = Self::Return>) {
    }

    /// Runs when an autonomous routine finishes.
    fn autonomous_route_finished(&mut self, _return_value: Self::Return) {}

    /// A reference to the controller, if available. This allows the UI task
    /// to exit early if a controller interaction is detected.
    fn controller(&self) -> Option<&vexide::controller::Controller> {
        None
    }
}

struct SharedData<'a, T, R> {
    selected_route: Option<&'a dyn AutonRoutine<T, Return = R>>,
    default_route: Option<&'a dyn AutonRoutine<T, Return = R>>,
    user: T,
    #[cfg(feature = "ui")]
    platform: SelectorV5Platform,
}

/// Extension methods for [`Compete`].
/// Automatically implemented for any type implementing [`Compete`].
#[allow(clippy::type_complexity)]
pub trait CompeteWithSelectorExt: CompeteWithSelector
where
    Self: 'static,
{
    #[allow(async_fn_in_trait)]
    #[cfg_attr(not(feature = "ui"), allow(unused_variables))]
    async fn compete_with_selector(
        self,
        display: VexideDisplay,
        default_route: Option<&'static dyn AutonRoutine<Self, Return = Self::Return>>,
    ) -> ! {
        #[cfg(feature = "ui")]
        let platform = {
            let p = platform::slint_platform::SelectorV5Platform::new(display);
            slint::platform::set_platform(Box::new(p.clone()))
                .expect("couldn't set slint platform");
            p
        };

        #[allow(clippy::unit_arg)]
        let runtime = CompetitionRuntime::builder(SharedData {
            selected_route: None,
            user: self,
            #[cfg(feature = "ui")]
            platform,
            default_route,
        })
        .on_connect(|s| Box::pin(async { ControlFlow::<!>::Continue(s.user.connected().await) }))
        .on_disconnect(|s| {
            Box::pin(async { ControlFlow::<!>::Continue(s.user.disconnected().await) })
        })
        .while_disabled(|s| {
            Box::pin(async {
                #[cfg(feature = "ui")]
                {
                    run_window_event_loop(s, false).await
                }
                #[cfg(not(feature = "ui"))]
                {
                    ControlFlow::<!>::Continue(())
                }
            })
        })
        .while_autonomous(|s| {
            Box::pin(async {
                if let Some(route) = s.selected_route {
                    let route_rt = route.run(&mut s.user).await;
                    s.user.autonomous_route_finished(route_rt);
                } else if let Some(route) = s.default_route {
                    let route_rt = route.run(&mut s.user).await;
                    s.user.autonomous_route_finished(route_rt);
                }

                ControlFlow::<!>::Continue(())
            })
        })
        .while_driving(|s| {
            Box::pin(async {
                #[cfg(feature = "ui")]
                if !vexide::competition::is_connected() {
                    // If we're not connected to the competition system, run the UI.
                    // The driver will not be able to control the robot until we are connected!
                    run_window_event_loop(s, true).await;
                }
                ControlFlow::<!>::Continue(s.user.driver().await)
            })
        })
        .finish();

        runtime.await
    }
}

impl<R> CompeteWithSelectorExt for R where R: CompeteWithSelector + 'static {}

#[cfg(feature = "ui")]
async fn run_window_event_loop<T, R: 'static>(
    s: &mut SharedData<'_, T, R>,
    is_driver: bool,
) -> ControlFlow<!>
where
    T: CompeteWithSelector<Return = R> + 'static,
{
    let window = MainWindow::new().expect("failed to initialize window");

    let mut last_diagnostics_refresh = Instant::now();

    window.set_categories(VecModel::from_slice(
        &s.user
            .autonomous_routes()
            .iter()
            .map(|x| Category {
                name: x.0.to_string().into(),
                routes: VecModel::from_slice(
                    &x.1.as_ref()
                        .iter()
                        .map(|y| Route {
                            name: y.name().to_string().into(),
                            description: y.description().to_string().into(),
                        })
                        .collect::<Vec<_>>(),
                ),
            })
            .collect::<Vec<_>>(),
    ));

    let build_diagnostics = |diagnostics: Vec<(String, String)>| {
        VecModel::from_slice(
            &diagnostics
                .iter()
                .map(|(k, v)| {
                    VecModel::from_slice(&[
                        slint::SharedString::from(k.to_string()).into(),
                        slint::SharedString::from(v.to_string()).into(),
                    ])
                })
                .collect::<Vec<_>>(),
        )
    };
    window.set_diagnostics(build_diagnostics(s.user.diagnostics()));

    let mut current_category_id = -1;
    let mut current_route_id = -1;
    loop {
        use vexide::competition;

        s.platform.check_events();

        // Handle the window state
        let category_id = window.get_picked_category_id();
        let route_id = window.get_picked_route_id();
        if category_id >= 0
            && route_id >= 0
            && (category_id != current_category_id || route_id != current_route_id)
        {
            let selected_route = *s
                .user
                .autonomous_routes()
                .iter()
                .nth(category_id as usize)
                .expect("nonexistent category")
                .1
                .as_ref()
                .get(route_id as usize)
                .expect("nonexistent route");
            s.selected_route = Some(selected_route);
            current_category_id = category_id;
            current_route_id = route_id;
        }

        window.set_gyro_calibrating(s.user.is_gyro_calibrating());

        if window.get_refresh_diagnostics_requested() {
            window.set_diagnostics(build_diagnostics(s.user.diagnostics()));
            window.set_refresh_diagnostics_requested(false);
        }

        if window.get_gyro_calibration_requested() {
            s.user.calibrate_gyro();
            window.set_gyro_calibration_requested(false);
        }

        if last_diagnostics_refresh.elapsed() > Duration::from_secs(1) {
            window.set_diagnostics(build_diagnostics(s.user.diagnostics()));
            last_diagnostics_refresh = Instant::now();
        }

        if is_driver
            && (competition::is_connected()
                || s.user.controller().is_some_and(controller::has_interaction))
        {
            break ControlFlow::Continue(());
        }

        vexide::time::sleep(Duration::from_millis(1000 / 30)).await;
    }
}
