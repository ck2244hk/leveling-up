use crate::{
    game::character::component::{Bag, BaseStates},
    model::{
        armor::ArmorData,
        helmet::HelmetData,
        monster::MonsterData,
        shoes::ShoesData,
        sub::{DropEquipment, Element, Tier},
        weapon::WeaponData,
        EquipmentData,
    },
    state::TerminalState,
};
use rand::prelude::SliceRandom;

use super::*;

// pub fn exit_battle(
//     touches: Res<Touches>,
//     keyboard: Res<ButtonInput<KeyCode>>,
//     boss_query: Query<&Boss>,
//     terminal_query: Query<&TerminalQueue, With<Terminal>>,
//     mut next_simulation_state: ResMut<NextState<SimulationState>>,
//     mut next_combat_state: ResMut<NextState<OverlayCombatState>>,
//     mut next_ending_credit_state: ResMut<NextState<OverlayEndingCreditState>>,
//     mut next_turn_event_reader: EventReader<NextTurnEvent>,
// ) {
//     // the purpose of this is for immediate exist after the press to exist is printed.
//     // to prevent an extra click after the queue the clear
//     let is_last_sentance_pressed = next_turn_event_reader.read().any(|_| true);

//     if let Ok(terminal_q) = terminal_query.get_single() {
//         // empty boss query is just for temp demo purpose.
//         // Eventually, it should have one boss or something to trigger the ending credit scene
//         // if it even has one
//         if boss_query.is_empty() && terminal_q.is_empty() {
//             if touches.any_just_pressed()
//                 || keyboard.any_just_pressed([KeyCode::Space])
//                 || is_last_sentance_pressed
//             {
//                 next_ending_credit_state.set(OverlayEndingCreditState::Opened);
//                 next_combat_state.set(OverlayCombatState::Closed);
//                 info!("Exited Battle ")
//             }
//         } else if terminal_q.is_empty() {
//             if touches.any_just_pressed()
//                 || keyboard.any_just_pressed([KeyCode::Space])
//                 || is_last_sentance_pressed
//             {
//                 next_simulation_state.set(SimulationState::Running);
//                 next_combat_state.set(OverlayCombatState::Closed);
//                 info!("Exited Battle")
//             }
//         }
//     }
// }

pub fn handle_after_battle(
    mut commands: Commands,

    mut player_query: Query<(&mut BaseStates, &mut Turns, &mut Bag), With<Hero>>,
    monster_query: Query<(&mut BaseStates, &MonsterData), (With<Monster>, Without<Hero>)>,
    mut battle_query: Query<(&mut TalkFlag, Entity, &mut TrashTalk)>,
    mut hero_lv_up_text_queue: Query<&mut LvUpQueue>,

    mut battle_event_reader: EventReader<BattleEvent>,
    mut next_combat_stage: ResMut<NextState<TerminalState>>,

    weapon_asset: Res<Assets<WeaponData>>,
    armor_asset: Res<Assets<ArmorData>>,
    helmet_asset: Res<Assets<HelmetData>>,
    shoes_asset: Res<Assets<ShoesData>>,
) {
    for ev in battle_event_reader.read() {
        let Ok(mut lv_up_queue) = hero_lv_up_text_queue.get_single_mut() else {
            warn!("level up queue not found");
            continue;
        };

        let Ok((mut player_state, mut player_turns, mut player_bag)) =
            player_query.get_single_mut()
        else {
            warn!("player not found");
            continue;
        };
        let Ok((monster_state, monster_data)) = monster_query.get(ev.monster_entity) else {
            warn!("monster not found");
            continue;
        };

        // 1. reset talking flag
        let Ok((mut talk_flag, entity, mut talk)) = battle_query.get_single_mut() else {
            warn!("combat not found");
            continue;
        };

        info!("Battle Ended");
        next_combat_stage.set(TerminalState::Talking);
        talk_flag.ready();
        commands.entity(entity).insert(ExitFlag);

        if ev.is_player_victory {
            // handle win
            // 1. Minus turn
            player_turns.minus();

            // 2. fire terminal message
            talk.push(format!("You defeated the monster\n"));
            // 3.Handle Drop Equipment
            if let Some(drop) = drop_item(
                &monster_state,
                &monster_data,
                &weapon_asset,
                &armor_asset,
                &helmet_asset,
                &shoes_asset,
            ) {
                player_bag.push(&drop);
                commands.entity(entity).insert(DropPopupFlag::new(1, &drop));

                talk.push(format!(
                    "You found a {}lv {} on the ground",
                    drop.level, drop.name
                ));
            }

            // level
            lv_up_queue.0 += player_state.exp_gain(monster_state.lv());
            talk.push(format!(
                "Gained {:.1} exp from battle\n",
                BaseStates::exp_drop_by_monster(monster_state.lv()),
            ));

            // final
            talk.push(format!("Press Anywhere to Continue"));
        } else {
            // handle lost
            player_turns.minus();
            player_turns.minus();
            player_turns.minus();
            talk.push(format!("You are fainted, 3 turns have come to pass"));
            talk.push(format!("Press Anywhere to Continue"));
        }
    }
}

