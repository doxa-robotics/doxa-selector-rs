use alloc::rc::Rc;
use core::cell::RefCell;

use slint::{
    platform::{
        software_renderer::{MinimalSoftwareWindow, RepaintBufferType},
        Platform,
    },
    PhysicalSize, Rgb8Pixel,
};
use vexide_core::time::Instant;
use vexide_devices::display::Display;

#[derive(Clone)]
pub struct SelectorV5Platform {
    start: Instant,
    window: Rc<MinimalSoftwareWindow>,
    display: Rc<RefCell<Display>>,
    display_pressed: Rc<RefCell<bool>>,

    buffer: Rc<
        RefCell<
            [Rgb8Pixel;
                Display::HORIZONTAL_RESOLUTION as usize * Display::VERTICAL_RESOLUTION as usize],
        >,
    >,
}
impl Platform for SelectorV5Platform {
    fn create_window_adapter(
        &self,
    ) -> Result<alloc::rc::Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError> {
        Ok(self.window.clone())
    }

    fn duration_since_start(&self) -> core::time::Duration {
        self.start.elapsed()
    }

    fn run_event_loop(&self) -> Result<(), slint::PlatformError> {
        Err(slint::PlatformError::from(
            "event loop is purposely not implemented",
        ))
    }
}

impl SelectorV5Platform {
    pub fn check_events(&self) {
        slint::platform::update_timers_and_animations();
        self.window.draw_if_needed(|renderer| {
            super::slint_interlope::render_to_display(
                renderer,
                &mut self.display.borrow_mut(),
                &mut self.buffer.borrow_mut(),
            );
        });
        self.window
            .dispatch_event(super::slint_interlope::convert_touch_event(
                &self.display.borrow().touch_status(),
                &self.display_pressed,
            ));
    }

    #[must_use]
    pub fn new(display: Display) -> Self {
        let window = MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer);
        window.set_size(PhysicalSize::new(
            Display::HORIZONTAL_RESOLUTION as _,
            Display::VERTICAL_RESOLUTION as _,
        ));
        Self {
            start: Instant::now(),
            window,
            display: Rc::new(RefCell::new(display)),
            display_pressed: Rc::new(RefCell::new(false)),
            #[allow(clippy::large_stack_arrays)] // we got plenty
            buffer: Rc::new(RefCell::new(
                [Rgb8Pixel::new(0, 0, 0);
                    Display::HORIZONTAL_RESOLUTION as usize * Display::VERTICAL_RESOLUTION as usize],
            )),
        }
    }
}
