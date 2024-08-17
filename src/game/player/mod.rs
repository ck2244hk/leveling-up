use bevy::prelude::*;
use bevy_rapier2d::dynamics::{Ccd, Damping, Velocity};
use bevy_rapier2d::geometry::Restitution;
use bevy_rapier2d::{
    dynamics::{CoefficientCombineRule, LockedAxes, RigidBody},
    geometry::{Collider, Friction},
};

use super::preload::Location;
use super::{
    battle::event::BattleEvent, character::component::Level, monster::Monster, ChangeMapEvent,
};

use crate::game::field::EncounterTimer;
use crate::model::armor::ArmorData;
use crate::model::player::PlayerData;
use crate::model::sub::DropEquipment;
use crate::model::weapon::WeaponData;
use crate::model::EquipmentData;
use crate::preload::data::PlayerPreloadHandler;
use crate::state::{OverlayCombatState, Screen, SimulationState};

pub mod component;
mod movement;

pub use component::*;
use movement::*;

pub const PLAYER_Z_INDEX: f32 = 9.;
pub const PLAYER_START_POSITION: Vec3 = Vec3::new(440., 370., PLAYER_Z_INDEX);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spanw_player)
            .add_systems(
                Update,
                (update_idle,)
                    .run_if(in_state(SimulationState::Running))
                    .run_if(in_state(Screen::Playing)),
            )
            .add_systems(
                Update,
                (
                    // touch_movement_input.in_set(UserInput),
                    // update_joystick,
                    movement_input,
                    player_fit_inside_current_level,
                )
                    .run_if(in_state(SimulationState::Running))
                    .run_if(in_state(Screen::Playing)),
            )
            .add_systems(OnEnter(Screen::Playing), init_player_position_w_map)
            .add_systems(
                Update,
                (handle_battle, spawn_player_position, init_player_w_save),
            )
            .add_systems(OnExit(Screen::Playing), despawn_joystick)
            .add_systems(OnExit(Screen::GameOver), reset_player)
            .add_systems(OnEnter(OverlayCombatState::Closed), reset_encounter)
            .register_type::<CurrentMovement>()
            .register_type::<Money>()
            .register_type::<Encounter>()
            .register_type::<Storage>()
            .register_type::<PlayerEnv>()
            .register_type::<Velocity>();
    }
}

pub fn init_player_w_save(
    player_handle: Option<Res<PlayerPreloadHandler>>,
    player_data: Res<Assets<PlayerData>>,
    mut player_query: Query<(&mut EquipmentBelt, &mut Storage), With<Player>>,
    weapon_assets: Res<Assets<WeaponData>>,
) {
    if let Some(handle) = player_handle {
        if handle.is_added() {
            let Ok((mut player_equipment_belt, mut player_storage)) = player_query.get_single_mut()
            else {
                info!("no player when pulling player data");
                return;
            };

            let Some(data) = player_data.get(handle.id()) else {
                info!("no player data when pulling player data");
                return;
            };

            player_equipment_belt.equip_weapon(&weapon_assets, &data);

            *player_storage = Storage::new(data);
        }
    }
}

pub fn spanw_player(mut commands: Commands, weapon_assets: Res<Assets<WeaponData>>) {
    let rotation_constraints = LockedAxes::ROTATION_LOCKED;
    // let player_data = player_query
    //     .get(player_id.0.id())
    //     .expect("Player Save not loaded successfully");

    let player = commands
        .spawn((
            Name::new("Player"),
            Player {},
            TransformBundle {
                local: Transform::from_translation(PLAYER_START_POSITION),
                ..default()
            },
            CurrentMovement::default(),
            FacingSide::default(),
            VisibilityBundle::default(),
            Ccd::enabled(),
            Collider::capsule_y(8., 8.),
            Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            Friction {
                coefficient: 0.,
                combine_rule: CoefficientCombineRule::Min,
            },
            RigidBody::Dynamic,
            Encounter::default(),
            rotation_constraints,
            EncounterTimer::default(),
        ))
        .id();

    let belt = EquipmentBelt::new();

    let storage = Storage::empty();

    commands.entity(player).insert((
        belt,
        storage,
        PlayerEnv::default(),
        Velocity::default(),
        Damping {
            linear_damping: 20.,
            angular_damping: 0.,
        },
    ));

    println!("Spawned Player")
}

pub fn handle_battle(
    player_query: Query<&mut Money, With<Player>>,
    monster_query: Query<&mut Level, With<Monster>>,
    battle_event_reader: EventReader<BattleEvent>,
) {
    // for ev in battle_event_reader.read() {
    //     if let Ok(mut money) = player_query.get_single_mut() {
    //         if let Ok(monster_lv) = monster_query.get(ev.monster_entity) {
    //             if ev.is_player_victory {
    //                 money.add((monster_lv.get()).pow(2));
    //             }
    //         }
    //     }
    // }
}

pub fn spawn_player_position(
    mut player_query: Query<&mut Transform, With<Player>>,
    location_query: Query<(&Transform, &Location), (With<Location>, Without<Player>)>,
    mut event_reader: EventReader<ChangeMapEvent>,
) {
    for event in event_reader.read() {
        info!("Transporting");
        if let Some((destination, location)) = location_query
            .iter()
            .find(|(_, location)| location.equal(&event.location))
        {
            let Ok(mut player_transform) = player_query.get_single_mut() else {
                warn!("Player is not found during map switching");
                continue;
            };
            info!(
                "New Location {:?}, is located at : {}",
                location, destination.translation
            );
            player_transform.translation = destination.translation;
            player_transform.translation.z = PLAYER_Z_INDEX;
        }
    }
}

pub fn init_player_position_w_map(mut player_query: Query<&mut Transform, With<Player>>) {
    let mut player_transform = player_query.single_mut();

    player_transform.translation = PLAYER_START_POSITION;
}

pub fn reset_player(mut player_query: Query<(&mut Transform, &mut Money), With<Player>>) {
    for (mut transform, mut money) in player_query.iter_mut() {
        transform.translation = PLAYER_START_POSITION;
        money.0 /= 10;
    }
}

pub fn reset_encounter(mut zone_query: Query<&mut Encounter, With<Player>>) {
    for mut encounter in zone_query.iter_mut() {
        encounter.reset()
    }
}
