use crate::game::TalkFlag;

use super::*;

// pub fn clear_queue_on_enter(
//     mut text_input_query: Query<
//         (Entity, &mut TerminalQueue, &TerminalTextStyle),
//         With<Terminal>,
//     >,
//     mut inner_text: InnerText,
// ) {
//     for (input_entity, mut queue, style) in text_input_query.iter_mut() {
//         let Some(mut text) = inner_text.get_mut(input_entity) else {
//             continue;
//         };

//         queue.0.pop_front();
//         let _ = text.set(Box::new(get_empty_terminal(style)));
//     }
// }

pub fn handle_click_in_talking(
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
    mut battle_query: Query<&TalkFlag, Without<TerminalQueue>>,

    keyboard_input: Res<ButtonInput<KeyCode>>,
    touches: Res<Touches>,
) {
    if keyboard_input.any_just_pressed([KeyCode::Space, KeyCode::ArrowLeft])
        || touches.any_just_pressed()
    {
        for (input_entity, terminal, mut queue, _queue_timer, style) in text_input_query.iter_mut()
        {
            let Some(mut text) = inner_text.get_mut(input_entity) else {
                continue;
            };

            let Some(first_item) = queue.0.front_mut() else {
                continue;
            };

            let Ok(talk_flag) = battle_query.get_single_mut() else {
                warn!("No battle found for entity ",);
                continue;
            };

            if !first_item.is_empty() && !talk_flag.is_terminal_ready {
                clear_queue_container(&mut text, first_item, style);
            }
        }
    }
}

pub fn next_sentance_in_talking(
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
    mut battle_query: Query<&TalkFlag, Without<TerminalQueue>>,
) {
    if keyboard_input.any_just_pressed([KeyCode::Space, KeyCode::ArrowLeft])
        || touches.any_just_pressed()
    {
        for (input_entity, terminal, mut queue, _queue_timer, style) in text_input_query.iter_mut()
        {
            let Some(mut text) = inner_text.get_mut(input_entity) else {
                continue;
            };

            let Some(first_item) = queue.0.front() else {
                continue;
            };

            let Ok(talk_flag) = battle_query.get_single_mut() else {
                warn!("No battle found for entity ",);
                continue;
            };

            if first_item.is_empty() && talk_flag.is_terminal_ready {
                info!("Queue pop front");
                queue.0.pop_front();
                let _ = text.set(Box::new(get_empty_terminal(style)));

                info!("Terminal Talk Next Turn");
            }
        }
    }
}

// check when the terminal is done printing
pub fn check_talk_state(
    mut text_input_query: Query<&TerminalQueue, Changed<TerminalQueue>>,
    mut battle_query: Query<&mut TalkFlag, Without<TerminalQueue>>,
) {
    for queue in text_input_query.iter_mut() {
        let Some(first_item) = queue.0.front() else {
            continue;
        };

        let Ok(mut talk_flag) = battle_query.get_single_mut() else {
            continue;
        };

        if first_item.is_empty() {
            // info!("Next Sentance Ready");
            talk_flag.is_terminal_ready = true;
        } else {
            // info!("Next Sentance Not Ready");
            talk_flag.is_terminal_ready = false;
        }
    }
}
