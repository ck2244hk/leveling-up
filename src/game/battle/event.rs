use crate::model::sub::DropEquipment;

use super::*;

#[derive(Event)]
pub struct BattleEvent {
    pub is_player_victory: bool,
    pub monster_entity: Entity,
    pub battle_entity: Entity,
}

#[derive(Event)]
pub struct AttackEvent {
    pub record: CombatRecord,
}

#[derive(Event)]
pub struct NextTurnEvent {
    pub is_player_turn: bool,
    pub battle_entity: Entity,
}

#[derive(Event)]
pub struct StartBattleEvent {}

#[derive(Event)]
pub struct SpawnBattleSceneEvent {
    pub battle: Entity,
    pub monster: Entity,
}

#[derive(Event)]
pub struct SpawnDropSceneEvent {
    pub eq: DropEquipment,
}
