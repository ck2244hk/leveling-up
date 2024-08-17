use bevy::prelude::*;

use crate::state::{OverlayStatusBoardState, SimulationState};
use crate::game::overlay::header::components::StatusBoardButton;
use crate::game::overlay::header::styles::{
    HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR,
};

pub fn interact_with_status_button_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<StatusBoardButton>),
    >,
    mut next_status_board_state: ResMut<NextState<OverlayStatusBoardState>>,
    status_board_state: Res<State<OverlayStatusBoardState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    for (interaction, mut background_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                if *status_board_state.get() == OverlayStatusBoardState::Opened {
                    next_status_board_state.set(OverlayStatusBoardState::Closed);
                    next_simulation_state.set(SimulationState::Running);
                    println!("Closed Status Board");
                } else if *status_board_state.get() == OverlayStatusBoardState::Closed {
                    next_status_board_state.set(OverlayStatusBoardState::Opened);
                    next_simulation_state.set(SimulationState::Pause);
                    println!("Open Status Board");
                }
            }

            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}
