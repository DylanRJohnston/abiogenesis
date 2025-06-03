use std::cell::RefCell;

use bevy::{ecs::system::SystemParam, prelude::*};

#[cfg(feature = "hot_reload")]
use bevy_simple_subsecond_system::hot;
use itertools::Itertools;

use crate::{math::remap, spatial_hash::SpatialHashGrid};

pub struct ParticlePlugin;

const PARTICLES_PER_COLOR: usize = 1000;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Model {
            weights: [[0.3, 0.4, 0.5], [0.7, -0.4, 0.3], [-0.5, 0.5, 0.0]],
        })
        .register_type::<SimulationParams>()
        .insert_resource(SimulationParams::DEFAULT)
        .insert_resource(ParticleIndexes(Vec::with_capacity(4 * PARTICLES_PER_COLOR)))
        .add_systems(Startup, spawn_particles)
        .add_systems(
            PreUpdate,
            (|mut spatial_index: ResMut<SpatialIndex>, simulation_size: SimulationSize| {
                spatial_index.update_bounds(Rect::from_center_size(
                    Vec2::ZERO,
                    simulation_size.dimensions(),
                ));
            }),
        )
        .add_observer(respawn_particles)
        .add_systems(Update, particle_decay)
        .add_systems(Update, compute_forces);
    }
}

#[derive(Debug, Reflect, Component, Default, Clone, Copy)]
pub enum ParticleColour {
    #[default]
    Red,
    Green,
    Blue,
    Orange,
}

pub const RED: Color = Color::srgb_from_array([172.0 / 255.0, 40.0 / 255.0, 71.0 / 255.0]);
pub const GREEN: Color = Color::srgb_from_array([90.0 / 255.0, 181.0 / 255.0, 82.0 / 255.0]);
pub const BLUE: Color = Color::srgb_from_array([51.0 / 255.0, 136.0 / 255.0, 222.0 / 255.0]);
// pub const ORANGE: Color = Color::srgb_from_array([233.0 / 255.0, 133.0 / 255.0, 55.0 / 255.0]);
pub const ORANGE: Color = Color::srgb_from_array([233.0 / 255.0, 133.0 / 255.0, 55.0 / 255.0]);

impl Into<Color> for ParticleColour {
    fn into(self) -> Color {
        match self {
            ParticleColour::Red => RED.into(),
            ParticleColour::Green => GREEN.into(),
            ParticleColour::Blue => BLUE.into(),
            ParticleColour::Orange => ORANGE.into(),
        }
    }
}

impl ParticleColour {
    fn index(&self) -> usize {
        match self {
            ParticleColour::Red => 0,
            ParticleColour::Green => 1,
            ParticleColour::Blue => 2,
            ParticleColour::Orange => 3,
        }
    }
}

#[derive(Debug, Reflect, Resource, Clone, Copy, Deref, DerefMut)]
pub struct Model {
    #[deref]
    pub weights: [[f32; 3]; 3],
}

#[derive(Debug, Reflect, Component, Default, Clone, Copy, Deref, DerefMut)]
pub struct Velocity(Vec2);

#[derive(Debug, Reflect, Component)]
#[require(Transform, ParticleColour, Velocity)]
pub struct Particle;

#[derive(Debug, Reflect, Resource, Clone, Copy)]
#[reflect(Resource)]
pub struct SimulationParams {
    pub friction: f32,
    pub force_factor: f32,
    pub peak_attraction_radius: f32,
    pub repulsion_radius: f32,
    pub max_distance: f32,
}

const INTERACTION_RADIUS: f32 = 75.0;

impl SimulationParams {
    const DEFAULT: Self = Self {
        friction: 2.0,
        force_factor: 100.0,
        peak_attraction_radius: 2.0 * INTERACTION_RADIUS / 3.0,
        repulsion_radius: INTERACTION_RADIUS / 3.0,
        max_distance: INTERACTION_RADIUS,
    };
}

#[derive(Debug, Event, Clone, Copy, Reflect)]
pub struct Respawn;

#[cfg_attr(feature = "hot_reload", hot)]
fn spawn_particles(mut commands: Commands, window: SimulationSize) {
    commands.insert_resource(SpatialIndex(SpatialHashGrid::new(
        Rect::from_center_size(Vec2::ZERO, window.dimensions()),
        (10, 10),
    )));
    commands.trigger(Respawn);
}

#[derive(Debug, Reflect, Resource, Deref, DerefMut)]
struct ParticleIndexes(Vec<Entity>);