pub fn drop_count_down(
    mut battle_query: Query<(&mut DropPopupFlag, &TalkFlag)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    touches: Res<Touches>,
    mut drop_event_writer: EventWriter<SpawnDropSceneEvent>,
) {
    let Ok((mut drop_count_flag, talk_flag)) = battle_query.get_single_mut() else {
        return;
    };

    if (keyboard_input.any_just_pressed([KeyCode::Space, KeyCode::ArrowLeft])
        || touches.any_just_pressed())
        && talk_flag.is_terminal_ready
    {
        drop_count_flag.count_down();
    }

    if drop_count_flag.counter == 0 {
        drop_event_writer.send(SpawnDropSceneEvent {
            eq: drop_count_flag.drop.clone(),
        });
    }
}

// Run after handle_after_battle
fn drop_item(
    monster_state: &BaseStates,
    monster_data: &MonsterData,

    weapon_asset: &Res<Assets<WeaponData>>,
    armor_asset: &Res<Assets<ArmorData>>,
    helmet_asset: &Res<Assets<HelmetData>>,
    shoes_asset: &Res<Assets<ShoesData>>,
) -> Option<DropEquipment> {
    // Drop Chance
    if random::<f32>() < 0.4 {
        let monster_tier = monster_data.tier.clone();
        let element = monster_data.element.clone();
        let mut list_possible: Vec<DropEquipment> = Vec::new();
        info!("Dropping!!!!!! Monster Tier: {:?}", monster_tier);

        weapon_possible_drop(
            &weapon_asset,
            &mut list_possible,
            monster_state.lv(),
            &element,
        );

        push_list_of_possible_drop::<ArmorData>(
            &armor_asset,
            &mut list_possible,
            &monster_tier,
            monster_state.lv(),
        );

        push_list_of_possible_drop::<HelmetData>(
            &helmet_asset,
            &mut list_possible,
            &monster_tier,
            monster_state.lv(),
        );

        push_list_of_possible_drop::<ShoesData>(
            &shoes_asset,
            &mut list_possible,
            &monster_tier,
            monster_state.lv(),
        );

        info!("Possible list of Equipment: {:?}", list_possible);

        let drop = list_possible
            .choose(&mut rand::thread_rng())
            .map(|x| x.clone());
        drop
    } else {
        None
    }
}

fn weapon_possible_drop(
    assets: &Res<Assets<WeaponData>>,
    list_possible: &mut Vec<DropEquipment>,
    monster_lv: u32,
    monster_element: &Element,
) {
    let weapon = assets
        .iter()
        .find(|(_, weapon)| weapon.element == *monster_element);

    if let Some((_, data)) = weapon {
        list_possible.push(DropEquipment {
            id: data.id(),
            level: monster_lv,
            item_type: data.item_type(),
            image: None,
            name: data.name(),
            description: data.description(),
        })
    }
}

fn push_list_of_possible_drop<T>(
    assets: &Res<Assets<T>>,
    list_possible: &mut Vec<DropEquipment>,
    monster_tier: &Tier,
    monster_lv: u32,
) where
    T: Asset + EquipmentData,
{
    info!("check");
    for (_, data) in assets.iter() {
        // info!(
        //     "Checking {:?} {} with tier {:?}",
        //     data.item_type(),
        //     data.id(),
        //     data.tier()
        // );
        if monster_tier.contain(&data.tier()) {
            info!("Contain {:?} {}", data.item_type(), data.id());

            list_possible.push(DropEquipment {
                id: data.id(),
                level: monster_lv,
                item_type: data.item_type(),
                image: None,
                name: data.name(),
                description: data.description(),
            })
        }
    }
}
