use bevy::app::AppExit;

use crate::game::character::component::{Hero, Turns};

use super::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (transition_to_game_state, transition_to_main_menu_state),
    )
    .add_systems(
        OnExit(OverlayCombatState::Opened),
        trigger_game_over_event.run_if(in_state(Screen::Playing)),
    )
    .add_systems(Startup, spawn_first_time_playing);
}

pub fn spawn_first_time_playing(mut commands: Commands) {
    commands.spawn(FirstTime);
}

pub fn transition_to_game_state(
    mut next_app_state: ResMut<NextState<Screen>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<Screen>>,
    mut next_dropping_state: ResMut<NextState<OverlayDroppingPickingState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) && *app_state.get() != Screen::Playing {
        next_app_state.set(Screen::GameOver);
        next_dropping_state.set(OverlayDroppingPickingState::Opened);
        println!("Entered App State Game Mode");
    }
}

pub fn transition_to_main_menu_state(
    mut next_app_state: ResMut<NextState<Screen>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<Screen>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) && *app_state.get() != Screen::Title {
        next_app_state.set(Screen::Title);
        println!("Entered App State Main Menu");
    }
}

pub fn trigger_game_over_event(
    player_query: Query<&Turns, With<Hero>>,
    mut next_dropping_state: ResMut<NextState<OverlayDroppingPickingState>>,
    mut next_app_state: ResMut<NextState<Screen>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if let Ok(player_turns) = player_query.get_single() {
        if player_turns.get() == 0 {
            // game
            next_dropping_state.set(OverlayDroppingPickingState::Opened);
            next_app_state.set(Screen::GameOver);
            next_simulation_state.set(SimulationState::Pause);
            println!("Game over")
        }
    }
}
