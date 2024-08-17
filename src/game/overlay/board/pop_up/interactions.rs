use bevy::prelude::*;

use crate::{
    state::{OverlayScoreBoardState, OverlayStatusBoardState, Screen},
    game::board::SpawnScoreBoardEvent,
};

use super::{CancelRestartButton, ConfirmRestartButton};

pub fn interact_with_confirm_restart_button(
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<ConfirmRestartButton>)>,
    mut app_state: ResMut<NextState<Screen>>,
    mut status_board_state: ResMut<NextState<OverlayStatusBoardState>>,
    mut next_score_board_state: ResMut<NextState<OverlayScoreBoardState>>,
    mut spawn_score_board: EventWriter<SpawnScoreBoardEvent>,
) {
    for interaction in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                app_state.set(Screen::GameOver);
                spawn_score_board.send(SpawnScoreBoardEvent(vec![]));
                status_board_state.set(OverlayStatusBoardState::Closed);
                next_score_board_state.set(OverlayScoreBoardState::Opened);
            }
            _ => (),
        }
    }
}

pub fn interact_with_cancel_buy_button(
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<CancelRestartButton>)>,
    mut status_board_state: ResMut<NextState<OverlayStatusBoardState>>,
) {
    for interaction in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                status_board_state.set(OverlayStatusBoardState::Opened);
                info!("Close Window");
            }
            _ => (),
        }
    }
}
