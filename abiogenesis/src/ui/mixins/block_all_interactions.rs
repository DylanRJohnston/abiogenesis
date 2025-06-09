use bevy::prelude::*;

use crate::observe::observe;

pub fn mixin() -> impl Bundle {
    (
        observe(|mut trigger: Trigger<Pointer<Click>>| trigger.propagate(false)),
        observe(|mut trigger: Trigger<Pointer<Cancel>>| trigger.propagate(false)),
        observe(|mut trigger: Trigger<Pointer<Scroll>>| trigger.propagate(false)),
        observe(|mut trigger: Trigger<Pointer<Over>>| trigger.propagate(false)),
        observe(|mut trigger: Trigger<Pointer<Out>>| trigger.propagate(false)),
        observe(|mut trigger: Trigger<Pointer<Move>>| trigger.propagate(false)),
        observe(|mut trigger: Trigger<Pointer<DragStart>>| trigger.propagate(false)),
        observe(|mut trigger: Trigger<Pointer<Drag>>| trigger.propagate(false)),
        observe(|mut trigger: Trigger<Pointer<DragEnd>>| trigger.propagate(false)),
        observe(|mut trigger: Trigger<Pointer<Pressed>>| trigger.propagate(false)),
    )
}
