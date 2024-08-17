use bevy::prelude::*;

use crate::{
    audio_effects::ButtonClickEffect,
    overlay::component::{CancelButton, ConfirmPopup, WarningText, WarningTextLayout},
};

use super::{NotEnoughMoneyPopupEvent, SpawnConfirmPopupEvent, SpawnEquipPopupEvent};

pub fn spawn_buy_pop_up(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawn_event: EventReader<SpawnConfirmPopupEvent>,
) {
    for ev in spawn_event.read() {
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
                ConfirmPopup,
            ))
            .id();

        let window = commands
            .spawn((
                Name::new("Confirm window"),
                NodeBundle {
                    background_color: Color::srgb(0.3, 0.3, 0.3).into(),

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

        let warning_text = commands
            .spawn((
                Name::new("Confirm text"),
                TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: format!("Are you sure you want to buy?"),
                            style: TextStyle {
                                font_size: 40.,
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
                    // background_color: NORMAL_BUTTON_COLOR.into(),
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
                CancelButton,
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
                    // background_color: NORMAL_BUTTON_COLOR.into(),
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
                // ConfirmButton(ev.0),
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
            .push_children(&[warning_text, button_row]);
        commands
            .entity(button_row)
            .push_children(&[confirm, cancel]);
    }
}

pub fn despawn_pop_up(mut commands: Commands, pop_up_query: Query<Entity, With<ConfirmPopup>>) {
    for pop_up in pop_up_query.iter() {
        commands.entity(pop_up).despawn_recursive();
    }
}

// spawn only right after the brought pop up confirm
pub fn spawn_equip_pop_up(
    mut commands: Commands,
    pop_up_query: Query<Entity, With<ConfirmPopup>>,
    asset_server: Res<AssetServer>,
    mut spawn_event: EventReader<SpawnEquipPopupEvent>,
) {
    for ev in spawn_event.read() {
        for pop_up in pop_up_query.iter() {
            commands.entity(pop_up).despawn_descendants();

            let window = commands
                .spawn((
                    Name::new("Confirm window"),
                    NodeBundle {
                        background_color: Color::srgb(0.3, 0.3, 0.3).into(),

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

            let warning_text = commands
                .spawn((
                    Name::new("Confirm text"),
                    TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: format!("Do you want to equip it?"),
                                style: TextStyle {
                                    font_size: 40.,
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
                        // background_color: NORMAL_BUTTON_COLOR.into(),
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
                    CancelButton,
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
                        // background_color: NORMAL_BUTTON_COLOR.into(),
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
                    // EquireButton(ev.0),
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

            commands.entity(pop_up).add_child(window);
            commands
                .entity(window)
                .push_children(&[warning_text, button_row]);
            commands
                .entity(button_row)
                .push_children(&[confirm, cancel]);
        }
    }
}

pub fn spawn_warning_text(
    mut commands: Commands,
    mut not_enough_event: EventReader<NotEnoughMoneyPopupEvent>,
    asset_server: Res<AssetServer>,
) {
    // let center = camera_query
    //     .get_single()
    //     .expect("No Camera been found")
    //     .translation;

    for _ in not_enough_event.read() {
        let layout = commands
            .spawn((
                Name::new("Confirm Popup"),
                NodeBundle {
                    background_color: Color::NONE.into(),

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
                WarningTextLayout(Timer::from_seconds(2.0, TimerMode::Once)),
            ))
            .id();
        let child: Entity = commands
            .spawn((
                TextBundle {
                    style: Style {
                        bottom: Val::Px(0.),
                        ..default()
                    },
                    text: Text {
                        sections: {
                            vec![TextSection {
                                value: String::from("Not Enough Money"),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                    font_size: 40.0,
                                    color: Color::srgb(0.5, 0.43, 0.234),
                                },
                            }]
                        },
                        ..default()
                    },
                    ..Default::default()
                },
                WarningText,
            ))
            .id();
        commands.entity(layout).add_child(child);
    }
}

pub fn update_warning_text(
    mut commands: Commands,
    mut text_layout_query: Query<
        (Entity, &mut WarningTextLayout, &Children),
        With<WarningTextLayout>,
    >,
    mut text_query: Query<&mut Style, With<WarningText>>,
    time: Res<Time>,
) {
    for (entity, mut warning, children) in text_layout_query.iter_mut() {
        for child in children.iter() {
            if let Ok(mut style) = text_query.get_mut(*child) {
                if let Val::Px(px) = style.bottom {
                    style.bottom = Val::Px(px + 100. * time.delta_seconds());
                }
            };
        }
        if warning.0.tick(time.delta()).finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
