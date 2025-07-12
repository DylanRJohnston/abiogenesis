use bevy::ecs::bundle::{BundleEffect, DynamicBundle};
use bevy::ecs::component::*;
use bevy::prelude::*;

pub trait Thunk: FnOnce(&mut EntityWorldMut) + Send + Sync + 'static {}
impl<F: FnOnce(&mut EntityWorldMut) + Send + Sync + 'static> Thunk for F {}

pub struct BundleFn<T: Thunk>(pub T);

unsafe impl<T: Thunk> Bundle for BundleFn<T> {
    fn component_ids(_: &mut ComponentsRegistrator, _: &mut impl FnMut(ComponentId)) {}

    fn get_component_ids(_: &Components, _: &mut impl FnMut(Option<ComponentId>)) {}

    fn register_required_components(_: &mut ComponentsRegistrator, _: &mut RequiredComponents) {}
}

impl<T: Thunk> DynamicBundle for BundleFn<T> {
    type Effect = BundleFn<T>;

    fn get_components(
        self,
        _func: &mut impl FnMut(StorageType, bevy::ptr::OwningPtr<'_>),
    ) -> Self::Effect {
        self
    }
}

impl<T: Thunk> BundleEffect for BundleFn<T> {
    fn apply(self, entity: &mut EntityWorldMut) {
        (self.0)(entity);
    }
}
