// mod animation;
mod animation;
mod assets;
mod audio;
mod audio_effects;
mod audio_helper;
mod camera;
// mod demo;
#[cfg(feature = "dev")]
mod dev_tools;
mod error;
mod game;
mod helper;
mod model;
mod overlay;
mod preload;
mod screens;
mod shader;
mod shader_util;
mod state;
mod theme;
mod util;

use animation::AnimationPlugin;
use bevy::{
    asset::AssetMetaCheck,
    audio::{AudioPlugin, Volume},
    prelude::*,
};
use bevy_rapier2d::plugin::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
use overlay::GeneralOverlayPlugin;
use shader::ShaderPlugin;
const IPHONE_BAND_SPACER_HEIGHT: f32 = 92_f32;
const RAPIER_PIXELS_PER_METER: f32 = 100_f32;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Order new `AppStep` variants by adding them here:
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            RAPIER_PIXELS_PER_METER,
        ))
        .insert_resource(RapierConfiguration::new(0.))
        .configure_sets(
            Update,
            (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
        );

        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Leveling Up".to_string(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AudioPlugin {
                    global_volume: GlobalVolume {
                        volume: Volume::new(0.3),
                    },
                    ..default()
                }),
        );

        // Add other plugins.
        app.add_plugins((
            // demo::plugin,
            screens::plugin,
            theme::plugin,
            assets::plugin,
            audio::plugin,
            preload::plugin,
            state::plugin,
            camera::plugin,
            ShaderPlugin,
            AnimationPlugin,
            GeneralOverlayPlugin,
            game::plugin,
        ));

        #[cfg(not(target_family = "wasm"))]
        {
            use bevy::input::common_conditions::input_toggle_active;
            use bevy_inspector_egui::quick::WorldInspectorPlugin;
            app.add_plugins(
                WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Tab)),
            );
        }

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSet {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}
