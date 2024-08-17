use bevy::app::AppExit;
use bevy::prelude::*;

use crate::controller::Screen;
use crate::main_menu::components::*;
use crate::main_menu::styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR};
use crate::{FirstTime, OverlayShopState, SimulationState};

pub fn interact_with_play_button(
    mut commands: Commands,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    first_time_query: Query<Entity, (With<FirstTime>, Without<PlayButton>)>,
    mut next_app_state: ResMut<NextState<Screen>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                next_app_state.set(Screen::Playing);
                next_simulation_state.set(SimulationState::Pause);
                println!("Set State to Game");

                if let Ok(entity) = first_time_query.get_single() {
                    commands.entity(entity).despawn();
                }
            }
            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

pub fn interact_with_quit_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.send(AppExit);
            }
            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

pub fn interact_with_shop_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ShopButton>),
    >,
    mut next_shop_state: ResMut<NextState<OverlayShopState>>,
) {
    for (interaction, mut background_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();

                next_shop_state.set(OverlayShopState::Opened);
                info!("Open Shop");
            }

            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
            _ => (),
        }
    }
}
