use components::{Header, InDangerDisplay};

use crate::{
    game::{
        character::component::{BaseStates, Hero, Turns},
        Encounter, Player,
    },
    state::OverlayStatusBoardState,
};

use self::components::{EncounterGaugeBar, LevelDisplay, StatusBoardButtonText, TurnsText};

use super::*;

pub fn update_status_board_button(
    mut status_board_button_query: Query<&mut Text, With<StatusBoardButtonText>>,
    status_board_state: Res<State<OverlayStatusBoardState>>,
) {
    for mut status_board_button_text in status_board_button_query.iter_mut() {
        if *status_board_state.get() == OverlayStatusBoardState::Opened {
            status_board_button_text.sections[0].value = "Close".to_string();
        }

        if *status_board_state.get() == OverlayStatusBoardState::Closed {
            status_board_button_text.sections[0].value = "Menu".to_string();
        }
    }
}

pub fn update_encounter_gauge(
    player_query: Query<&Encounter, With<Player>>,
    mut bar_query: Query<(&mut Style, &mut BackgroundColor), With<EncounterGaugeBar>>,
    time: Res<Time>,
) {
    // let chance = area_query.iter().fold(0., |accum, a| {
    //     if a.percentage > accum {
    //         a.percentage
    //     } else {
    //         accum
    //     }
    // });

    for (mut bar_style, _background_color) in &mut bar_query {
        let Val::Percent(current_width) = bar_style.width else {
            continue;
        };
        let Ok(chance) = player_query.get_single() else {
            warn!("Cannot find player");
            continue;
        };

        // info!(
        //     "Encounter Percentage: {}, chance: {}",
        //     current_width + (chance - current_width) * 2. * time.delta_seconds(),
        //     chance
        // );
        bar_style.width = Val::Percent(
            current_width + (chance.percentage - current_width) * 2. * time.delta_seconds(),
        );
    }
}

pub fn update_level_display(
    mut level_display_query: Query<&mut Text, With<LevelDisplay>>,
    player_query: Query<&BaseStates, With<Hero>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(mut level_display_text) = level_display_query.get_single_mut() {
        if let Ok(player_state) = player_query.get_single() {
            level_display_text.sections = vec![TextSection::new(
                "Lv: ".to_string() + &player_state.lv().to_string(),
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            )]
        }
    }
}

pub fn update_turns_display(
    mut turns_text_query: Query<&mut Text, With<TurnsText>>,
    player_query: Query<&Turns, With<Hero>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(mut turns_text) = turns_text_query.get_single_mut() {
        if let Ok(player_turns) = player_query.get_single() {
            turns_text.sections = vec![TextSection::new(
                "Turns: ".to_string() + &player_turns.get().to_string(),
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            )]
        }
    }
}

pub fn update_in_danger_display(
    mut turns_text_query: Query<&mut Text, With<InDangerDisplay>>,
    player_query: Query<&Encounter, With<Player>>,
    asset_server: Res<AssetServer>,
    mut header_query: Query<&mut BackgroundColor, With<Header>>,
) {
    if let Ok(mut turns_text) = turns_text_query.get_single_mut() {
        let Ok(mut header) = header_query.get_single_mut() else {
            return;
        };
        if let Ok(player_encounter) = player_query.get_single() {
            if let Some(field) = player_encounter.field {
                turns_text.sections[0].value = format!("In Danger: {}lv", field);
                *header = Color::srgba(1., 0.3, 0.3, 0.7).into();
            } else {
                turns_text.sections = vec![TextSection::new(
                    format!("Safe"),
                    TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                )];
                *header = Color::srgba(0.3, 0.3, 0.3, 0.7).into();
            }
        }
    }
}
