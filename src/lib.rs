#![no_main]
#![no_std]
#![feature(never_type)]

extern crate alloc;

mod platform;

use alloc::{boxed::Box, collections::btree_map::BTreeMap, rc::Rc, vec::Vec};
use core::{
    cell::RefCell,
    error::Error,
    fmt::{Debug, Display},
    ops::ControlFlow,
    sync::atomic::AtomicBool,
    time::Duration,
};

use async_trait::async_trait;
use platform::slint_platform::SelectorV5Platform;
use slint::{platform::software_renderer::MinimalSoftwareWindow, Rgb8Pixel};
use vexide::{
    core::{
        competition::CompetitionSystem,
        sync::{Barrier, Condvar, Mutex},
    },
    prelude::*,
};

slint::include_modules!();

#[async_trait]
pub trait AutonRoutine<CompeteT>: 'static {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    async fn run(&self, context: &mut CompeteT) -> Result<(), Box<dyn Error>>;
}

/// A set of tasks to run when the competition is in a particular mode.
#[allow(async_fn_in_trait, clippy::unused_async)]
pub trait CompeteWithSelector<Category>: Sized
where
    Category: Display + Debug + Clone,
{
    /// Runs when the competition system is connected.
    ///
    /// See [`Compete::connected`] for more information.
    async fn connected(&mut self) {}

    /// Runs when the competition system is disconnected.
    ///
    /// See [`Compete::disconnected`] for more information.
    async fn disconnected(&mut self) {}

    /// Returns a map of autonomous routines to run.
    fn autonomous_routes(&self) -> BTreeMap<Category, impl AsRef<[&dyn AutonRoutine<Self>]>>;

    /// Runs when the robot is put into driver control mode.
    ///
    /// See [`Compete::driver`] for more information.
    async fn driver(&mut self);
}

static STOP_UI_FLAG: (Mutex<bool>, Condvar) = (Mutex::new(true), Condvar::new());

struct SharedData<'a, T> {
    selected_category: usize,
    selected_route: usize,
}

/// Extension methods for [`Compete`].
/// Automatically implemented for any type implementing [`Compete`].
#[allow(clippy::type_complexity)]
pub trait CompeteWithSelectorExt<Category>: CompeteWithSelector<Category>
where
    Self: 'static,
    Category: Display + Debug + Clone + 'static,
{
    #[allow(async_fn_in_trait)]
    async fn compete_with_selector(self, display: vexide::devices::display::Display) -> ! {
        let window = MainWindow::new().expect("failed to initialize window");
        let weak = window.as_weak();

        let shared_data = Rc::new(Mutex::new(SharedData {
            selected_category: 0,
            selected_route: 0,
        }));

        #[allow(clippy::unit_arg)]
        let runtime = CompetitionRuntime::builder((self, shared_data.clone()))
            .on_connect(|s| {
                Box::pin(async { ControlFlow::<!>::Continue(s.0.user.connected().await) })
            })
            .on_disconnect(|s| {
                Box::pin(async { ControlFlow::<!>::Continue(s.user.disconnected().await) })
            })
            .while_disabled(|_| {
                Box::pin(async {
                    // Tell the UI task to start rendering/checking for events.
                    // Reenable the screen.
                    let mut lock = STOP_UI_FLAG.0.lock().await;
                    *lock = true;
                    STOP_UI_FLAG.1.notify_one();
                    ControlFlow::<!>::Continue(())
                })
            })
            .while_autonomous(|s| {
                Box::pin(async {
                    // Tell the UI task to stop rendering/checking for events.
                    // Only do this if the competition system is connected.
                    if vexide::core::competition::is_connected() {
                        let mut lock = STOP_UI_FLAG.0.lock().await;
                        *lock = false;
                        STOP_UI_FLAG.1.notify_one();
                    }
                    // TODO: Run the autonomous routine
                    let route = s
                        .selected_route
                        .lock()
                        .await
                        .expect("couldn't lock selected route");
                    route.run(&mut s.user);

                    ControlFlow::<!>::Continue(())
                })
            })
            .while_driving(|s| {
                Box::pin(async {
                    // Tell the UI task to stop rendering/checking for events.
                    // Only do this if the competition system is connected.
                    if vexide::core::competition::is_connected() {
                        let mut lock = STOP_UI_FLAG.0.lock().await;
                        *lock = false;
                        STOP_UI_FLAG.1.notify_one();
                    }
                    ControlFlow::<!>::Continue(s.user.driver().await)
                })
            })
            .finish();

        let platform = platform::slint_platform::SelectorV5Platform::new(display);
        slint::platform::set_platform(Box::new(platform.clone()))
            .expect("couldn't set slint platform");

        let selected_route_2 = selected_route.clone();
        window.on_picked(move |category_id, route_id| {
            *selected_route_2.try_lock().unwrap() = Some(
                autonomous_routes
                    .iter()
                    .skip(category_id as usize)
                    .next()
                    .unwrap()
                    .1
                    .as_ref()[route_id as usize],
            );
        });

        spawn(async move {
            window.show().expect("failed to run window");
            loop {
                if let Some(lock) = STOP_UI_FLAG.0.try_lock() {
                    if !*lock {
                        let _ = STOP_UI_FLAG.1.wait(lock).await;
                    }
                }
                platform.check_events();
                sleep(Duration::from_millis(1000 / 30)).await;
            }
        })
        .detach();

        runtime.await;
    }
}

impl<R, Category> CompeteWithSelectorExt<Category> for R
where
    R: CompeteWithSelector<Category> + 'static,
    Category: Display + Debug + Clone,
{
}
