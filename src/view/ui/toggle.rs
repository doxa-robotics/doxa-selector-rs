use std::time::Duration;

use buoyant::{
    if_view,
    primitives::UnitPoint,
    transition::{Edge, Move},
    view::prelude::*,
};
use embedded_graphics::prelude::{RgbColor as _, WebColors as _};

use crate::view::{color, font, spacing};

pub fn toggle_text<C>(
    label: &'static str,
    is_on: bool,
    description: &'static str,
    hides_description: bool,
    action: fn(&mut C),
) -> impl View<color::Color, C> + use<C> {
    VStack::new((
        HStack::new((
            Text::new(label, &*font::BODY).foreground_color(color::Color::WHITE),
            toggle_button(is_on, action),
        ))
        .with_spacing(spacing::ELEMENT),
        if_view!((is_on || !hides_description) {
            Text::new(description, &*font::CAPTION)
                .multiline_text_alignment(HorizontalTextAlignment::Trailing)
                .foreground_color(color::Color::WHITE)
                .transition(Move::new(Edge::Trailing))
        }),
    ))
    .with_spacing(spacing::ELEMENT)
    .with_alignment(HorizontalAlignment::Trailing)
    .flex_infinite_width(HorizontalAlignment::Trailing)
}

pub fn toggle_button<C>(is_on: bool, on_tap: fn(&mut C)) -> impl View<color::Color, C> + use<C> {
    let (color, alignment) = if is_on {
        (color::ACCENT, HorizontalAlignment::Trailing)
    } else {
        (color::Color::CSS_LIGHT_GRAY, HorizontalAlignment::Leading)
    };

    Button::new(on_tap, move |is_pressed: bool| {
        ZStack::new((
            buoyant::view::shape::Capsule.foreground_color(color),
            buoyant::view::shape::Circle
                .foreground_color(if is_pressed {
                    color::Color::CSS_LIGHT_GRAY
                } else {
                    color::Color::WHITE
                })
                .scale_effect(if is_pressed { 1.5 } else { 1.0 }, UnitPoint::center())
                .padding(Edges::All, 2)
                .animated(
                    Animation::ease_out(Duration::from_millis(500)),
                    (is_on, is_pressed),
                ),
        ))
        .with_horizontal_alignment(alignment)
        .frame_sized(50, 25)
        .geometry_group()
    })
}
