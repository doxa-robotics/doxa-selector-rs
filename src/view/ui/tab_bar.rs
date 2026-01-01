use std::time::Duration;

use buoyant::{if_view, view::prelude::*};

use crate::view::{color, font, spacing, ui::Tab};

pub fn tab_bar(tab: Tab) -> impl View<color::Color, Tab> + use<> {
    HStack::new((
        tab_item("Brew", tab == Tab::Brew, |tab: &mut Tab| {
            *tab = Tab::Brew;
        }),
        tab_item("Clean", tab == Tab::Clean, |tab: &mut Tab| {
            *tab = Tab::Clean;
        }),
        tab_item("Settings", tab == Tab::Settings, |tab: &mut Tab| {
            *tab = Tab::Settings;
        }),
    ))
    .fixed_size(false, true)
    .animated(Animation::linear(Duration::from_millis(125)), tab)
}

fn tab_item<C, F: Fn(&mut C)>(
    name: &'static str,
    is_selected: bool,
    on_tap: F,
) -> impl View<color::Color, C> + use<C, F> {
    let (text_color, bar_height) = if is_selected {
        (color::ACCENT, 4)
    } else {
        (color::FOREGROUND_SECONDARY, 0)
    };

    Button::new(on_tap, move |is_pressed: bool| {
        VStack::new((
            ZStack::new((
                if_view!((is_selected || is_pressed) {
                    Rectangle.foreground_color(color::BACKGROUND_SECONDARY)
                }),
                VStack::new((
                    Circle.frame().with_width(15),
                    Text::new(name, &*font::CAPTION),
                ))
                .with_spacing(spacing::ELEMENT)
                .padding(Edges::All, spacing::ELEMENT)
                .hint_background_color(if is_selected || is_pressed {
                    color::BACKGROUND_SECONDARY
                } else {
                    color::BACKGROUND
                }),
            )),
            Rectangle.frame().with_height(bar_height),
        ))
        .foreground_color(text_color)
        .flex_frame()
        .with_min_width(100)
    })
}
