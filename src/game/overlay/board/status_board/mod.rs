use bevy::prelude::*;

pub mod interactions;
mod layout;
mod update;

use interactions::*;
use layout::*;
use update::*;

use crate::state::{OverlayStatusBoardState, SimulationState};

pub struct StatusBoardPlugin;

impl Plugin for StatusBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(OverlayStatusBoardState::Closed), spawn_status_board)
            .add_systems(
                OnEnter(OverlayStatusBoardState::Closed),
                (despawn_status_board, return_state_points),
            )
            .add_systems(
                Update,
                (
                    interact_with_agi_half_button,
                    interact_with_str_half_button,
                    interact_with_def_half_button,
                    interact_with_agi_all_button,
                    interact_with_str_all_button,
                    interact_with_def_all_button,
                    interact_with_confirm_button,
                    interact_with_reset_button,
                    interact_with_restart_button,
                )
                    .run_if(in_state(SimulationState::Pause))
                    .run_if(in_state(OverlayStatusBoardState::Opened)),
            )
            .add_systems(
                Update,
                (
                    update_str_text,
                    update_str_buffer_text,
                    update_def_text,
                    update_agi_text,
                    update_state_point_text,
                    update_money_text,
                    click_to_close,
                    update_experience_gauge_ui,
                    update_bag_grid,
                    handle_selecting_grid,
                    interact_with_grid_button,
                )
                    .run_if(in_state(SimulationState::Pause))
                    .run_if(in_state(OverlayStatusBoardState::Opened)),
            );
    }
}
