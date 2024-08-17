use crate::game::TurnFlag;

use super::*;

pub fn handle_click_in_combat(
    mut text_input_query: Query<
        (
            Entity,
            &Terminal,
            &mut TerminalQueue,
            &mut TerminalQueueTimer,
            &TerminalTextStyle,
        ),
        With<Terminal>,
    >,
    mut inner_text: InnerText,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    touches: Res<Touches>,
    mut battle_query: Query<&mut TurnFlag, Without<TerminalQueue>>,
) {
    for (input_entity, terminal, mut queue, _queue_timer, style) in text_input_query.iter_mut() {
        let Some(mut text) = inner_text.get_mut(input_entity) else {
            continue;
        };

        let Some(first_item) = queue.0.front_mut() else {
            continue;
        };

        let Ok(turn_flag) = battle_query.get_single_mut() else {
            warn!("No battle found for entity ",);
            continue;
        };

        if keyboard_input.any_just_pressed([KeyCode::Space, KeyCode::ArrowLeft])
            || touches.any_just_pressed()
        {
            if !first_item.is_empty() && !turn_flag.is_all_ready() {
                clear_queue_container(&mut text, first_item, style);
            }
        }
    }
}

pub fn next_sentance_combat(
    mut text_input_query: Query<
        (
            Entity,
            &Terminal,
            &mut TerminalQueue,
            &mut TerminalQueueTimer,
            &TerminalTextStyle,
        ),
        With<Terminal>,
    >,
    mut inner_text: InnerText,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    touches: Res<Touches>,
    mut battle_query: Query<&mut TurnFlag, Without<TerminalQueue>>,
) {
    for (input_entity, terminal, mut queue, _queue_timer, style) in text_input_query.iter_mut() {
        let Some(mut text) = inner_text.get_mut(input_entity) else {
            continue;
        };

        let Some(first_item) = queue.0.front() else {
            continue;
        };

        let Ok(turn_flag) = battle_query.get_single_mut() else {
            warn!("No battle found for entity ",);
            continue;
        };

        if keyboard_input.any_just_pressed([KeyCode::Space, KeyCode::ArrowLeft])
            || touches.any_just_pressed()
        {
            info!(
                "first item: {:?}, turn_flag: {}",
                first_item.is_empty(),
                turn_flag.is_all_ready()
            );
            if first_item.is_empty() && turn_flag.is_all_ready() {
                queue.0.pop_front();
                let _ = text.set(Box::new(get_empty_terminal(style)));

                info!("Terminal Combat Next Turn");
            }
        }
    }
}

// check when the terminal is done printing
pub fn set_ready_for_next_turn(
    mut text_input_query: Query<(&Terminal, &mut TerminalQueue), Changed<TerminalQueue>>,
    mut battle_query: Query<&mut TurnFlag, Without<TerminalQueue>>,
) {
    for (terminal, mut queue) in text_input_query.iter_mut() {
        let Some(first_item) = queue.0.front_mut() else {
            continue;
        };
        let Ok(mut turn_flag) = battle_query.get_single_mut() else {
            continue;
        };

        if first_item.is_empty() {
            // info!("Next Turn Ready");
            turn_flag.is_terminal_ready = true;
        } else {
            turn_flag.is_terminal_ready = false;
        }
    }
}
