#![no_main]
#![no_std]
#![feature(never_type)]

extern crate alloc;

mod platform;

use alloc::{
    boxed::Box,
    collections::btree_map::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use core::{
    fmt::{Debug, Display},
    ops::ControlFlow,
    time::Duration,
};

use async_trait::async_trait;
use platform::slint_platform::SelectorV5Platform;
use slint::VecModel;
use vexide::{core::time::Instant, prelude::*};

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

    fn calibrate_gyro(&mut self) {}

    fn is_gyro_calibrating(&self) -> bool {
        false
    }

    fn diagnostics(&self) -> Vec<(String, String)> {
        Vec::new()
    }

    fn autonomous_route_finished(&mut self, _return_value: Self::Return) {}
}

struct SharedData<'a, T, R> {
    selected_route: Option<&'a dyn AutonRoutine<T, Return = R>>,
    user: T,
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
    async fn compete_with_selector(self, display: vexide::devices::display::Display) -> ! {
        let platform = platform::slint_platform::SelectorV5Platform::new(display);
        slint::platform::set_platform(Box::new(platform.clone()))
            .expect("couldn't set slint platform");

        #[allow(clippy::unit_arg)]
        let runtime = CompetitionRuntime::builder(SharedData {
            selected_route: None,
            user: self,
            platform,
        })
        .on_connect(|s| Box::pin(async { ControlFlow::<!>::Continue(s.user.connected().await) }))
        .on_disconnect(|s| {
            Box::pin(async { ControlFlow::<!>::Continue(s.user.disconnected().await) })
        })
        .while_disabled(|s| {
            Box::pin(async {
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

                    sleep(Duration::from_millis(1000 / 30)).await;
                }
            })
        })
        .while_autonomous(|s| {
            Box::pin(async {
                // TODO: Run the autonomous routine
                let route = s.selected_route.expect("no selected route!");
                let route_rt = route.run(&mut s.user).await;
                s.user.autonomous_route_finished(route_rt);

                ControlFlow::<!>::Continue(())
            })
        })
        .while_driving(|s| Box::pin(async { ControlFlow::<!>::Continue(s.user.driver().await) }))
        .finish();

        runtime.await;
    }
}

impl<R> CompeteWithSelectorExt for R where R: CompeteWithSelector + 'static {}