fn respawn_particles(
    _trigger: Trigger<Respawn>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut particle_indexes: ResMut<ParticleIndexes>,
    simulation_size: SimulationSize,
    particles: Query<Entity, With<Particle>>,
) -> Result {
    particles
        .iter()
        .for_each(|entity| commands.entity(entity).despawn());

    let mesh = meshes.add(Circle::new(1.0));
    let red = materials.add(Color::from(RED));
    let green = materials.add(Color::from(GREEN));
    let blue = materials.add(Color::from(BLUE));
    let orange = materials.add(Color::from(ORANGE));

    let commands = RefCell::new(commands);

    let Vec2 {
        x: width,
        y: height,
    } = simulation_size.dimensions();

    particle_indexes.clear();
    (0..PARTICLES_PER_COLOR)
        .map(|_| {
            commands
                .borrow_mut()
                .spawn((
                    Particle,
                    Transform::from_xyz(
                        rand::random::<f32>() * width - width / 2.0,
                        rand::random::<f32>() * height - height / 2.0,
                        0.0,
                    ),
                    ParticleColour::Red,
                    Mesh2d(mesh.clone()),
                    MeshMaterial2d(red.clone()),
                ))
                .id()
        })
        .interleave((0..PARTICLES_PER_COLOR).map(|_| {
            commands
                .borrow_mut()
                .spawn((
                    Particle,
                    Transform::from_xyz(
                        rand::random::<f32>() * width - width / 2.0,
                        rand::random::<f32>() * height - height / 2.0,
                        0.0,
                    ),
                    ParticleColour::Green,
                    Mesh2d(mesh.clone()),
                    MeshMaterial2d(green.clone()),
                ))
                .id()
        }))
        .interleave((0..PARTICLES_PER_COLOR).map(|_| {
            commands
                .borrow_mut()
                .spawn((
                    Particle,
                    Transform::from_xyz(
                        rand::random::<f32>() * width - width / 2.0,
                        rand::random::<f32>() * height - height / 2.0,
                        0.0,
                    ),
                    ParticleColour::Blue,
                    Mesh2d(mesh.clone()),
                    MeshMaterial2d(blue.clone()),
                ))
                .id()
        }))
        // .chain((0..PARTICLES_PER_COLOR).map(|_| {
        //     commands
        //         .borrow_mut()
        //         .spawn((
        //             Particle,
        //             Transform::from_xyz(
        //                 rand::random::<f32>() * width - width / 2.0,
        //                 rand::random::<f32>() * height - height / 2.0,
        //                 0.0,
        //             ),
        //             ParticleColour::Orange,
        //             Mesh2d(mesh.clone()),
        //             MeshMaterial2d(orange.clone()),
        //         ))
        //         .id()
        // }))
        .collect_into(&mut **particle_indexes);

    Ok(())
}

fn toroidal_displacement(a: Vec2, b: Vec2, width: f32, height: f32) -> Vec2 {
    let mut dx = b.x - a.x;
    let mut dy = b.y - a.y;

    // Check if wrapping horizontally gives a shorter path
    if dx.abs() > width / 2.0 {
        dx = if dx > 0.0 { dx - width } else { dx + width };
    }

    // Check if wrapping vertically gives a shorter path
    if dy.abs() > height / 2.0 {
        dy = if dy > 0.0 { dy - height } else { dy + height };
    }

    Vec2::new(dx, dy)
}

fn toroidal_wrap(pos: Vec2, width: f32, height: f32) -> Vec2 {
    let mut pos = pos;

    if pos.x > width / 2.0 {
        pos.x -= width;
    } else if pos.x < -width / 2.0 {
        pos.x += width;
    }

    if pos.y > height / 2.0 {
        pos.y -= height;
    } else if pos.y < -height / 2.0 {
        pos.y += height;
    }

    pos
}

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct SpatialIndex(SpatialHashGrid<(Entity, ParticleColour)>);

#[cfg_attr(feature = "hot_reload", hot)]
fn compute_forces(
    mut particles: Query<(Entity, &mut Transform, &mut Velocity, &ParticleColour), With<Particle>>,
    mut spatial_index: ResMut<SpatialIndex>,
    model: Res<Model>,
    params: Res<SimulationParams>,
    simulation_size: SimulationSize,
    time: Res<Time>,
) -> Result<()> {
    let model = *model;
    let Vec2 {
        x: width,
        y: height,
    } = simulation_size.dimensions();

    spatial_index.clear();
    particles.iter().for_each(|(entity, transform, _, color)| {
        spatial_index.insert(transform.translation.truncate(), (entity, *color));
    });

    let dt = time.delta_secs();
    let friction_factor = (-params.friction * dt).exp();

    particles
        .par_iter_mut()
        .for_each(|(entity, mut transform, mut velocity, a_color)| {
            let force = spatial_index
                .query(transform.translation.truncate(), params.max_distance)
                .filter(|(_, (it, _))| *it != entity)
                .fold(Vec2::ZERO, |force, (b_position, (_, b_color))| {
                    let displacement = toroidal_displacement(
                        transform.translation.truncate(),
                        b_position,
                        width,
                        height,
                    );

                    let influence = influence(
                        &params,
                        model.weights[a_color.index()][b_color.index()],
                        displacement.length(),
                    );

                    force + displacement.normalize() * influence * params.force_factor
                });

            **velocity += force * dt;
            **velocity *= friction_factor;
            **velocity = velocity.clamp_length(0.0, 200.0);

            transform.translation = toroidal_wrap(
                transform.translation.truncate() + **velocity * dt,
                width,
                height,
            )
            .extend(0.0);
        });

    Ok(())
}

