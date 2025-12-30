use embedded_touch::{traits::TouchInputDevice, Touch};
use vexide::display::TouchEvent;

pub struct DisplayTouchDriver<'a> {
    display: &'a mut vexide::display::Display,
    touch: Option<embedded_touch::Touch>,
}

impl DisplayTouchDriver<'_> {
    pub fn new(display: &mut vexide::display::Display) -> DisplayTouchDriver<'_> {
        DisplayTouchDriver {
            display,
            touch: None,
        }
    }
}

impl TouchInputDevice for DisplayTouchDriver<'_> {
    type Error = !;

    fn touches(&mut self) -> Result<impl IntoIterator<Item = &Touch>, !> {
        let status = self.display.touch_status();
        let phase = match status.state {
            vexide::display::TouchState::Pressed => embedded_touch::Phase::Started,
            vexide::display::TouchState::Held => embedded_touch::Phase::Moved,
            vexide::display::TouchState::Released => embedded_touch::Phase::Ended,
        };
        self.touch = Some(embedded_touch::Touch {
            id: 1,
            location: embedded_touch::TouchPoint::new(status.point.x, status.point.y),
            phase,
            tool: embedded_touch::Tool::Finger,
        });
        Ok([self.touch.as_ref().unwrap()])
    }
}
