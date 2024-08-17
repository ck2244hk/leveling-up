use bevy::{input::touch::TouchPhase, window::PrimaryWindow};
use bevy_ecs_ldtk::{
    assets::{LdtkProject, LevelMetadataAccessor},
    LevelIid,
};

use super::*;
use crate::game::{
    character::component::{BaseStates, Hero},
    joystick::{
        create_joystick, JoystickFixed, NoAction, VirtualJoystickEvent, VirtualJoystickNode,
    },
};

pub fn spawn_joystick(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn Virtual Joystick at horizontal center using helper function
    create_joystick(
        &mut commands,
        asset_server.load("joystick/Knob.png"),
        asset_server.load("joystick/Outline.png"),
        Color::WHITE.into(),
        None,
        None,
        Vec2::new(40., 40.),
        Vec2::new(80., 80.),
        Style {
            width: Val::Px(150.),
            height: Val::Px(150.),
            position_type: PositionType::Absolute,
            left: Val::Percent(62.),
            bottom: Val::Percent(8.),
            ..default()
        },
        JoystickFixed,
        NoAction,
    );
}

pub fn update_joystick(
    mut joystick: EventReader<VirtualJoystickEvent>,
    mut player_query: Query<(&mut Velocity, &mut CurrentMovement, &mut FacingSide), With<Player>>,
    time_step: Res<Time>,
) {
    for j in joystick.read() {
        let (mut velocity, mut current_movement, mut face_side) = player_query.single_mut();
        // info!("joystick: {:?}", j);
        let Vec2 { x, y } = j.snap_axis(Some(0.3));
        let speed: f32 = 170.;

        velocity.linvel.x = x * speed;
        velocity.linvel.y = y * speed;

        if x > 0. {
            face_side.set_if_neq(FacingSide::Right);
        } else {
            face_side.set_if_neq(FacingSide::Left);
        }

        if x == 0. && y == 0. {
            current_movement.set_if_neq(CurrentMovement(Movement::Idle));
        } else {
            current_movement.set_if_neq(CurrentMovement(Movement::Run));
        }
    }
}

pub fn movement_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Velocity, &mut CurrentMovement, &mut FacingSide), With<Player>>,
    time: Res<Time>,
    hero_query: Query<(&BaseStates), With<Hero>>,
) {
    // #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
    if let Ok((mut velocity, mut current_movement, mut face_side)) = player_query.get_single_mut() {
        let Ok(state) = hero_query.get_single() else {
            return;
        };
        let speed: f32 = 30_f32.max(300_f32.min(200_f32 * state.lv_f32() / 100.));
        if keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
            velocity.linvel.y = speed;
            current_movement.set_if_neq(CurrentMovement(Movement::Run));
        }
        if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
            velocity.linvel.y = -speed;
            current_movement.set_if_neq(CurrentMovement(Movement::Run));
        }
        if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
            velocity.linvel.x = speed;
            current_movement.set_if_neq(CurrentMovement(Movement::Run));
            face_side.set_if_neq(FacingSide::Right);
        }
        if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
            velocity.linvel.x = -speed;
            current_movement.set_if_neq(CurrentMovement(Movement::Run));
            face_side.set_if_neq(FacingSide::Left);
        }
    }
}

// pub fn touch_movement_input(
//     touches: Res<Touches>,
//     mut player_query: Query<(&mut Transform, &mut CurrentMovement, &mut FacingSide), With<Player>>,
//     window_query: Query<&Window>,
//     time: Res<Time>,
// ) {
//     #[cfg(any(target_os = "ios", target_os = "android"))]
//     if let Ok((mut player_transform, mut current_movement, mut face_side)) =
//         player_query.get_single_mut()
//     {
//         let window = window_query.get_single().expect("Camera Not Found");
//         let center = Vec2::new(window.width() / 2.0, window.height() / 2.0);
//         let dead_zone_radias: f32 = 50.;
//         let walking_radias: f32 = 100.;

