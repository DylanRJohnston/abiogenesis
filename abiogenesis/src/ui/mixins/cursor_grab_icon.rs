use bevy::{prelude::*, window::SystemCursorIcon, winit::cursor::CursorIcon};

use crate::observe::observe;

pub fn mixin() -> impl Bundle {
    (observe(hover_start), observe(hover_end))
}

fn hover_start(
    _trigger: Trigger<Pointer<Over>>,
    window: Single<Entity, With<Window>>,
    mut commands: Commands,
) {
    commands
        .entity(*window)
        .insert(CursorIcon::from(SystemCursorIcon::Grab));
}

fn hover_end(
    _trigger: Trigger<Pointer<Out>>,
    window: Single<Entity, With<Window>>,
    mut commands: Commands,
) {
    commands
        .entity(*window)
        .insert(CursorIcon::from(SystemCursorIcon::Default));
}
