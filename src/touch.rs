use embedded_touch::{traits::TouchInputDevice, Touch, TouchPoint};

// Blend factor for filtering touch input
const BLEND_FACTOR: f32 = 0.25;

pub struct DisplayTouchDriver<'a> {
    display: &'a mut vexide::display::Display,
    touch: Option<embedded_touch::Touch>,
    filtered: Option<TouchPoint>,
}

impl DisplayTouchDriver<'_> {
    pub fn new(display: &mut vexide::display::Display) -> DisplayTouchDriver<'_> {
        DisplayTouchDriver {
            display,
            touch: None,
            filtered: None,
        }
    }
}

impl TouchInputDevice for DisplayTouchDriver<'_> {
    type Error = !;

    fn touches(&mut self) -> Result<impl IntoIterator<Item = &Touch>, !> {
        // Collect touch status from the display
        let status = self.display.touch_status();

        // Map vexide touch state to embedded_touch phase
        let phase = match status.state {
            vexide::display::TouchState::Pressed => embedded_touch::Phase::Started,
            vexide::display::TouchState::Held => embedded_touch::Phase::Moved,
            vexide::display::TouchState::Released => embedded_touch::Phase::Ended,
        };

        let raw = TouchPoint::new(status.point.x, status.point.y);
        // Only interpolate if the touch is moving
        // We need to apply this filter because the VEX V5 brain appears to
        // emit touch events at around 5Hz, which can lead to jittery input
        // when the user is trying to drag their finger across the screen.
        let filtered = if phase == embedded_touch::Phase::Moved {
            let blended = match self.filtered {
                // Blend the new raw point with the previous filtered point
                // Apparently this is called a low-pass filter
                Some(prev) => {
                    let x = (prev.x as f32 + BLEND_FACTOR * (raw.x as f32 - prev.x as f32)) as i32;
                    let y = (prev.y as f32 + BLEND_FACTOR * (raw.y as f32 - prev.y as f32)) as i32;
                    TouchPoint::new(x, y)
                }
                None => raw,
            };
            self.filtered = Some(blended);
            blended
        } else {
            self.filtered = None;
            raw
        };

        self.touch = Some(embedded_touch::Touch {
            id: 1,
            location: filtered,
            phase,
            tool: embedded_touch::Tool::Finger,
        });
        Ok([self.touch.as_ref().unwrap()])
    }
}