//         for finger in touches.iter() {
//             let distance = finger.position().distance(center);
//             let speed = if distance > dead_zone_radias && distance < walking_radias {
//                 100.
//             } else if distance >= walking_radias {
//                 200.
//             } else {
//                 0.
//             };

//             let direction = (center - finger.position()).normalize() * Vec2::new(-1., 1.);

//             if direction.x >= 0. {
//                 face_side.set_if_neq(FacingSide::Right);
//             } else {
//                 face_side.set_if_neq(FacingSide::Left);
//             }

//             player_transform.translation += direction.extend(0.) * speed * time.delta_seconds();
//             current_movement.set_if_neq(CurrentMovement(Movement::Run));
//         }
//     }
// }

#[allow(warnings)]
pub fn update_idle(
    mut touch_evr: EventReader<TouchInput>,
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut CurrentMovement, &mut FacingSide), With<Player>>,
) {
    if let Ok((_player_transform, mut current_movement, _face_side)) = player_query.get_single_mut()
    {
        #[cfg(any(target_os = "ios", target_os = "android"))]
        for ev in touch_evr.read() {
            // in real apps you probably want to store and track touch ids somewhere
            match ev.phase {
                TouchPhase::Ended => {
                    current_movement.set_if_neq(CurrentMovement(Movement::Idle));
                }
                TouchPhase::Canceled => {
                    current_movement.set_if_neq(CurrentMovement(Movement::Idle));
                }
                _ => (),
            }
        }

        // #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
        if !keys.any_pressed([
            KeyCode::ArrowLeft,
            KeyCode::KeyA,
            KeyCode::ArrowRight,
            KeyCode::KeyD,
            KeyCode::ArrowDown,
            KeyCode::KeyS,
            KeyCode::ArrowUp,
            KeyCode::KeyW,
        ]) && !buttons.any_pressed([MouseButton::Left, MouseButton::Right])
        {
            current_movement.set_if_neq(CurrentMovement(Movement::Idle));
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn player_fit_inside_current_level(
    mut player_query: Query<&mut Transform, With<Player>>,
    level_query: Query<(&Transform, &LevelIid), (Without<OrthographicProjection>, Without<Player>)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(player_tranform) = player_query.get_single_mut() {
        let mut player_translation = player_tranform.translation;
        let window = window_query.get_single().unwrap();
        let aspect_ratio = window.width() / window.height();

        for (level_transform, level_iid) in &level_query {
            let ldtk_project = ldtk_project_assets
                .get(ldtk_projects.single())
                .expect("Project should be loaded if level has spawned");

            let level = ldtk_project
                .get_raw_level_by_iid(&level_iid.to_string())
                .expect("Spawned level should exist in LDtk project");

            // if level_selection.is_match(&LevelIndices::default(), level) {
            let height = 16.;
            let width = 16.;

            // orthographic_projection.scaling_mode =
            //     bevy::render::camera::ScalingMode::Fixed { width, height };
            // info!("Level tranform: {}, level px width: {}, Levle px Height: {}, Window Width: {}, Height: {}", level_transform.translation, level.px_wid, level.px_hei, width, height);

            player_translation.x = if 32. >= level.px_wid as f32 {
                player_translation.x + width - level.px_wid as f32
            } else {
                player_translation.x.clamp(
                    0. + width,
                    level_transform.translation.x + level.px_wid as f32 - width,
                )
            };

            player_translation.y = if 32. >= level.px_hei as f32 {
                player_translation.y + height - level.px_hei as f32
            } else {
                player_translation.y.clamp(
                    0. + height,
                    level_transform.translation.y + level.px_hei as f32 - height,
                )
            };

            // camera_transform.translation.x += level_transform.translation.x;
            // camera_transform.translation.y += level_transform.translation.y;
        }
        // }
    }
}

pub fn despawn_joystick(
    mut commands: Commands,
    joystick_query: Query<Entity, With<VirtualJoystickNode>>,
) {
    for joystick in joystick_query.iter() {
        commands.entity(joystick).despawn_recursive();
    }
}
