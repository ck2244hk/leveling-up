use bevy::prelude::*;
use bevy::{input::touch::TouchPhase, log::info};

use crate::game::board::styles::NORMAL_BUTTON_COLOR;
use crate::game::board::SelectGridEvent;
use crate::game::overlay::board::SpawnRestartConfirmPopupEvent;
use crate::game::overlay::header::styles::{HOVERED_BUTTON_COLOR, PRESSED_BUTTON_COLOR};
use crate::game::{
    character::component::*, overlay::board::components::*,
    overlay::header::components::StatusBoardButton,
};
use crate::state::{OverlayStatusBoardState, SimulationState};

pub fn interact_with_str_half_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<StrButton>),
    >,
    mut buffer_query: Query<&mut UpgradeBuffer>,
    mut hero_query: Query<&mut StatePoint, With<Hero>>,
) {
    for (interaction, mut background_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                let Ok(mut buffer) = buffer_query.get_single_mut() else {
                    continue;
                };
                let Ok(mut status_point) = hero_query.get_single_mut() else {
                    continue;
                };
                match status_point.half() {
                    Ok(point) => {
                        buffer.strength += point;
                    }
                    Err(err) => info!("Error: {:?}", err),
                }
            }

            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

pub fn interact_with_def_half_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<DefButton>),
    >,
    mut buffer_query: Query<&mut UpgradeBuffer>,
    mut hero_query: Query<&mut StatePoint, With<Hero>>,
) {
    for (interaction, mut background_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                let Ok(mut buffer) = buffer_query.get_single_mut() else {
                    continue;
                };
                let Ok(mut status_point) = hero_query.get_single_mut() else {
                    continue;
                };
                match status_point.half() {
                    Ok(point) => {
                        buffer.defense += point;
                    }
                    Err(err) => info!("Error: {:?}", err),
                }
            }

            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

pub fn interact_with_agi_half_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<AgiButton>),
    >,
    mut buffer_query: Query<&mut UpgradeBuffer>,
    mut hero_query: Query<&mut StatePoint, With<Hero>>,
) {
    for (interaction, mut background_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                let Ok(mut buffer) = buffer_query.get_single_mut() else {
                    continue;
                };
                let Ok(mut status_point) = hero_query.get_single_mut() else {
                    continue;
                };
                match status_point.half() {
                    Ok(point) => {
                        buffer.agi += point;
                    }
                    Err(err) => info!("Error: {:?}", err),
                }
            }

            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

pub fn interact_with_str_all_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<AllStrButton>),
    >,
    mut buffer_query: Query<&mut UpgradeBuffer>,
    mut hero_query: Query<&mut StatePoint, With<Hero>>,
) {
    for (interaction, mut background_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                let Ok(mut buffer) = buffer_query.get_single_mut() else {
                    continue;
                };
                let Ok(mut status_point) = hero_query.get_single_mut() else {
                    continue;
                };
                match status_point.all() {
                    Ok(point) => {
                        buffer.strength += point;
                    }
                    Err(err) => info!("Error: {:?}", err),
                }
            }

            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

pub fn interact_with_def_all_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<AllDefButton>),
    >,
    mut buffer_query: Query<&mut UpgradeBuffer>,
    mut hero_query: Query<&mut StatePoint, With<Hero>>,
) {
    for (interaction, mut background_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                let Ok(mut buffer) = buffer_query.get_single_mut() else {
                    continue;
                };
                let Ok(mut status_point) = hero_query.get_single_mut() else {
                    continue;
                };
                match status_point.all() {
                    Ok(point) => {
                        buffer.defense += point;
                    }
                    Err(err) => info!("Error: {:?}", err),
                }
            }

            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

pub fn interact_with_agi_all_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<AllAgiButton>),
    >,
    mut buffer_query: Query<&mut UpgradeBuffer>,
    mut hero_query: Query<&mut StatePoint, With<Hero>>,
) {
    for (interaction, mut background_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                let Ok(mut buffer) = buffer_query.get_single_mut() else {
                    continue;
                };
                let Ok(mut status_point) = hero_query.get_single_mut() else {
                    continue;
                };
                match status_point.all() {
                    Ok(point) => {
                        buffer.agi += point;
                    }
                    Err(err) => info!("Error: {:?}", err),
                }
            }

            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

pub fn interact_with_confirm_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ConfirmButton>),
    >,
    mut buffer_query: Query<&mut UpgradeBuffer>,
    mut hero_query: Query<(&mut Strength, &mut Defense, &mut Critical), With<Hero>>,
) {
    for (interaction, mut background_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                let Ok(mut buffer) = buffer_query.get_single_mut() else {
                    continue;
                };
                let Ok((mut strength, mut defense, mut critical)) = hero_query.get_single_mut()
                else {
                    continue;
                };

                strength.upgrade(buffer.strength);
                defense.upgrade(buffer.defense);
                critical.upgrade(buffer.agi);

                buffer.reset();
            }

            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

pub fn interact_with_reset_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResetButton>),
    >,
    mut buffer_query: Query<&mut UpgradeBuffer>,
    mut hero_query: Query<&mut StatePoint, With<Hero>>,
) {
    for (interaction, mut background_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                let Ok(mut buffer) = buffer_query.get_single_mut() else {
                    continue;
                };
                let Ok(mut status_point) = hero_query.get_single_mut() else {
                    continue;
                };

                status_point.plus(buffer.get_total_point());

                buffer.reset();
            }

            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

pub fn interact_with_restart_button(
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    mut spawn_confirm_event: EventWriter<SpawnRestartConfirmPopupEvent>,
    mut status_board_state: ResMut<NextState<OverlayStatusBoardState>>,
) {
    for interaction in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                status_board_state.set(OverlayStatusBoardState::Popup);
                spawn_confirm_event.send(SpawnRestartConfirmPopupEvent);
            }
            _ => (),
        }
    }
}

