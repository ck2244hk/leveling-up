use super::*;

use component::AutoTerminalText;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn next_sentance_in_auto(
    mut text_input_query: Query<
        (
            Entity,
            &AutoTerminalText,
            &mut TerminalQueue,
            &mut TerminalQueueTimer,
            &TerminalTextStyle,
            &mut TerminalSentancePauseTimer,
        ),
        With<Terminal>,
    >,
    mut write_terminal_event: EventWriter<WriteTerminalEvent>,
) {
    for (input_entity, auto, queue, _queue_timer, style, mut pause_timer) in
        text_input_query.iter_mut()
    {
        if queue.0.is_empty() {
            let mut auto_text = auto.text.clone();
            auto_text.shuffle(&mut thread_rng());
            for sentance in auto_text.iter() {
                write_terminal_event.send(WriteTerminalEvent(sentance.to_string()));
            }
        }

        if queue.0.front().is_some_and(|front| front.is_empty()) {
            pause_timer.0.unpause();
        }

        // let Ok((auto_text)) = auto_query.get_single() else {
        //     warn!("No auto text found for entity ",);
        //     continue;
        // };
    }
}

// check when the terminal is done printing
pub fn check_auto_text(
    mut inner_text: InnerText,
    mut text_input_query: Query<
        (
            Entity,
            &AutoTerminalText,
            &mut TerminalQueue,
            &mut TerminalQueueTimer,
            &TerminalTextStyle,
            &mut TerminalSentancePauseTimer,
        ),
        Changed<AutoTerminalText>,
    >,
    mut write_terminal_event: EventWriter<WriteTerminalEvent>,
) {
    for (input_entity, auto_text, mut queue, _queue_timer, style, mut pause_timer) in
        text_input_query.iter_mut()
    {
        let Some(mut text) = inner_text.get_mut(input_entity) else {
            warn!("No inner text found");
            continue;
        };

        let Some(first_item) = queue.0.front_mut() else {
            warn!("Completely Empty queue");
            continue;
        };

        clear_queue_container(&mut text, first_item, style);

        queue.0.clear();
        let Some(mut text) = inner_text.get_mut(input_entity) else {
            warn!("No inner text found, when try to clear");
            continue;
        };
        let _ = text.set(Box::new(get_empty_terminal(style)));

        let text = auto_text.text.clone();
        pause_timer.0.reset();
        pause_timer.0.pause();

        for sentance in text.iter() {
            write_terminal_event.send(WriteTerminalEvent(sentance.to_string()));
        }
    }
}

pub fn handle_sentance_pause_timer(
    mut text_input_query: Query<
        (
            Entity,
            &AutoTerminalText,
            &mut TerminalQueue,
            &mut TerminalQueueTimer,
            &TerminalTextStyle,
            &mut TerminalSentancePauseTimer,
        ),
        With<AutoTerminalText>,
    >,
    mut inner_text: InnerText,
    time: Res<Time>,
) {
    for (input_entity, auto, mut queue, _queue_timer, style, mut pause_timer) in
        text_input_query.iter_mut()
    {
        // info!(
        //     "Timer ticking, Timer pause is {:?}, remaining: {:?}",
        //     pause_timer.0.paused(),
        //     pause_timer.0.remaining()
        // );
        pause_timer.0.tick(time.delta());

        // info!(
        //     "Timer ticking, Timer pause is {:?}, remaining: {:?}",
        //     pause_timer.0.paused(),
        //     pause_timer.0.remaining()
        // );

        if pause_timer.0.just_finished() {
            // info!("Timer finished");
            // this means starting next line
            queue.0.pop_front();
            pause_timer.0.reset();
            pause_timer.0.pause();
            let Some(mut text) = inner_text.get_mut(input_entity) else {
                warn!("No inner text found, when try to clear");
                continue;
            };
            let _ = text.set(Box::new(get_empty_terminal(style)));

            // info!("Terminal Talk Next Line");
        }
    }
}

// Situation 1: Queue is empty -> need to generate new content (ok)
// Situation 2: Queue first item is empty -> ready to be pop for next line ()
// Situation 3: ready to be pop for next line -> set timer (ok)
// Situation 4: when is done -> pop for next line (ok)
// Stituation 5: When changed of Auto text is detected -> clear queue and set timer to finished (ok?)
