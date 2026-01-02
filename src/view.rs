use std::{
    cell::RefCell,
    rc::Rc,
    time::{Duration, Instant},
};

use buoyant::{
    environment::DefaultEnvironment,
    event::{Event, EventContext},
    primitives::Point,
    render::{AnimatedJoin, AnimationDomain, Render},
    render_target::{EmbeddedGraphicsRenderTarget, RenderTarget as _},
    view::prelude::*,
};
use embedded_touch::traits::TouchInputDevice;
use unwrap_infallible::UnwrapInfallible;

use crate::{
    driver::DisplayTouchDriver,
    view::ui::{root_view, AppData, AppState},
};

mod color;
mod font;
mod image;
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
///
/// Maximum animation duration
const INACTIVITY_TIMEOUT: Duration = Duration::from_secs(1);

pub async fn run<C: crate::route::Category, R: 'static>(
    display: vexide::display::Display,
    external: Rc<RefCell<crate::ExternalState>>,
    interface: Rc<RefCell<dyn crate::DoxaSelectInterface>>,
    routes: Vec<crate::Route<C, R>>,
    categories: Vec<C>,
) {
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
    let mut app_state = AppState::new(external, interface);
    let app_data = AppData::new(routes, categories);

    // Create the initial view and state
    let mut view = root_view(&app_state, &app_data);
    let mut state = view.build_state(&mut app_state);

    // Create initial source and target trees for animation
    let time = app_start.elapsed();
    let env = DefaultEnvironment::new(time);
    let layout = view.layout(&target.size().into(), &env, &mut app_state, &mut state);

    let mut source_tree =
        &mut view.render_tree(&layout, Point::default(), &env, &mut app_state, &mut state);
    let mut target_tree =
        &mut view.render_tree(&layout, Point::default(), &env, &mut app_state, &mut state);

    // Store the last update time, so we can cease animations after inactivity
    let mut last_update = Instant::now();

    // Store the last external state to detect changes
    let mut external_state = app_state.external.borrow().clone();
    let mut previous_mode = vexide::competition::mode();
    let mut should_render = true;

    loop {
        // Update state
        {
            let interface = app_state.interface.borrow();
            let mut external = app_state.external.borrow_mut();
            external.show_calibrating = interface.calibrating_enable();
            external.show_diagnostics = interface.diagnostics_enable();
            external.calibrating = *interface.calibrating_calibrating().borrow();
        }

        let frame_start = Instant::now();

        let mode = vexide::competition::mode();
        let time = app_start.elapsed();
        let domain = AnimationDomain::top_level(time);

        if should_render {
            if mode == vexide::competition::CompetitionMode::Autonomous {
                // In autonomous mode, we skip animations to save compute
                Render::render(target_tree, &mut target, &color::M3_PRIMARY_CONTAINER);
            } else {
                // Render animated transition between source and target trees
                Render::render_animated(
                    &mut target,
                    source_tree,
                    target_tree,
                    &color::M3_PRIMARY_CONTAINER,
                    &domain,
                );
            }
            // Flush the rendered frame to the display
            display.render();
            // Clear the render target for the next frame
            target.clear(color::M3_BACKGROUND);
        }

        // For the next frame, determine if we should keep rendering
        if mode != vexide::competition::CompetitionMode::Autonomous {
            // In non-autonomous mode, we render until inactivity timeout
            should_render = last_update.elapsed() < INACTIVITY_TIMEOUT;
        } else {
            // In autonomous mode, we don't render at all unless there's activity
            should_render = false;
        }

        // Handle events
        let context = EventContext::new(time);
        let touch_events = touch
            .touches()
            .unwrap_infallible()
            .into_iter()
            .map(|e| Event::Touch(e.clone()));
        // Diff external state to generate synthetic events if needed
        let synthetic_events = {
            let current_external = app_state.external.borrow();
            let mut events = Vec::new();
            if *current_external != external_state {
                events.push(Event::External);
                external_state = current_external.clone();
            }
            if previous_mode != mode {
                if mode == vexide::competition::CompetitionMode::Autonomous {
                    // Switch to confirmed screen in autonomous mode, since
                    // that means that the match has started
                    app_state.screen = crate::view::ui::Screen::Confirmed;
                    events.push(Event::External);
                }
                previous_mode = mode;
            }
            events
        };
        for event in touch_events.chain(synthetic_events) {
            let result =
                view.handle_event(&event, &context, target_tree, &mut app_state, &mut state);
            // Buoyant seems to have a bug where external events don't trigger recompute_view
            if result.recompute_view || matches!(event, Event::External) {
                // Join source and target trees at current time, "freezing" animation progress
                target_tree.join_from(source_tree, &domain);
                // Swap trees so the current target becomes the next source.
                // Note this swaps the references instead of the whole section of memory
                core::mem::swap(&mut source_tree, &mut target_tree);
                // Create new view and target tree
                view = root_view(&app_state, &app_data);
                let env = DefaultEnvironment::new(time);
                let layout = view.layout(&target.size().into(), &env, &mut app_state, &mut state);
                *target_tree =
                    view.render_tree(&layout, Point::default(), &env, &mut app_state, &mut state);
                should_render = true;
            }
            last_update = Instant::now();
        }
        // Update the external state. The view could have changed it, and we don't
        // need to re-render for internally-generated changes.
        // We know the view changed it and not external code because there's no
        // await points here.
        if *app_state.external.borrow() != external_state {
            external_state = app_state.external.borrow().clone();
        }

        let elapsed = frame_start.elapsed();
        let sleep_time = Duration::max(FRAME_DURATION.saturating_sub(elapsed), MIN_FRAME_GAP);
        // println!(
        //     "\x1B[1A\x1B[KFPS: {:.2} | {:.0}%",
        //     1.0 / (elapsed + sleep_time).as_secs_f32(),
        //     (elapsed.as_secs_f32() / FRAME_DURATION.as_secs_f32()) * 100.0
        // );
        // Throttle to maintain constant FPS
        vexide::time::sleep(sleep_time).await;
    }
}
