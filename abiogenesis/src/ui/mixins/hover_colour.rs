use bevy::prelude::*;

use crate::observe::observe;

pub fn mixin(base_color: Color, hover_colour: Color) -> impl Bundle {
    (
        BackgroundColor(base_color),
        observe(hover_start(hover_colour)),
        observe(hover_end(base_color)),
        observe(touch_end(base_color)),
    )
}

fn hover_start(
    hover_colour: Color,
) -> impl Fn(Trigger<Pointer<Over>>, Query<&mut BackgroundColor>) {
    move |trigger, mut background_color| {
        let Ok(mut background_color) = background_color.get_mut(trigger.target) else {
            return;
        };

        background_color.0 = hover_colour;
    }
}

fn hover_end(base_colour: Color) -> impl Fn(Trigger<Pointer<Out>>, Query<&mut BackgroundColor>) {
    move |trigger, mut background_color| {
        let Ok(mut background_color) = background_color.get_mut(trigger.target) else {
            return;
        };

        background_color.0 = base_colour;
    }
}

fn touch_end(base_colour: Color) -> impl Fn(Trigger<Pointer<Click>>, Query<&mut BackgroundColor>) {
    move |trigger, mut background_color| {
        let Ok(mut background_color) = background_color.get_mut(trigger.target) else {
            return;
        };

        background_color.0 = base_colour;
    }
}
