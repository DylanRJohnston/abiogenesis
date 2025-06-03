#![feature(iter_collect_into)]
#![feature(coroutines)]
#![feature(gen_blocks)]

use bevy::{asset::AssetMetaCheck, prelude::*, window::WindowResolution};
use bevy_tweening::TweeningPlugin;
use particles::ParticlePlugin;
use ui::UIPlugin;

#[cfg(feature = "hot_reload")]
use bevy_simple_subsecond_system::SimpleSubsecondPlugin;

mod math;
mod observe;
mod particles;
mod spatial_hash;
mod ui;

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    RecordInput,
    Update,
}

const CLEAR_COLOUR: Color = Color::srgb_from_array([44.0 / 255.0, 30.0 / 255.0, 49.0 / 255.0]);

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Window {
                    title: "Abiogenesis".into(),
                    resolution: WindowResolution::new(1920.0, 1080.0),
                    fit_canvas_to_parent: true,
                    ..default()
                }
                .into(),
                ..default()
            }),
    )
    .add_plugins((
        TweeningPlugin,
        #[cfg(feature = "dev_native")]
        (
            bevy_inspector_egui::bevy_egui::EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
        ),
    ))
    .insert_resource(ClearColor(CLEAR_COLOUR))
    .configure_sets(
        Update,
        (AppSystems::RecordInput, AppSystems::Update).chain(),
    )
    .add_plugins((ParticlePlugin, UIPlugin))
    .add_systems(Startup, spawn_camera);

    #[cfg(feature = "hot_reload")]
    app.add_plugins(SimpleSubsecondPlugin::default());

    app.run()
}

#[cfg_attr(
    feature = "hot_reload",
    bevy_simple_subsecond_system::hot(rerun_on_hot_patch = true)
)]
fn spawn_camera(
    mut commands: Commands,
    #[cfg(feature = "hot_reload")] cameras: Query<Entity, With<Camera>>,
) {
    #[cfg(feature = "hot_reload")]
    cameras
        .iter()
        .for_each(|camera| commands.entity(camera).despawn());

    commands.spawn((
        Name::from("Camera"),
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 1.0,
            ..OrthographicProjection::default_2d()
        }),
        Camera {
            // hdr: true,
            ..default()
        },
        // Tonemapping::AcesFitted,
        // Bloom::default(),
        // DebandDither::Enabled,
    ));
}
