use bevy::prelude::*;
use interactions::*;
use layout::*;
use update::*;

pub mod components;
pub mod interactions;
pub mod layout;
pub mod styles;
mod update;

use crate::state::{OverlayCombatState, Screen};

#[derive(Event)]
pub struct SpawnHeaderEvent {}

pub struct HeaderPlugin;

impl Plugin for HeaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnHeaderEvent>()
            .add_systems(OnExit(Screen::Playing), despawn_header)
            .add_systems(OnEnter(Screen::Playing), spawn_header)
            .add_systems(
                Update,
                (
                    update_status_board_button,
                    update_level_display,
                    update_turns_display,
                    update_encounter_gauge,
                    update_in_danger_display,
                )
                    .run_if(in_state(Screen::Playing)),
            )
            .add_systems(
                Update,
                (interact_with_status_button_button)
                    // .run_if(in_state(SimulationState::Running))
                    .run_if(in_state(Screen::Playing))
                    .run_if(in_state(OverlayCombatState::Closed)),
            );
    }
}
