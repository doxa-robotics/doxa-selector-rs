use std::time::Duration;

use buoyant::{primitives::UnitPoint, view::prelude::*};
use embedded_ttf::FontTextStyle;

use crate::view::{
    color::{self, Color},
    font,
};

pub struct ButtonStyle<'a> {
    pub height: u32,
    pub horizontal_padding: u32,
    pub vertical_padding: u32,
    pub background: color::Color,
    pub foreground: color::Color,
    pub background_pressed: color::Color,
    pub foreground_pressed: color::Color,

    pub animation_duration: Duration,

    pub text_style: &'a FontTextStyle<Color>,
}

impl Default for ButtonStyle<'_> {
    fn default() -> Self {
        Self {
            height: 24,
            horizontal_padding: 12,
            vertical_padding: 8,
            background: color::M3_SECONDARY_CONTAINER,
            foreground: color::M3_ON_SECONDARY_CONTAINER,
            background_pressed: color::M3_SECONDARY_CONTAINER,
            foreground_pressed: color::M3_ON_SECONDARY_CONTAINER,
            animation_duration: Duration::from_millis(200),
            text_style: &*font::CAPTION,
        }
    }
}

pub fn button<'a, C: 'a>(
    label: &'a str,
    style: ButtonStyle<'a>,
    on_tap: fn(&mut C),
) -> impl View<color::Color, C> + 'a {
    Button::new(on_tap, move |is_pressed: bool| {
        Text::new(label, style.text_style)
            .foreground_color(if is_pressed {
                style.foreground_pressed
            } else {
                style.foreground
            })
            .hint_background_color(if is_pressed {
                style.background_pressed
            } else {
                style.background
            })
            .padding(Edges::Horizontal, style.horizontal_padding)
            .padding(Edges::Vertical, style.vertical_padding)
            .flex_frame()
            .with_min_height(style.height)
            .background(
                Alignment::Center,
                Capsule
                    .foreground_color(if is_pressed {
                        style.background_pressed
                    } else {
                        style.background
                    })
                    .scale_effect(if is_pressed { 0.9 } else { 1.0 }, UnitPoint::center())
                    .animated(Animation::ease_out(style.animation_duration), is_pressed),
            )
            .animated(Animation::ease_out(style.animation_duration), is_pressed)
    })
    .geometry_group()
}
