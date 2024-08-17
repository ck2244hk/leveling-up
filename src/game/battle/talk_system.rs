use crate::{state::SimulationState, game::character::component::BaseStates};

use super::*;
// Battle was spawned by the creation of fightable monster
// Actions -> 1. set simulation to pause and set combat state to opened.
//           2. by doing so it spawns the battle scene and the terminal and disable the player
//          3. send an event for to the terminal for initialization.
pub fn spawn_battle(
    mut commands: Commands,
    hero_query: Query<&BaseStates, With<Hero>>,
    monster_query: Query<(&BaseStates, Entity), (Added<Monster>, Without<Hero>)>,
    mut spawn_battle_scene_event: EventWriter<SpawnBattleSceneEvent>,
    mut next_combat_state: ResMut<NextState<OverlayCombatState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
    mut next_combat_stage: ResMut<NextState<TerminalState>>,
    battle_query: Query<Entity, With<Combat>>,
) {
    if let Ok((monster_state, monster_entity)) = monster_query.get_single() {
        let hero_state = hero_query.get_single().expect("No Hero Found");

        next_combat_state.set(OverlayCombatState::Opened);
        next_simulation_state.set(SimulationState::Pause);
        next_combat_stage.set(TerminalState::Talking);

        for prev_battle in battle_query.iter() {
            commands.entity(prev_battle).despawn_recursive();
        }

        info!("Battle Started");
        let id = commands
            .spawn((
                Name::new("Battle"),
                Combat {
                    monster_entity,
                    records: Vec::new(),
                    monster_hp_remain: monster_state.hp(),
                    player_hp_remain: hero_state.hp(),
                },
                TalkFlag::default(),
                TurnFlag::default(),
                TrashTalk::demo(),
            ))
            .id();

        spawn_battle_scene_event.send(SpawnBattleSceneEvent {
            battle: id,
            monster: monster_entity,
        });
    }
}

pub fn loop_talking(
    mut talk_query: Query<(&mut TalkFlag, &mut TrashTalk, Option<&ExitFlag>)>,
    mut write_terminal_event: EventWriter<WriteTerminalEvent>,
    mut next_combat_stage: ResMut<NextState<TerminalState>>,
    text_input_query: Query<&TerminalQueue>,

    mut next_simulation_state: ResMut<NextState<SimulationState>>,
    mut next_combat_state: ResMut<NextState<OverlayCombatState>>,
) {
    for (mut flag, mut talk, exit) in talk_query.iter_mut() {
        let Ok(queue) = text_input_query.get_single() else {
            continue;
        };

        if flag.is_terminal_ready {
            match talk.pop() {
                Some(sentance) => {
                    info!("Sending {:?} to the terminal", sentance);
                    write_terminal_event.send(WriteTerminalEvent::new(sentance));
                    flag.reset();
                }
                None => {
                    flag.is_ended = true;
                }
            };
        }

        if flag.is_all_finished() && queue.is_empty() {
            match exit {
                Some(_) => {
                    next_simulation_state.set(SimulationState::Running);
                    next_combat_state.set(OverlayCombatState::Closed);
                    info!("Exited Battle")
                }
                None => {
                    info!("Talking Stage Done");
                    next_combat_stage.set(TerminalState::Combating);
                }
            };
        }
    }
}