fn influence(params: &SimulationParams, factor: f32, distance: f32) -> f32 {
    if distance <= params.repulsion_radius {
        remap(distance, 0.0, params.repulsion_radius, -1.0, 0.0)
    } else if distance <= params.peak_attraction_radius {
        remap(
            distance,
            params.repulsion_radius,
            params.peak_attraction_radius,
            0.0,
            factor,
        )
    } else {
        remap(
            distance,
            params.peak_attraction_radius,
            params.max_distance,
            factor,
            0.0,
        )
    }
}

const DECAY_PER_SECOND: f32 = 100.0;

#[cfg_attr(feature = "hot_reload", hot)]
fn particle_decay(
    mut particles: Query<(&mut Transform, &mut Velocity), With<Particle>>,
    particle_indexes: Res<ParticleIndexes>,
    simulation_size: SimulationSize,
    time: Res<Time>,
    mut index: Local<usize>,
) -> Result<()> {
    let Vec2 {
        x: width,
        y: height,
    } = simulation_size.dimensions();

    let mut count = 0;
    while count < (DECAY_PER_SECOND * time.delta_secs()) as i32 {
        count += 1;

        let particle_index = particle_indexes.get(*index);
        *index = (*index + 1) % particle_indexes.len();

        let Some(particle_index) = particle_index else {
            return Ok(());
        };

        let (mut transform, mut velocity) = particles.get_mut(*particle_index)?;
        transform.translation = Vec2::new(
            rand::random::<f32>() * width - width / 2.0,
            rand::random::<f32>() * height - height / 2.0,
        )
        .extend(0.0);

        **velocity = Vec2::ZERO;
    }

    Ok(())
}

#[derive(SystemParam)]
pub struct SimulationSize<'w> {
    window: Single<'w, &'static Window>,
    // projection: Single<'w, &'static Projection>,
}

impl SimulationSize<'_> {
    pub fn dimensions(&self) -> Vec2 {
        let (width, height) = (self.window.width(), self.window.height());
        // let Projection::Orthographic(OrthographicProjection { scale, ..}) = *self.projection else {
        //     panic!("Projection is not orthographic");
        // };

        Vec2::new(width, height)
    }
}

#[cfg(test)]
mod test {
    mod toroidal_displacement {

        use bevy::math::Vec2;

        use crate::particles::toroidal_displacement;

        #[test]
        fn zero() {
            assert_eq!(
                toroidal_displacement(Vec2::ZERO, Vec2::ZERO, 100.0, 100.0),
                Vec2::ZERO
            );
        }

        #[test]
        fn diagonal() {
            assert_eq!(
                toroidal_displacement(Vec2::ZERO, Vec2::splat(25.0), 100.0, 100.0),
                Vec2::splat(25.0)
            );
        }

        #[test]
        fn wrap_x() {
            assert_eq!(
                toroidal_displacement(Vec2::ZERO, Vec2::new(75.0, 0.0), 100.0, 100.0),
                Vec2::new(-25.0, 0.0)
            );
        }

        #[test]
        fn wrap_y() {
            assert_eq!(
                toroidal_displacement(Vec2::ZERO, Vec2::new(0.0, 75.0), 100.0, 100.0),
                Vec2::new(0.0, -25.0)
            );
        }

        #[test]
        fn wrap_xy() {
            assert_eq!(
                toroidal_displacement(Vec2::ZERO, Vec2::new(75.0, 75.0), 100.0, 100.0),
                Vec2::new(-25.0, -25.0)
            );
        }
    }

    mod toroidal_wrap {
        use bevy::math::Vec2;

        use crate::particles::toroidal_wrap;

        #[test]
        fn zero() {
            assert_eq!(toroidal_wrap(Vec2::ZERO, 100.0, 100.0), Vec2::ZERO);
        }

        #[test]
        fn within_bounds() {
            assert_eq!(
                toroidal_wrap(Vec2::new(25.0, 25.0), 100.0, 100.0),
                Vec2::new(25.0, 25.0)
            );
        }

