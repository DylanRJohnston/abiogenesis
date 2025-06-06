use bevy::ecs::system::IntoObserverSystem;
use bevy::prelude::*;

use crate::bundle_fn::BundleFn;

pub fn observe<E, B, M, O>(observer: O) -> BundleFn<impl FnOnce(&mut EntityWorldMut)>
where
    E: Event,
    B: Bundle,
    M: Send + Sync + 'static,
    O: IntoObserverSystem<E, B, M> + Send + Sync + 'static,
{
    BundleFn(move |entity| {
        entity.observe(observer);
    })
}
