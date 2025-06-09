#![feature(iter_collect_into)]
#![feature(coroutines)]
#![feature(gen_blocks)]
#![feature(trait_alias)]

use bevy::{
    asset::{AssetMetaCheck, load_internal_binary_asset},
    prelude::*,
    window::WindowResolution,
};
use bevy_tweening::{AnimationSystem, TweeningPlugin};
use particles::ParticlePlugin;
use ui::UIPlugin;

use crate::{
    browser_state::BrowserStatePlugin, camera::CameraPlugin, controls::ControlsPlugin,
    scenes::ScenePlugin, systems::AppSystems,
};

mod browser_state;
mod bundle_fn;
mod camera;
mod controls;
mod math;
mod observe;
mod particles;
mod scenes;
mod spatial_hash;
mod systems;
mod ui;

const CLEAR_COLOUR: Color = Color::srgb_from_array([44.0 / 255.0, 30.0 / 255.0, 49.0 / 255.0]);

fn main() -> AppExit {
    let mut app = App::new();

    // Bevy Plugins;
    bevy_systems(&mut app);
    third_party_systems(&mut app);
    app_systems(&mut app);

    app.run()
}

fn bevy_systems(app: &mut App) {
    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Window {
                    title: "ABIOGENESIS".into(),
                    // // #[cfg(not(target_arch = "wasm32"))]
                    // resolution: WindowResolution::new(2.0 * 362.0, 688.0),
                    fit_canvas_to_parent: true,
                    ..default()
                }
                .into(),
                ..default()
            }),
    )
    .insert_resource(ClearColor(CLEAR_COLOUR));

    #[cfg(feature = "debug_ui")]
    app.insert_resource(UiDebugOptions {
        enabled: true,
        ..default()
    });

    #[cfg(all(debug_assertions, not(feature = "hot_reload")))]
    app.edit_schedule(Update, |schedule| {
        schedule.set_build_settings(ScheduleBuildSettings {
            ambiguity_detection: LogLevel::Warn,
            ..default()
        });
    });
}

fn third_party_systems(app: &mut App) {
    app.add_plugins((
        TweeningPlugin,
        #[cfg(feature = "egui")]
        (
            bevy_inspector_egui::bevy_egui::EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
        ),
    ));

    #[cfg(feature = "hot_reload")]
    app.add_plugins(bevy_simple_subsecond_system::SimpleSubsecondPlugin::default());
}

fn app_systems(app: &mut App) {
    app.add_plugins((
        ParticlePlugin,
        UIPlugin,
        ScenePlugin,
        CameraPlugin,
        ControlsPlugin,
        BrowserStatePlugin,
    ));

    app.configure_sets(
        Update,
        (
            AppSystems::TickTimers,
            AnimationSystem::AnimationUpdate,
            AppSystems::RecordInput,
            AppSystems::Update,
            AppSystems::Camera,
        )
            .chain(),
    );

    load_internal_binary_asset!(
        app,
        TextFont::default().font,
        "../assets/fonts/CreatoDisplay-Bold.otf",
        |bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
    );
}