        #[test]
        fn within_bounds_negative() {
            assert_eq!(
                toroidal_wrap(Vec2::new(-25.0, -25.0), 100.0, 100.0),
                Vec2::new(-25.0, -25.0)
            );
        }

        #[test]
        fn wrap_x() {
            assert_eq!(
                toroidal_wrap(Vec2::new(75.0, 0.0), 100.0, 100.0),
                Vec2::new(-25.0, 0.0)
            );
        }

        #[test]
        fn wrap_y() {
            assert_eq!(
                toroidal_wrap(Vec2::new(0.0, 75.0), 100.0, 100.0),
                Vec2::new(0.0, -25.0)
            );
        }

        #[test]
        fn wrap_xy() {
            assert_eq!(
                toroidal_wrap(Vec2::new(75.0, 75.0), 100.0, 100.0),
                Vec2::new(-25.0, -25.0)
            );
        }
    }

    mod influence {
        use crate::{
            math::lerp,
            particles::{SimulationParams, influence},
        };

        fn approx_eq(a: f32, b: f32) -> bool {
            (a - b).abs() < 0.00001
        }

        #[test]
        fn repulsed_self() {
            assert!(
                approx_eq(influence(&SimulationParams::DEFAULT, 1.0, 0.0), -1.0),
                "Expected influence to be -1.0, but got {}",
                influence(&SimulationParams::DEFAULT, 1.0, 0.0)
            );
        }

        #[test]
        fn repulsed_other() {
            assert!(
                approx_eq(
                    influence(
                        &SimulationParams::DEFAULT,
                        -1.0,
                        SimulationParams::DEFAULT.peak_attraction_radius
                    ),
                    -1.0
                ),
                "Expected influence to be -1.0, but got {}",
                influence(
                    &SimulationParams::DEFAULT,
                    -1.0,
                    SimulationParams::DEFAULT.peak_attraction_radius
                )
            );
        }

        #[test]
        fn balanced() {
            assert!(
                approx_eq(
                    influence(
                        &SimulationParams::DEFAULT,
                        1.0,
                        SimulationParams::DEFAULT.repulsion_radius
                    ),
                    0.0
                ),
                "Expected influence to be 0.0, but got {}",
                influence(
                    &SimulationParams::DEFAULT,
                    1.0,
                    SimulationParams::DEFAULT.repulsion_radius
                )
            );
        }

        #[test]
        fn attracted() {
            assert!(
                approx_eq(
                    influence(
                        &SimulationParams::DEFAULT,
                        1.0,
                        SimulationParams::DEFAULT.peak_attraction_radius
                    ),
                    1.0
                ),
                "Expected influence to be 1.0, but got {}",
                influence(
                    &SimulationParams::DEFAULT,
                    1.0,
                    SimulationParams::DEFAULT.peak_attraction_radius
                )
            );
        }

        #[test]
        fn halfway_repulsed() {
            assert!(
                approx_eq(
                    influence(
                        &SimulationParams::DEFAULT,
                        1.0,
                        SimulationParams::DEFAULT.repulsion_radius / 2.0
                    ),
                    -0.5
                ),
                "Expected influence to be -0.5, but got {}",
                influence(
                    &SimulationParams::DEFAULT,
                    1.0,
                    SimulationParams::DEFAULT.repulsion_radius / 2.0
                )
            );
        }

        #[test]
        fn halfway_attracted() {
            assert!(
                approx_eq(
                    influence(
                        &SimulationParams::DEFAULT,
                        1.0,
                        lerp(
                            SimulationParams::DEFAULT.repulsion_radius,
                            SimulationParams::DEFAULT.peak_attraction_radius,
                            0.5
                        )
                    ),
                    0.5
                ),
                "Expected influence to be 0.5, but got {}",
                influence(
                    &SimulationParams::DEFAULT,
                    1.0,
                    lerp(
                        SimulationParams::DEFAULT.repulsion_radius,
                        SimulationParams::DEFAULT.peak_attraction_radius,
                        0.5
                    )
                )
            );
        }

        #[test]
        fn halfway_attracted_otherside() {
            assert!(
                approx_eq(
                    influence(
                        &SimulationParams::DEFAULT,
                        1.0,
                        lerp(
                            SimulationParams::DEFAULT.peak_attraction_radius,
                            SimulationParams::DEFAULT.max_distance,
                            0.5
                        )
                    ),
                    0.5
                ),
                "Expected influence to be 0.5, but got {}",
                influence(
                    &SimulationParams::DEFAULT,
                    1.0,
                    lerp(
                        SimulationParams::DEFAULT.peak_attraction_radius,
                        SimulationParams::DEFAULT.max_distance,
                        0.5
                    )
                )
            );
        }
    }
}
