use crate::game::character::component::BaseStates;

use super::*;

pub fn start_combat_turn(
    mut next_turn_event_writer: EventWriter<NextTurnEvent>,
    mut battle_query: Query<(&mut TurnFlag, Entity)>,
) {
    for (mut turn_flag, entity) in battle_query.iter_mut() {
        next_turn_event_writer.send(NextTurnEvent {
            is_player_turn: true,
            battle_entity: entity,
        });

        turn_flag.reset();
    }
}

pub fn send_next_turn(
    mut battle_query: Query<(&mut Combat, &mut TurnFlag, Entity)>,
    mut next_turn_event_writer: EventWriter<NextTurnEvent>,
    mut battle_event_writer: EventWriter<BattleEvent>,

    keyboard_input: Res<ButtonInput<KeyCode>>,
    touches: Res<Touches>,
) {
    for (mut battle, turn_flag, entity) in battle_query.iter_mut() {
        if turn_flag.is_all_ready() {
            if keyboard_input.any_just_pressed([KeyCode::Space, KeyCode::ArrowLeft])
                || touches.any_just_pressed()
            {
                info!("Applying Last Turn Damage");
                battle.push_update(turn_flag.clone().record.unwrap());

                match (
                    battle.monster_hp_remain <= 0.0,
                    battle.player_hp_remain <= 0.0,
                ) {
                    (true, true) | (false, true) => {
                        // Lost
                        info!("Monster Won");
                        battle_event_writer.send(BattleEvent {
                            is_player_victory: false,
                            monster_entity: battle.monster_entity,
                            battle_entity: entity,
                        });
                        // write_terminal_event
                        //     .send(WriteTerminalEvent::new(format!("You are fainted\n")));
                        // commands.spawn(BattleEndingTimer::lost());
                    }
                    (true, false) => {
                        info!("Player Won");
                        battle_event_writer.send(BattleEvent {
                            is_player_victory: true,
                            monster_entity: battle.monster_entity,
                            battle_entity: entity,
                        });
                        // write_terminal_event.send(WriteTerminalEvent::new(format!(
                        //     "You defeated the monster\n"
                        // )));
                        // commands.spawn(BattleEndingTimer::victory());
                    }
                    (false, false) => {
                        info!("Sending Next Turn Signal");
                        next_turn_event_writer.send(NextTurnEvent {
                            is_player_turn: !battle
                                .records
                                .last()
                                .is_some_and(|record| record.is_player_turn),
                            battle_entity: entity,
                        });
                    }
                }
            }
        }
    }
}

#[allow(clippy::complexity)]
pub fn battle_loop(
    mut battle_query: Query<(&Combat, &mut TurnFlag)>,
    equipment_query: Query<&EquipmentBelt, With<Player>>,
    mut hero_query: Query<(&BaseStates, &HeroClass, Entity, &Name), With<Hero>>,
    mut monster_query: Query<
        (&BaseStates, &Name, &MonsterType, Entity),
        (With<Monster>, Without<Hero>),
    >,
    mut attack_event_writer: EventWriter<AttackEvent>,
    mut next_turn_event_reader: EventReader<NextTurnEvent>,
) {
    for ev in next_turn_event_reader.read() {
        let Ok((battle, mut turn_flag)) = battle_query.get_mut(ev.battle_entity) else {
            warn!("No battle found");
            continue;
        };

        let Ok((player_base, hero_class, player_entity, player_name)) = hero_query.get_single_mut()
        else {
            warn!("No hero found");
            continue;
        };

        let Ok(player_belt) = equipment_query.get_single() else {
            warn!("No equipment belt found for player");
            continue;
        };

        let Ok((monster_base, monster_name, _monster_type, monster_entity)) =
            monster_query.get_mut(battle.monster_entity)
        else {
            warn!(
                "No monster entity found for battle: {:?}",
                battle.monster_entity,
            );
            continue;
        };

        if ev.is_player_turn {
            info!("Player Turn");
        } else {
            info!("Monster Turn");
        }

        // battle until either one die
        if battle.player_hp_remain > 0. && battle.monster_hp_remain > 0. {
            let damage_out = if ev.is_player_turn {
                // Player attack
                let mut damage_out = DamageBuilder::build(player_base, monster_base);
                damage_out
                    .crit_hit(Some(hero_class))
                    .attacker_class_scaling()
                    .defense()
                    .weapon(&player_belt)
                    .innate_damage()
                    .hero_additional_damage();

                // battle.monster_hp_remain -= damage_out.get_damage_f32();

                damage_out
            } else {
                // Monster attack
                let mut damage_out = DamageBuilder::build(monster_base, player_base);
                damage_out
                    .crit_hit(None)
                    .defense()
                    .armor(&player_belt)
                    .innate_damage();

                // battle.player_hp_remain -= damage_out.get_damage_f32();

                damage_out
            };

            let record = CombatRecord {
                is_player_turn: ev.is_player_turn,
                player: player_entity,
                monster: monster_entity,
                player_name: player_name.clone(),
                monster_name: monster_name.clone(),
                damage_out: damage_out.get_damage(),
            };

            turn_flag.record = Some(record.clone());

            attack_event_writer.send(AttackEvent {
                record: record.clone(),
            });

            info!("{}", record.to_string());
        }
    }
}

// pub fn detect_battle_end(
//     commands: Commands,
//     battle_query: Query<(&Combat, Entity), Changed<Combat>>,
//     battle_event_writer: EventWriter<BattleEvent>,
//     next_combat_stage_state: ResMut<NextState<TerminalState>>,
// ) {
//     for (battle, entity) in battle_query.iter() {
//         // Battle Ended

//         // Player win
//     }
// }

pub fn send_attack_event_terminal(
    mut attack_event: EventReader<AttackEvent>,
    mut write_terminal_event: EventWriter<WriteTerminalEvent>,
) {
    for ev in attack_event.read() {
        write_terminal_event.send(WriteTerminalEvent::new(ev.record.to_string()));
    }
}
