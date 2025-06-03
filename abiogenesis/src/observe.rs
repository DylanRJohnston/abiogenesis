use std::marker::PhantomData;

use bevy::ecs::bundle::{BundleEffect, DynamicBundle};
use bevy::ecs::component::*;
use bevy::ecs::system::IntoObserverSystem;
use bevy::prelude::*;

pub struct Observe<E, B, M, O>(O, PhantomData<(E, B, M)>);

impl<E, B, M, O> Observe<E, B, M, O>
where
    E: Event,
    B: Bundle,
    M: 'static + Send + Sync,
    O: IntoObserverSystem<E, B, M> + Send + Sync,
{
    pub fn event(observer: O) -> Self {
        Self(observer, PhantomData)
    }
}

unsafe impl<
    E: Event,
    B: Bundle,
    M: 'static + Send + Sync,
    O: IntoObserverSystem<E, B, M> + Send + Sync,
> Bundle for Observe<E, B, M, O>
{
    fn component_ids(_: &mut ComponentsRegistrator, _: &mut impl FnMut(ComponentId)) {}

    fn get_component_ids(_: &Components, _: &mut impl FnMut(Option<ComponentId>)) {}

    fn register_required_components(_: &mut ComponentsRegistrator, _: &mut RequiredComponents) {}
}

impl<E: Event, B: Bundle, M: Send + Sync, O: IntoObserverSystem<E, B, M>> DynamicBundle
    for Observe<E, B, M, O>
{
    type Effect = Observe<E, B, M, O>;

    fn get_components(
        self,
        _func: &mut impl FnMut(StorageType, bevy::ptr::OwningPtr<'_>),
    ) -> Self::Effect {
        self
    }
}

impl<E: Event, B: Bundle, M: Send + Sync, O: IntoObserverSystem<E, B, M>> BundleEffect
    for Observe<E, B, M, O>
{
    fn apply(self, entity: &mut EntityWorldMut) {
        entity.observe(self.0);
    }
}
