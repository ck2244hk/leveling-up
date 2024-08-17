use bevy::prelude::*;

use crate::game::overlay::board::components::{MenuButton, ShopButton};
use crate::game::overlay::header::styles::{NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR};
use crate::state::{OverlayScoreBoardState, OverlayShopState, Screen};

pub fn interact_with_score_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MenuButton>),
    >,
    mut next_app_state: ResMut<NextState<Screen>>,
    mut next_score_board_state: ResMut<NextState<OverlayScoreBoardState>>,
) {
    for (interaction, mut background_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();

                next_app_state.set(Screen::Title);
                next_score_board_state.set(OverlayScoreBoardState::Closed);
                info!("Back To Menu");
            }

            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),

            _ => (),
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