pub fn interact_with_grid_button(
    mut button_query: Query<
        (Entity, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<SingleDropEqGrid>),
    >,
    mut event: EventWriter<SelectGridEvent>,
) {
    for (entity, interaction, mut bg_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                event.send(SelectGridEvent(Some(entity)));
                *bg_color = BackgroundColor::from(Color::linear_rgb(0.5, 0.5, 0.5));
            }
            _ => {
                *bg_color = BackgroundColor::from(Color::WHITE);
            }
        }
    }
}

pub fn click_to_close(
    buttons: Res<ButtonInput<MouseButton>>,
    mut touch_evr: EventReader<TouchInput>,
    touches: Res<Touches>,
    mut next_board_state: ResMut<NextState<OverlayStatusBoardState>>,

    mut button_query: Query<&Interaction, With<StatusBoardZone>>,
    mut status_button_query: Query<&Interaction, With<StatusBoardButton>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if buttons.any_just_released([MouseButton::Right]) {
        // Left Button was released
        next_board_state.set(OverlayStatusBoardState::Closed);
        next_simulation_state.set(SimulationState::Running);
        info!("Exited status board with mouse")
    }

    if buttons.any_just_released([MouseButton::Left]) {
        if button_query
            .iter_mut()
            .map(|interaction| *interaction == Interaction::None)
            .all(|x| x)
        {
            next_board_state.set(OverlayStatusBoardState::Closed);
            next_simulation_state.set(SimulationState::Running);
            info!("Exited status board with mouse")
        }
    }

    if touches.any_just_pressed() {
        if button_query
            .iter_mut()
            .map(|interaction| *interaction == Interaction::None)
            .all(|x| x)
            && status_button_query
                .iter_mut()
                .map(|interaction| *interaction == Interaction::None)
                .all(|x| x)
        {
            next_board_state.set(OverlayStatusBoardState::Closed);
            next_simulation_state.set(SimulationState::Running);
            info!("Exited status board with touch")
        }
    }
}
