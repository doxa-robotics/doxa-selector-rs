use std::time::Duration;

use buoyant::{primitives::UnitPoint, view::prelude::*};

use crate::view::{
    color::{self},
    font,
};

pub struct CardStyle {
    pub height: u32,
    pub padding: u32,
    pub radius: u16,
    pub background: color::Color,
    pub foreground: color::Color,
    pub background_pressed: color::Color,
    pub foreground_pressed: color::Color,

    pub border_width: u32,
    pub border_color: color::Color,

    pub animation_duration: Duration,

    pub font_size: u32,
}

impl Default for CardStyle {
    fn default() -> Self {
        Self {
            height: 24,
            padding: 16,
            radius: 16,
            border_width: 1,
            border_color: color::M3_OUTLINE,
            background: color::M3_BACKGROUND,
            foreground: color::M3_ON_BACKGROUND,
            background_pressed: color::M3_SURFACE_CONTAINER,
            foreground_pressed: color::M3_ON_SURFACE,
            animation_duration: Duration::from_millis(200),
            font_size: font::SIZE_BODY,
        }
    }
}

pub fn card<'a, C: 'a, OnTapFn>(
    label: &'a str,
    style: CardStyle,
    on_tap: OnTapFn,
) -> impl View<color::Color, C> + 'a
where
    OnTapFn: Fn(&mut C) + 'a,
{
    Button::new(on_tap, move |is_pressed: bool| {
        ZStack::new((
            RoundedRectangle::new(style.radius)
                .foreground_color(if is_pressed {
                    style.background_pressed
                } else {
                    style.background
                })
                .padding(Edges::All, style.border_width)
                .background_color(
                    style.border_color,
                    RoundedRectangle::new(style.radius + style.border_width as u16),
                )
                .scale_effect(if is_pressed { 0.9 } else { 1.0 }, UnitPoint::center())
                .animated(Animation::ease_out(style.animation_duration), is_pressed),
            Text::new(label, &*font::MONTSERRAT)
                .with_font_size(style.font_size)
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
                .padding(Edges::All, style.padding)
                .flex_frame()
                .with_min_height(style.height),
        ))
        .animated(Animation::ease_out(style.animation_duration), is_pressed)
    })
    .geometry_group()
}
