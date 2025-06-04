use bevy::prelude::*;

use crate::particles::simulation::Particle;

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
pub fn drag_screen(
    trigger: Trigger<Pointer<Drag>>,
    mut particles: Query<&mut Transform, With<Particle>>,
    projection: Single<&Projection>,
) {
    let Projection::Orthographic(ref project) = **projection else {
        return;
    };

    let mut delta = trigger.delta;
    delta.y *= -1.0;
    delta *= project.scale;

    for mut particle in &mut particles {
        particle.translation += delta.extend(0.0);
    }
}

const MAX_ZOOM: f32 = 1.0;
const MIN_ZOOM: f32 = 0.1;

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
pub fn zoom(trigger: Trigger<Pointer<Scroll>>, mut projection: Single<&mut Projection>) {
    let Projection::Orthographic(ref mut project) = **projection else {
        return;
    };

    project.scale = (project.scale - trigger.y * 0.01).clamp(MIN_ZOOM, MAX_ZOOM);
}

// #[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
// fn explode(
//     trigger: Trigger<Pointer<Click>>,
//     mut particles: Query<(&mut Transform, &mut Velocity), With<Particle>>,
//     simulation_size: SimulationSize,
//     index: Res<SpatialIndex>,
// ) {
//     let Vec2 {
//         x: width,
//         y: height,
//     } = simulation_size.dimensions();

//     // Pointer coordinates start in the top right corner, with positive x to the left and positive y downard.
//     let pointer_location = Vec2::new(
//         trigger.pointer_location.position.x - width / 2.0,
//         height - trigger.pointer_location.position.y - height / 2.0,
//     );

//     for (pos, (entity, _)) in index.query(pointer_location, 150.0) {
//         let Ok((mut transform, mut velocity)) = particles.get_mut(*entity) else {
//             continue;
//         };

//         let direction = (pos - pointer_location).normalize();

//         **velocity = direction * 4000.0;
//         transform.translation += (direction * 100.0).extend(0.0);
//     }
// }
