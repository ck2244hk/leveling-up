use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ecs_ldtk::{
    assets::{LdtkProject, LevelMetadataAccessor},
    LevelIid, LevelSelection,
};

use crate::{
    game::{
        character::component::{BaseStates, Hero},
        map::{preload::Location, ChangeMapEvent},
        player::{DecendingHero, Player, PLAYER_START_POSITION},
    },
    helper::{SmoothDamp, Velocity, Zooming},
    state::{Screen, SimulationState},
};

use super::MainCamera;

const ASPECT_RATIO: f32 = 1.;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), init_camera_position)
        .add_systems(
            Update,
            update_smooth_damp_camera_w_decending.run_if(in_state(Screen::Playing)),
        )
        .add_systems(
            Update,
            (
                update_smooth_damp_camera,
                update_camera_w_player,
                spawn_camera_position,
                zoom_in,
                zoom_w_camera,
                // camera_fit_inside_current_level.after(update_camera_w_player),
            )
                .run_if(in_state(SimulationState::Running))
                .run_if(in_state(Screen::Playing)),
        )
        .register_type::<Zooming>();
}

pub fn zoom_w_camera(
    mut player_query: Query<&mut Zooming, With<Camera>>,
    hero_query: Query<&BaseStates, With<Hero>>,
) {
    let Ok((mut zoom)) = player_query.get_single_mut() else {
        return;
    };
    if let Ok(base_state) = hero_query.get_single() {
        zoom.0 = 0.1_f32.max(base_state.lv_f32() * 0.02);
    }
}

pub fn zoom_in(
    mut query: Query<(&mut OrthographicProjection, &Zooming), With<Camera>>,
    time: Res<Time>,
) {
    for (mut projection, zooming) in query.iter_mut() {
        match projection.scale > zooming.0 {
            true => {
                let mut log_scale = projection.scale.ln();
                log_scale -= 0.3 * time.delta_seconds();
                projection.scale = zooming.0.max(log_scale.exp());

                println!("Current zoom scale: {}", projection.scale);
            }
            false => {
                let mut log_scale = projection.scale.ln();
                log_scale += 0.3 * time.delta_seconds();
                projection.scale = zooming.0.min(log_scale.exp());

                println!("Current zoom scale: {}", projection.scale);
            }
        }
    }
}

pub fn init_camera_position(mut camera_query: Query<&mut Transform, With<MainCamera>>) {
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        camera_transform.translation = PLAYER_START_POSITION;
    }
}

pub fn update_camera_w_player(
    mut camera_query: Query<(&mut Transform, &Velocity), With<MainCamera>>,
) {
    if let Ok((mut camera_transform, velocity)) = camera_query.get_single_mut() {
        camera_transform.translation += velocity.0.extend(0.);
    }
}

pub fn update_smooth_damp_camera_w_decending(
    player_query: Query<&Transform, (With<DecendingHero>, Without<MainCamera>)>,
    mut camera_query: Query<(&mut Transform, &mut SmoothDamp, &mut Velocity), With<MainCamera>>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        // info!("No player found");
        return;
    };
    let Ok((camera_transform, mut smooth_damp, mut velocity)) = camera_query.get_single_mut()
    else {
        // info!("No camera found");
        return;
    };

    smooth_damp.update_acceleration(
        camera_transform.translation.truncate(),
        player_transform.translation.truncate(),
        velocity.0.length(),
        time.delta_seconds(),
    );

    velocity.0 = smooth_damp.acceleration;
}

pub fn update_smooth_damp_camera(
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera_query: Query<(&mut Transform, &mut SmoothDamp, &mut Velocity), With<MainCamera>>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok((camera_transform, mut smooth_damp, mut velocity)) = camera_query.get_single_mut()
        {
            smooth_damp.update_acceleration(
                camera_transform.translation.truncate(),
                player_transform.translation.truncate(),
                velocity.0.length(),
                time.delta_seconds(),
            );

            velocity.0 = smooth_damp.acceleration;
        }
    }
}

pub fn spawn_camera_position(
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    location_query: Query<(&Transform, &Location), (With<Location>, Without<MainCamera>)>,
    mut event_reader: EventReader<ChangeMapEvent>,
) {
    for event in event_reader.read() {
        if let Some((destination, location)) = location_query
            .iter()
            .find(|(_, location)| location.equal(&event.location))
        {
            let Ok(mut camera_transform) = camera_query.get_single_mut() else {
                warn!("Player is not found during map switching");
                continue;
            };

            camera_transform.translation = destination.translation;
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn camera_fit_inside_current_level(
    mut camera_query: Query<(&mut Transform), (Without<Player>, With<MainCamera>)>,
    player_query: Query<&Transform, With<Player>>,
    level_query: Query<(&Transform, &LevelIid), (Without<MainCamera>, Without<Player>)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    level_selection: Res<LevelSelection>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single()
    else {
        return;
    };

    let player_translation = player_translation;
    let window = window_query.get_single().unwrap();
    let aspect_ratio = window.width() / window.height();

    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        return;
    };

    for (level_transform, level_iid) in &level_query {
        let ldtk_project = ldtk_project_assets
            .get(ldtk_projects.single())
            .expect("Project should be loaded if level has spawned");

        let level = ldtk_project
            .get_raw_level_by_iid(&level_iid.to_string())
            .expect("Spawned level should exist in LDtk project");

        // if level_selection.is_match(&LevelIndices::default(), level) {
        let adjust_window_height = window.height() - 200.;
        let height = adjust_window_height / 2.;
        let width = window.width() / 2. - 75.;

        // orthographic_projection.scaling_mode =
        //     bevy::render::camera::ScalingMode::Fixed { width, height };
        // info!("Level tranform: {}, level px width: {}, Levle px Height: {}, Window Width: {}, Height: {}", level_transform.translation, level.px_wid, level.px_hei, width, height);

        camera_transform.translation.x = if window.width() >= level.px_wid as f32 {
            camera_transform.translation.x + width - level.px_wid as f32
        } else {
            camera_transform.translation.x.clamp(
                0. + width,
                level_transform.translation.x + level.px_wid as f32 - width,
            )
        };

        camera_transform.translation.y = if adjust_window_height >= level.px_hei as f32 {
            camera_transform.translation.y + height - level.px_hei as f32
        } else {
            camera_transform.translation.y.clamp(
                0. + height - 50.,
                level_transform.translation.y + level.px_hei as f32 - height + 75.,
            )
        };

        // camera_transform.translation.x += level_transform.translation.x;
        // camera_transform.translation.y += level_transform.translation.y;
    }
}
// }
