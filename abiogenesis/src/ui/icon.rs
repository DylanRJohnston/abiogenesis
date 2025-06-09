use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};

#[derive(Debug, Component, Deref)]
#[component(immutable, on_insert = load_icon)]
pub struct Icon(pub &'static str);

fn load_icon(mut world: DeferredWorld, ctx: HookContext) {
    let icon = world.entity(ctx.entity).get::<Icon>().unwrap();
    let handle = world.load_asset::<Image>(**icon);

    world
        .commands()
        .entity(ctx.entity)
        .insert(ImageNode::new(handle));
}
