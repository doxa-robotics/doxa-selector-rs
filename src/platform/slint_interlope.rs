//! Utilities to interface between Slint and vexide.
//!
//! Largely borrowed from https://github.com/vexide/vexide/blob/5231eff4ca8c9c630cba24f676866b2385759aa4/packages/vexide-graphics/src/slint.rs

use std::cell::RefCell;

use slint::{
    platform::{software_renderer::SoftwareRenderer, PointerEventButton, WindowEvent},
    LogicalPosition, PhysicalPosition, Rgb8Pixel,
};
use vexide::{
    color::Rgb,
    display::{Display, Rect, TouchEvent, TouchState},
};

pub fn convert_touch_event(event: &TouchEvent, display_pressed: &RefCell<bool>) -> WindowEvent {
    let physical_pos = PhysicalPosition::new(event.point.x.into(), event.point.y.into());
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
    draw_buffer(
        display,
        Rect::from_dimensions(
            [0, 0],
            Display::HORIZONTAL_RESOLUTION as _,
            Display::VERTICAL_RESOLUTION as _,
        ),
        *buf,
    );
}
pub(crate) trait RgbExt {
    #[allow(unused)]
    fn from_raw(raw: u32) -> Self;
    fn into_raw(self) -> u32;
}

impl RgbExt for Rgb<u8> {
    fn from_raw(raw: u32) -> Self {
        const BITMASK: u32 = 0b1111_1111;

        Self {
            r: ((raw >> 16) & BITMASK) as _,
            g: ((raw >> 8) & BITMASK) as _,
            b: (raw & BITMASK) as _,
        }
    }

    fn into_raw(self) -> u32 {
        (u32::from(self.r) << 16) + (u32::from(self.g) << 8) + u32::from(self.b)
    }
}

pub fn draw_buffer<T, I>(display: &mut Display, region: Rect, buf: T)
where
    T: IntoIterator<Item = I>,
    I: Into<Rgb<u8>>,
{
    let mut raw_buf = buf
        .into_iter()
        .map(|i| i.into().into_raw())
        .collect::<Vec<_>>();
    // Convert the coordinates to u32 to avoid overflows when multiplying.
    let expected_size =
        ((region.end.y - region.start.y) as u32 * (region.end.x - region.start.x) as u32) as usize;

    let buffer_size = raw_buf.len();
    assert_eq!(
            buffer_size, expected_size,
            "The given buffer of colors was wrong size to fill the specified area: expected {expected_size} bytes, got {buffer_size}."
        );

    // SAFETY: The buffer is guaranteed to be the correct size.
    unsafe {
        vex_sdk::vexDisplayCopyRect(
            i32::from(region.start.x),
            i32::from(region.start.y + Display::HEADER_HEIGHT),
            i32::from(region.end.x),
            i32::from(region.end.y + Display::HEADER_HEIGHT),
            raw_buf.as_mut_ptr(),
            i32::from(region.end.x - region.start.x),
        );
    }
}
