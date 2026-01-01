//! # Example: Espresso UI
//!
//! This example allows you to switch between three tabs using the left and right arrow keys.
//! The settings can be toggled using the `b`, `w`, and `o` keys.
//!
//! To run this example using the `embedded_graphics` simulator, you must have the `sdl2` package installed.
//! See [SDL2](https://github.com/Rust-SDL2/rust-sdl2) for installation instructions.

use std::time::{Duration, Instant};

use buoyant::{
    environment::DefaultEnvironment,
    event::{Event, EventContext},
    primitives::Point,
    render::{AnimatedJoin, AnimationDomain, Render},
    render_target::{EmbeddedGraphicsRenderTarget, RenderTarget as _},
    view::prelude::*,
};
use embedded_graphics::prelude::*;
use embedded_touch::traits::TouchInputDevice;
use unwrap_infallible::UnwrapInfallible;

use crate::{
    driver::DisplayTouchDriver,
    view::ui::{root_view, AppState},
};

mod color;
mod font;
mod spacing;
mod ui;

/// Target frames per second
///
/// The display can do 60 FPS, but to avoid unnecessary CPU usage, we limit to
/// 30 FPS
const FPS: u32 = 30;

/// Duration of each frame
const FRAME_DURATION: Duration = Duration::from_micros(1_000_000 / FPS as u64);

/// Minimum duration in between frames
///
/// To avoid hogging the executor, we ensure at least this much time passes
/// between frames
const MIN_FRAME_GAP: Duration = Duration::from_millis(5);

/// Time before we stop rendering due to inactivity
const INACTIVITY_TIMEOUT: Duration = Duration::from_secs(5);

pub async fn run(display: vexide::display::Display) {
    // DISPLAY RENDERING SETUP

    // Initialize display driver, which maps DrawTarget calls to the vexide Display API
    let mut display_driver = vexide_embedded_graphics::DisplayDriver::new(display);
    // Initialize another display to use for refreshing the screen
    let mut display = unsafe {
        // SAFETY: Technically, creating multiple Display instances is not good,
        // but in practice, the VEX SDK operates doesn't specify what a "display"
        // instance itself is, so creating multiple instances that all refer to the
        // same underlying hardware is okay.
        vexide::display::Display::new()
    };
    display.set_render_mode(vexide::display::RenderMode::DoubleBuffered);
    // Create a new buoyant render target
    let mut target =
        EmbeddedGraphicsRenderTarget::new_hinted(&mut display_driver, color::M3_BACKGROUND);

    // DISPLAY TOUCH SETUP
    let mut touch_display = unsafe { vexide::display::Display::new() };
    let mut touch = DisplayTouchDriver::new(&mut touch_display);

    // APPLICATION STATE SETUP

    // Application start time for animations
    let app_start = Instant::now();

    // Initial application state
    let mut app_data = AppState::default();

    // Create the initial view and state
    let mut view = root_view(&app_data);
    let mut state = view.build_state(&mut app_data);

    // Create initial source and target trees for animation
    let time = app_start.elapsed();
    let env = DefaultEnvironment::new(time);
    let layout = view.layout(&target.size().into(), &env, &mut app_data, &mut state);

    let mut source_tree =
        &mut view.render_tree(&layout, Point::default(), &env, &mut app_data, &mut state);
    let mut target_tree =
        &mut view.render_tree(&layout, Point::default(), &env, &mut app_data, &mut state);

    // Store the last update time, so we can cease animations after inactivity
    let mut last_update = Instant::now();

    loop {
        let frame_start = Instant::now();

        let time = app_start.elapsed();
        let domain = AnimationDomain::top_level(time);

        if last_update.elapsed() < INACTIVITY_TIMEOUT {
            // Render animated transition between source and target trees
            Render::render_animated(
                &mut target,
                source_tree,
                target_tree,
                &color::M3_PRIMARY_CONTAINER,
                &domain,
            );
            // Flush the rendered frame to the display
            display.render();
            // Clear the render target for the next frame
            target.clear(color::M3_BACKGROUND);
        }

        // Handle events
        let context = EventContext::new(time);
        for event in touch
            .touches()
            .unwrap_infallible()
            .into_iter()
            .map(|e| Event::Touch(e.clone()))
        {
            let result =
                view.handle_event(&event, &context, target_tree, &mut app_data, &mut state);
            if result.recompute_view {
                // Join source and target trees at current time, "freezing" animation progress
                target_tree.join_from(source_tree, &domain);
                // Swap trees so the current target becomes the next source.
                // Note this swaps the references instead of the whole section of memory
                core::mem::swap(&mut source_tree, &mut target_tree);
                // Create new view and target tree
                view = root_view(&app_data);
                let env = DefaultEnvironment::new(time);
                let layout = view.layout(&target.size().into(), &env, &mut app_data, &mut state);
                *target_tree =
                    view.render_tree(&layout, Point::default(), &env, &mut app_data, &mut state);
            }
            last_update = Instant::now();
        }

        let elapsed = frame_start.elapsed();
        let sleep_time = Duration::max(FRAME_DURATION.saturating_sub(elapsed), MIN_FRAME_GAP);
        println!(
            "\x1B[1A\x1B[KFPS: {:.2} | {:.0}%",
            1.0 / (elapsed + sleep_time).as_secs_f32(),
            (elapsed.as_secs_f32() / FRAME_DURATION.as_secs_f32()) * 100.0
        );
        // Throttle to maintain constant FPS
        vexide::time::sleep(sleep_time).await;
    }
}
