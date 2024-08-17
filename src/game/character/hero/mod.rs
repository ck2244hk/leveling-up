use bevy::prelude::*;

mod sprite;
mod status;

use sprite::*;
use status::*;

use crate::game::Player;
use crate::state::{Screen, SimulationState};

use super::component::*;

#[derive(Event)]
pub struct SpawnHeroEvent();

pub struct HeroPlugin;

impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnHeroEvent>()
            .add_systems(OnEnter(Screen::Playing), (spawn_hero, spawn_decending_hero))
            .add_systems(Update, (update_player_level, spawn_hero_sprite))
            .add_systems(
                Update,
                (
                    update_sprite_anime,
                    update_sprite_flip,
                    spawn_scale_w_sprite,
                )
                    .run_if(in_state(SimulationState::Running))
                    .run_if(in_state(Screen::Playing)),
            )
            .add_systems(Update, (spawn_hero_sprite_battle, despawn_decending))
            .add_systems(OnExit(Screen::GameOver), despawn_hero)
            .register_type::<HeroClass>();
    }
}

pub fn spawn_hero(mut commands: Commands, mut player_query: Query<Entity, With<Player>>) {
    // for ev in spawn_event.read() {
    let player_entity = player_query.get_single_mut().expect("No Player is spawned");
    let hero_id = commands
        .spawn((
            Name::new("Hero"),
            Hero {},
            HeroClass::Warrior,
            Turns::default(),
            BaseStates::new_hero(),
            Bag::default(),
        ))
        .id();

    commands.entity(player_entity).add_child(hero_id);
    // }
}

fn despawn_hero(hero_query: Query<Entity, With<Player>>, mut commands: Commands) {
    for entity in hero_query.iter() {
        commands.entity(entity).despawn_descendants();
    }
}
