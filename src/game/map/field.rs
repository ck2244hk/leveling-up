use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ecs_ldtk::LevelSelection;
use rand::random;

use crate::camera::game::camera_fit_inside_current_level;
use crate::state::{Screen, SimulationState};

use crate::game::character::component::{BaseStates, Hero};
use crate::game::{CurrentMovement, Encounter, Movement, Player};

use super::preload::{CollisionDetection, Field, ShapeWrapper};

#[derive(Event)]
pub struct EncounterEvent {
    pub monster_id: Vec<i32>,
    pub lv: u32,
}

// FieldIndicator contain the location of each field
#[derive(Component)]
pub struct FieldIndicator {
    location: Vec2,
    level: u32,
}

impl FieldIndicator {
    fn build_test(lv: u32) -> Self {
        Self {
            location: Vec2::ZERO,
            level: lv,
        }
    }
}

pub struct FieldPlugin;

impl Plugin for FieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EncounterEvent>()
            .add_systems(
                Update,
                (
                    detect_in_danger,
                    tick_encounter_timer,
                    encounter_monster,
                    spawn_zone_lv,
                    update_level_indicator_position.after(camera_fit_inside_current_level),
                    // constrict_level_indicator.after(update_level_indicator_position),
                    despawn_danger_indicator_change_lv,
                    clear_in_danger,
                    spawn_mesh,
                )
                    .run_if(in_state(SimulationState::Running))
                    .run_if(in_state(Screen::Playing)),
            )
            .add_systems(OnExit(Screen::GameOver), despawn_danger_indicator);
    }
}

// pub fn spawn_collider_shape(
//     mut commands: Commands,
//     shape_query: Query<(&Collider, Entity), Added<Collider>>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     for (collider, entity) in shape_query.iter() {
//         collider.raw

//         let mesh = commands
//             .spawn(MaterialMesh2dBundle {
//                 mesh: meshes.add(shape.0.clone()).into(),
//                 material: materials.add(Color::rgb(0.5, 0.2, 0.2)),
//                 ..default()
//             })
//             .id();
//         commands.entity(entity).add_child(mesh);
//     }
// }

pub fn spawn_mesh(
    // mut commands: Commands,
    shape_query: Query<(&ShapeWrapper, &Transform), With<ShapeWrapper>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    mut gizmos: Gizmos,
) {
    for (shape, transform) in shape_query.iter() {
        // info!("Drawing Line: {}", shape.0.clone());
        // let mesh = commands
        //     .spawn((MaterialMesh2dBundle {
        //         mesh: meshes.add(shape.0.clone()).into(),
        //         material: materials.add(Color::rgba_u8(150, 70, 70, 20)),
        //         ..default()
        //     },))
        //     .id();
        // commands.entity(entity).add_child(mesh);

        gizmos.rect_2d(
            transform.translation.truncate(),
            0.,
            shape.0.clone(),
            Color::linear_rgb(1.0, 0., 0.),
        );
    }
}

#[derive(Component)]
pub struct EncounterTimer(Timer);

impl Default for EncounterTimer {
    fn default() -> Self {
        let timer = Timer::from_seconds(1., TimerMode::Repeating);
        EncounterTimer(timer)
    }
}

fn spawn_zone_lv(mut commands: Commands, area_query: Query<(&Transform, &Field), Added<Field>>) {
    for (transform, zone) in area_query.iter() {
        commands.spawn((
            Text2dBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: format!("{}Lv", zone.lv()),
                        style: TextStyle {
                            font_size: 20_f32,
                            ..default()
                        },
                        ..Default::default()
                    }],
                    ..default()
                },

                transform: Transform::from_translation(transform.translation),
                ..default()
            },
            FieldIndicator {
                location: transform.translation.truncate(),
                level: zone.lv(),
            },
        ));
    }
}

pub fn detect_in_danger(
    mut player_query: Query<&mut Encounter, With<Player>>,
    field_query: Query<
        (&mut CollisionDetection, &Field),
        (Changed<CollisionDetection>, With<Field>),
    >,
) {
    for (collision, field) in field_query.iter() {
        let Ok(mut player) = player_query.get_single_mut() else {
            continue;
        };

        info!("Player was on field {:?}", player.field);
        match (collision.0, player.field.is_some_and(|x| x == field.level)) {
            // stepping to a new field with the same ID, happen when polygon field
            (true, true) => {
                info!(
                    "Colliding with {} and id is same as the last one",
                    field.level
                );
            }
            // Walk to a new zone, this method simply takes the newer field
            (true, false) => {
                // player.reset();
                player.update(field.level);
                info!(
                    "Colliding with {} and id is not the same as the last one",
                    field.level
                );
            }
            // Walking out of a zone
            (false, true) => {
                info!(
                    "Stop Colliding with {} which was coliding before",
                    field.level
                )
            }
            // The player have walked into a new field already while walking out the old zone
            (false, false) => {
                // player.reset();
                info!(
                    "Steping out Field lv {}, while player was not encounting on that before",
                    field.level
                )
            }
        }
        info!("Player currently is on field {:?}", player.field);
    }
}

pub fn clear_in_danger(
    mut player_query: Query<&mut Encounter, With<Player>>,
    field_query: Query<&mut CollisionDetection, (With<CollisionDetection>, With<Field>)>,
) {
    if !field_query.iter().any(|collision| collision.0) {
        if let Ok(mut player) = player_query.get_single_mut() {
            // player.reset();
            player.field = None;
        }
    }
}

pub fn tick_encounter_timer(
    mut timer: Query<(&mut EncounterTimer, &mut Encounter)>,
    time: Res<Time>,
) {
    for (mut timer, mut encounter) in timer.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            encounter.immune = true;
        }
    }
}

