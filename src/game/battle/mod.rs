use bevy::prelude::*;
use rand::random;

mod after_battle_sys;
mod combat_system;
pub mod component;
mod damage;
pub mod event;
mod talk_system;

use after_battle_sys::*;
use combat_system::*;
pub use component::*;
use damage::*;
use event::*;
use talk_system::*;

use super::{
    battle_scene::component::LvUpQueue,
    character::component::{Hero, HeroClass, MonsterType, Turns},
    combat_system::next_sentance_combat,
    monster::Monster,
    Player, TerminalQueue, WriteTerminalEvent,
};
use crate::{
    game::EquipmentBelt,
    state::{OverlayCombatState, TerminalState},
};

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BattleEvent>()
            .add_event::<StartBattleEvent>()
            .add_event::<NextTurnEvent>()
            .add_event::<AttackEvent>()
            .add_event::<SpawnBattleSceneEvent>()
            .add_event::<SpawnDropSceneEvent>()
            .add_systems(
                Update,
                (loop_talking, drop_count_down)
                    .run_if(in_state(TerminalState::Talking))
                    .run_if(in_state(OverlayCombatState::Opened)),
            )
            .add_systems(OnEnter(TerminalState::Combating), start_combat_turn)
            .add_systems(
                Update,
                (
                    battle_loop,
                    send_attack_event_terminal,
                    send_next_turn,
                    handle_after_battle.after(next_sentance_combat),
                )
                    .run_if(in_state(TerminalState::Combating))
                    .run_if(in_state(OverlayCombatState::Opened)),
            )
            .add_systems(Update, spawn_battle)
            .add_systems(OnEnter(OverlayCombatState::Closed), despawn_battle)
            .register_type::<Combat>()
            .register_type::<TurnFlag>()
            .register_type::<TalkFlag>()
            .register_type::<TrashTalk>();
    }
}

pub fn despawn_battle(battle_query: Query<Entity, With<Combat>>, mut commands: Commands) {
    for entity in battle_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
