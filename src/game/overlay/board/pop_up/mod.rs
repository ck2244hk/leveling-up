use bevy::prelude::*;

use crate::{
    audio_effects::ButtonClickEffect,
    state::{OverlayStatusBoardState, Screen},
};

use super::{styles::NORMAL_BUTTON_COLOR, SpawnRestartConfirmPopupEvent};

mod interactions;

use interactions::*;

pub struct MenuPopupPlugin;

impl Plugin for MenuPopupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_confirm_pop_up.run_if(in_state(Screen::Playing)),
        )
        .add_systems(
            Update,
            (
                interact_with_confirm_restart_button,
                interact_with_cancel_buy_button,
            )
                .run_if(in_state(OverlayStatusBoardState::Popup)),
        )
        .add_systems(OnExit(OverlayStatusBoardState::Popup), despawn_pop_up);
    }
}

#[derive(Component)]
struct RestartConfirmPopup;

#[derive(Component)]
struct CancelRestartButton;

#[derive(Component)]
struct ConfirmRestartButton;

pub fn spawn_confirm_pop_up(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawn_event: EventReader<SpawnRestartConfirmPopupEvent>,
) {
    for _ in spawn_event.read() {
        let popup_layout = commands
            .spawn((
                Name::new("Confirm Popup"),
                NodeBundle {
                    background_color: Color::srgba(0.0, 0.0, 0.0, 0.3).into(),

                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        justify_items: JustifyItems::Center,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },

                    z_index: ZIndex::Global(99),
                    ..default()
                },
                RestartConfirmPopup,
            ))
            .id();

        let window = commands
            .spawn((
                Name::new("Confirm window"),
                NodeBundle {
                    background_color: Color::linear_rgb(0.5, 0.5, 0.5).into(),

                    style: Style {
                        flex_direction: FlexDirection::Column,
                        width: Val::Px(400.),
                        height: Val::Px(300.),
                        margin: UiRect::horizontal(Val::Px(20.)),
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    ..default()
                },
            ))
            .id();

        let text_container = commands
            .spawn((
                Name::new("Text Container"),
                NodeBundle {
                    background_color: Color::linear_rgb(0.5, 0.5, 0.5).into(),

                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                },
            ))
            .id();

        let warning_text = commands
            .spawn((
                Name::new("Confirm text"),
                TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: format!(
                                "You are going to lose your current progress.\n\nAre you sure?"
                            ),
                            style: TextStyle {
                                font_size: 30.,
                                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                color: Color::WHITE,
                            },
                        }],
                        ..default()
                    },
                    ..default()
                },
            ))
            .id();

        let button_row = commands
            .spawn((
                Name::new("Confirm Row"),
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        height: Val::Px(100.),
                        ..default()
                    },
                    ..default()
                },
            ))
            .id();

        let cancel = commands
            .spawn((
                Name::new("Cancel"),
                ButtonBundle {
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    border_color: Color::BLACK.into(),
                    style: Style {
                        width: Val::Percent(50.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        border: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    ..default()
                },
                CancelRestartButton,
                ButtonClickEffect,
            ))
            .with_children(|builder| {
                builder.spawn(TextBundle {
                    style: Style {
                        margin: UiRect {
                            left: Val::Px(10.),
                            right: Val::Px(10.),
                            top: Val::ZERO,
                            bottom: Val::ZERO,
                        },
                        ..default()
                    },
                    text: Text {
                        sections: {
                            vec![TextSection {
                                value: String::from("Cancel"),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            }]
                        },
                        ..default()
                    },
                    ..Default::default()
                });
            })
            .id();

        let confirm = commands
            .spawn((
                Name::new("Confirm"),
                ButtonBundle {
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    border_color: Color::BLACK.into(),
                    style: Style {
                        width: Val::Percent(50.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        border: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    ..default()
                },
                ButtonClickEffect,
                ConfirmRestartButton,
            ))
            .with_children(|builder| {
                builder.spawn(TextBundle {
                    style: Style {
                        margin: UiRect {
                            left: Val::Px(10.),
                            right: Val::Px(10.),
                            top: Val::ZERO,
                            bottom: Val::ZERO,
                        },
                        ..default()
                    },
                    text: Text {
                        sections: {
                            vec![TextSection {
                                value: String::from("Confirm"),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            }]
                        },
                        ..default()
                    },
                    ..Default::default()
                });
            })
            .id();

        commands.entity(popup_layout).add_child(window);
        commands
            .entity(window)
            .push_children(&[text_container, button_row]);

        commands
            .entity(text_container)
            .push_children(&[warning_text]);

        commands
            .entity(button_row)
            .push_children(&[confirm, cancel]);
    }
}

fn despawn_pop_up(mut commands: Commands, pop_up_query: Query<Entity, With<RestartConfirmPopup>>) {
    for pop_up in pop_up_query.iter() {
        commands.entity(pop_up).despawn_recursive();
    }
}