pub fn encounter_monster(
    mut zone_query: Query<(&Field, &CollisionDetection), With<Field>>,
    mut player_query: Query<(&CurrentMovement, &mut Encounter), With<Player>>,
    mut encounter_event_writer: EventWriter<EncounterEvent>,
    time: Res<Time>,
) {
    let (player_movemnt, mut encounter) = player_query.get_single_mut().expect("No Player Spawned");

    let player_field = encounter.field;

    for (field, in_danger_detection) in zone_query
        .iter_mut()
        .filter(|(field, _)| player_field.is_some_and(|lv| field.level == lv))
    {
        if in_danger_detection.0 && player_movemnt.0 == Movement::Run {
            if encounter.immune {
                let seed = random::<f32>() * 100.;

                if seed <= encounter.percentage {
                    encounter_event_writer.send(EncounterEvent {
                        monster_id: field.monster_id.clone(),
                        lv: field.lv(),
                    });
                    info!("Encounter a Monster");
                    encounter.reset();
                }

                // reset encounter to false to prevent encountering 2 monster at once
                encounter.immune = false;
            }

            encounter.percentage += 50. * time.delta_seconds();
            // info!("Encounter percentage: {}", encounter.percentage);
        }
    }
}

// Todo: Later separate the get cloest three to a separate function to reduce computational cost
pub fn constrict_level_indicator(
    hero_query: Query<&BaseStates, With<Hero>>,
    center_query: Query<&Transform, With<Camera>>,
    mut field_query: Query<
        (&mut Transform, &FieldIndicator),
        (With<FieldIndicator>, Without<Camera>),
    >,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(base_state) = hero_query.get_single() {
        let mut fields = field_query.iter_mut().collect::<Vec<_>>();

        let mut filtered = fields
            .iter_mut()
            .filter(|(_, ind)| ind.level >= base_state.lv())
            .collect::<Vec<_>>();

        filtered.sort_by(|(_, a_ind), (_, b_ind)| {
            (a_ind.level - base_state.lv()).cmp(&(b_ind.level - base_state.lv()))
        });

        // Only take the first 4 zone which are the closest to hero level
        for danger_zones_transform in filtered.iter_mut().take(4) {
            let Ok(window) = window_query.get_single() else {
                continue;
            };
            let Ok(center_transform) = center_query.get_single() else {
                continue;
            };

            let center = center_transform.translation;

            // let horizontal_padding: f32 = 90.;
            // let bottom_padding: f32 = 150.;
            // let top_padding: f32 = 190.;

            //if camera scale = 0.
            let horizontal_padding: f32 = 10.;
            let bottom_padding: f32 = 15.;
            let top_padding: f32 = 50.;

            let height_min = center.y - window.height() * 0.65 / 2. + bottom_padding;
            let height_max = center.y + window.height() * 0.65 / 2. - top_padding;
            let width_min = center.x - window.width() * 0.65 / 2. + horizontal_padding;
            let width_max = center.x + window.width() * 0.65 / 2. - horizontal_padding;

            if danger_zones_transform.0.translation.y < height_min {
                danger_zones_transform.0.translation.y = height_min;
            } else if danger_zones_transform.0.translation.y > height_max {
                danger_zones_transform.0.translation.y = height_max;
            }
            if danger_zones_transform.0.translation.x < width_min {
                danger_zones_transform.0.translation.x = width_min;
            } else if danger_zones_transform.0.translation.x > width_max {
                danger_zones_transform.0.translation.x = width_max;
            }

            // danger_zones_transform.translation.y =
            //     height_min.max(height_max.min(danger_zones_transform.translation.y));

            // danger_zones_transform.translation.x =
            //     width_min.max(width_max.min(danger_zones_transform.translation.x));
        }
    }
}

pub fn update_level_indicator_position(
    mut danger_zones_query: Query<(&mut Transform, &FieldIndicator), With<FieldIndicator>>,
) {
    for (mut danger_zones_transform, indicator) in danger_zones_query.iter_mut() {
        danger_zones_transform.translation = indicator.location.extend(9.);
    }
}

pub fn despawn_danger_indicator(
    mut commands: Commands,
    indicator_query: Query<Entity, With<FieldIndicator>>,
) {
    for indicator in indicator_query.iter() {
        commands.entity(indicator).despawn();
    }
}

pub fn despawn_danger_indicator_change_lv(
    mut commands: Commands,
    indicator_query: Query<Entity, With<FieldIndicator>>,
    next_map: Res<LevelSelection>,
) {
    if next_map.is_changed() {
        for indicator in indicator_query.iter() {
            commands.entity(indicator).despawn();
        }
    }
}

#[test]
fn test_pick_field_closest_to_hero_lv() {
    let fields: Vec<(Transform, FieldIndicator)> = vec![
        (Transform::default(), FieldIndicator::build_test(1)),
        (Transform::default(), FieldIndicator::build_test(2)),
        (Transform::default(), FieldIndicator::build_test(3)),
        (Transform::default(), FieldIndicator::build_test(4)),
        (Transform::default(), FieldIndicator::build_test(9)),
        (Transform::default(), FieldIndicator::build_test(16)),
        (Transform::default(), FieldIndicator::build_test(24)),
    ];

    let player_lv: u32 = 5;

    let mut filter = fields
        .iter()
        .filter(|(_, ind)| ind.level >= player_lv)
        .collect::<Vec<_>>();

    assert_ne!(
        filter.iter().next().unwrap().1.level,
        1,
        "Field filtering not working"
    );

    filter.sort_by(|(_, a_ind), (_, b_ind)| {
        (a_ind.level - player_lv).cmp(&(b_ind.level - player_lv))
    });

    assert_eq!(
        filter.iter().next().unwrap().1.level,
        9,
        "Field sorting not working"
    );
}
