//! Utilities to interface between Slint and vexide.
//!
//! Largely borrowed from https://github.com/vexide/vexide/blob/5231eff4ca8c9c630cba24f676866b2385759aa4/packages/vexide-graphics/src/slint.rs

use core::cell::RefCell;

use slint::{
    platform::{software_renderer::SoftwareRenderer, PointerEventButton, WindowEvent},
    LogicalPosition, PhysicalPosition, Rgb8Pixel,
};
use vexide_devices::display::{Display, Rect, TouchEvent, TouchState};

pub fn convert_touch_event(event: &TouchEvent, display_pressed: &RefCell<bool>) -> WindowEvent {
    let physical_pos = PhysicalPosition::new(event.x.into(), event.y.into());
    let position = LogicalPosition::from_physical(physical_pos, 1.0);
    match event.state {
        TouchState::Released => {
            *display_pressed.borrow_mut() = false;
            WindowEvent::PointerReleased {
                position,
                button: PointerEventButton::Left,
            }
        }
        TouchState::Pressed => {
            if display_pressed.replace(true) {
                WindowEvent::PointerMoved { position }
            } else {
                WindowEvent::PointerPressed {
                    position,
                    button: PointerEventButton::Left,
                }
            }
        }
        TouchState::Held => WindowEvent::PointerMoved { position },
    }
}

pub fn render_to_display(
    renderer: &SoftwareRenderer,
    display: &mut Display,
    buf: &mut [Rgb8Pixel;
             Display::HORIZONTAL_RESOLUTION as usize * Display::VERTICAL_RESOLUTION as usize],
) {
    renderer.render(buf, Display::HORIZONTAL_RESOLUTION as _);
    // Unwrap because the buffer is guaranteed to be the correct size
    display.draw_buffer(
        Rect::from_dimensions(
            [0, 0],
            Display::HORIZONTAL_RESOLUTION as _,
            Display::VERTICAL_RESOLUTION as _,
        ),
        *buf,
        Display::HORIZONTAL_RESOLUTION.into(),
    );
}
