use crate::audio_effects::ButtonClickEffect;
use crate::game::overlay::board::components::*;
use crate::game::overlay::board::styles::*;
use crate::state::SimulationState;
use crate::state::TerminalState;

use crate::game::TerminalBundle;
use crate::IPHONE_BAND_SPACER_HEIGHT;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

// struct StatusPack<'a> {
//     pub strength: &'a Strength,
//     pub defense: &'a Defense,
//     pub critical: &'a Critical,
//     pub state_point: &'a StatePoint,
//     pub money: &'a Money,
// }

// impl<'a> StatusPack<'a> {
//     fn new(
//         strength: &'a Strength,
//         defense: &'a Defense,
//         critical: &'a Critical,
//         state_point: &'a StatePoint,
//         money: &'a Money,
//     ) -> Self {
//         StatusPack {
//             strength,
//             defense,
//             critical,
//             state_point,
//             money,
//         }
//     }
// }

pub fn spawn_status_board(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_input_state: ResMut<NextState<SimulationState>>,
    mut next_terminal_state: ResMut<NextState<TerminalState>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    info!("Status Board Spawned");
    // let status_pack = StatusPack::new(strength, defense, critical, state_point, money);

    // let image_handler: Handle<Image> = match hero_class {
    //     HeroClass::Paladin => asset_server.load("images/paladin.png"),
    //     HeroClass::Warrior => asset_server.load("images/paladin.png"),
    //     HeroClass::Rogue => asset_server.load("images/paladin.png"),
    // };

    let window_height = window_query.single().height();

    build_status_board(&mut commands, &asset_server, window_height);
    next_terminal_state.set(TerminalState::Auto);
    next_input_state.set(SimulationState::Pause);
}

pub fn despawn_status_board(
    mut commands: Commands,
    status_board_query: Query<Entity, With<StatusBoardFrame>>,
    mut next_input_state: ResMut<NextState<SimulationState>>,
    mut next_terminal_state: ResMut<NextState<TerminalState>>,
) {
    for status_board_entity in status_board_query.iter() {
        commands.entity(status_board_entity).despawn_recursive();
    }
    next_input_state.set(SimulationState::Running);
    next_terminal_state.set(TerminalState::None);
}

fn build_status_board(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    window_height: f32,
) {
    let layout_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    // padding: UiRect::top(Val::Px(82.)),
                    justify_content: JustifyContent::SpaceBetween,
                    ..Style::DEFAULT
                },
                z_index: ZIndex::Global(10),
                ..default()
            },
            StatusBoardFrame,
            UpgradeBuffer::default(),
            Name::new("Status Board Frame"),
        ))
        .id();

    let header_spacer = commands
        .spawn((
            NodeBundle {
                style: Style {
                    min_height: Val::Px(IPHONE_BAND_SPACER_HEIGHT),
                    min_width: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
            StatusBoardZone {},
        ))
        .id();

    let spacer = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    max_height: Val::Percent(100.0),
                    justify_content: JustifyContent::Stretch,
                    ..Style::DEFAULT
                },
                background_color: Color::srgba(0.4, 0.4, 0.4, 0.7).into(),
                ..default()
            },
            Name::new("Status Board Spacer"),
        ))
        .id();

    let top_container = commands
        .spawn((
            Name::new("Top Container"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let features_buttons_container = commands
        .spawn((
            Name::new("features buttons container"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::End,
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let features_buttons_list = commands
        .spawn((
            Name::new("features buttons container"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::top(Val::Px(10.)),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let restart_button = commands
        .spawn((
            ButtonBundle {
                background_color: NORMAL_BUTTON_COLOR.into(),
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Px(100.0),
                    height: Val::Px(35.0),
                    margin: UiRect::all(Val::Px(10.)),
                    ..Style::DEFAULT
                },
                ..Default::default()
            },
            RestartButton {},
            ButtonClickEffect,
            StatusBoardZone {},
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Restart".to_string(),
                        TextStyle {
                            font_size: 15.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    )],
                    justify: JustifyText::Right,
                    ..default()
                },
                ..default()
            },));
        })
        .id();

    let top_assets = commands
        .spawn((
            Name::new("Hero Assets"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_content: AlignContent::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let board = commands
        .spawn((
            Name::new("StatusBoard"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Auto,
                    height: Val::Auto,
                    min_height: Val::Px(300.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    padding: UiRect {
                        left: Val::Px(10.),
                        right: Val::Px(10.),
                        top: Val::Px(10.),
                        bottom: Val::Px(20.),
                    },
                    margin: UiRect::all(Val::Px(10.)),

                    ..Style::DEFAULT
                },
                ..default()
            },
            Interaction::None,
            StatusBoard {},
            StatusBoardZone {},
        ))
        .id();

    let exp_container = commands
        .spawn((
            Name::new("Exp container"),
            ExperienceUI,
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.),
                    height: Val::Px(20.),
                    ..Style::DEFAULT
                },

                ..default()
            },
        ))
        .id();

    let exp_text = commands
        .spawn((TextBundle {
            text: Text {
                sections: vec![TextSection::new(
                    format!("Exp until next lv"),
                    TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 10.0,
                        color: Color::WHITE,
                    },
                )],
                justify: JustifyText::Center,
                ..default()
            },
            ..default()
        },))
        .id();

    let confirm_row = commands
        .spawn((
            Name::new("Comfirm Row"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    width: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,

                    ..Style::DEFAULT
                },
                ..default()
            },
            StatusBoardRow,
        ))
        .id();

    let row = commands
        .spawn((
            Name::new("Main Row"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    width: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,

                    ..Style::DEFAULT
                },
                background_color: Color::linear_rgb(0.7, 0.7, 0.7).into(),
                ..default()
            },
            StatusBoardRow,
        ))
        .id();

    // let available_point = commands
    //     .spawn((
    //         Name::new("Available Point"),
    //         TextBundle {
    //             style: Style {
    //                 height: Val::Px(30.0),
    //                 margin: UiRect::all(Val::Px(30.)),
    //                 ..Style::DEFAULT
    //             },
    //             text: Text {
    //                 sections: vec![TextSection::new(
    //                     format!("Point: {}", status_pack.state_point.get()),
    //                     TextStyle {
    //                         font: asset_server.load("fonts/FiraMono-Medium.ttf"),
    //                         font_size: 30.0,
    //                         color: Color::WHITE,
    //                     },
    //                 )],
    //                 justify: JustifyText::Center,
    //                 ..default()
    //             },
    //             ..default()
    //         },
    //         AvailablePointText,
    //     ))
    //     .id();

    let money = commands
        .spawn((
            Name::new("Money"),
            TextBundle {
                style: Style {
                    height: Val::Px(20.0),
                    margin: UiRect::all(Val::Px(20.)),
                    ..Style::DEFAULT
                },
                text: Text {
                    sections: vec![TextSection::new(
                        format!("$"),
                        TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    )],
                    justify: JustifyText::Center,
                    ..default()
                },
                ..default()
            },
            MoneyText(0.),
        ))
        .id();

    let terminal_node = commands
        .spawn((
            Name::new("Terminal Spacer"),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(100.0),
                    max_height: Val::Percent(100.0),
                    border: UiRect::all(Val::Px(8.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    // margin: UiRect::top(Val::Px(30.)),
                    overflow: Overflow::clip(),
                    ..default()
                },
                border_color: Color::BLACK.into(),
                background_color: Color::srgb(103., 102., 81.).into(),
                ..default()
            },
            TerminalBundle::new_with_auto(22.),
        ))
        .id();

    // let hero_picture = commands
    //     .spawn((
    //         // replace with image bundle later on
    //         Name::new("HeroPic"),
    //         ImageBundle {
    //             style: Style {
    //                 width: Val::Vw(40.0),
    //                 min_width: Val::Px(100.0),
    //                 max_width: Val::Px(170.),
    //                 height: Val::Px(150.0),
    //                 align_items: AlignItems::Center,
    //                 justify_content: JustifyContent::SpaceBetween,
    //                 padding: UiRect::horizontal(Val::Px(25.)),
    //                 ..Style::DEFAULT
    //             },
    //             image: hero_image.clone().into(),
    //             background_color: Color::linear_rgb(0.7, 0.7, 0.7).into(),
    //             ..default()
    //         },
    //         HeroPicture,
    //     ))
    //     .id();

    // let statuses = commands
    //     .spawn((
    //         Name::new("All Status"),
    //         NodeBundle {
    //             background_color: Color::linear_rgb(0.7, 0.7, 0.7).into(),
    //             style: Style {
    //                 flex_direction: FlexDirection::Column,
    //                 width: Val::Percent(100.),
    //                 height: Val::Percent(100.),
    //                 align_items: AlignItems::Center,
    //                 justify_content: JustifyContent::SpaceEvenly,
    //                 padding: UiRect::horizontal(Val::Px(10.)),
    //                 ..Style::DEFAULT
    //             },
    //             ..default()
    //         },
    //         StatusBoardStatues,
    //     ))
    //     .id();

    // let str_stat = commands
    //     .spawn((
    //         Name::new("Str"),
    //         NodeBundle {
    //             style: get_stat_row_style(),

    //             ..default()
    //         },
    //         StrStateNode,
    //     ))
    //     .id();

    // let str_display = commands
    //     .spawn((
    //         TextBundle {
    //             style: get_stat_text_style(),
    //             text: Text {
    //                 sections: vec![
    //                     TextSection::new(
    //                         format!("Str: "),
    //                         TextStyle {
    //                             font: asset_server.load("fonts/FiraMono-Medium.ttf"),
    //                             font_size: 20.0,
    //                             color: Color::WHITE,
    //                         },
    //                     ),
    //                     TextSection::new(
    //                         format!(""),
    //                         TextStyle {
    //                             font: asset_server.load("fonts/FiraMono-Medium.ttf"),
    //                             font_size: 15.0,
    //                             color: Color::WHITE,
    //                         },
    //                     ),
    //                 ],
    //                 justify: JustifyText::Center,
    //                 ..default()
    //             },
    //             ..default()
    //         },
    //         StrText {},
    //     ))
    //     .id();

    // // let str_button = commands
    // //     .spawn((
    // //         ButtonBundle {
    // //             background_color: NORMAL_BUTTON_COLOR.into(),
    // //             style: get_state_button_style(),
    // //             ..Default::default()
    // //         },
    // //         StrButton {},
    // //         ButtonClickEffect,
    // //         StatusBoardZone {},
    // //     ))
    // //     .with_children(|parent| {
    // //         parent.spawn((
    // //             TextBundle {
    // //                 text: Text {
    // //                     sections: vec![TextSection::new(
    // //                         "Half".to_string(),
    // //                         TextStyle {
    // //                             font_size: 15.0,
    // //                             color: Color::WHITE,
    // //                             ..default()
    // //                         },
    // //                     )],
    // //                     justify: JustifyText::Right
    // //                     ..default()
    // //                 },
    // //                 ..default()
    // //             },
    // //             StrButtonText {},
    // //         ));
    // //     })
    // //     .id();

    // let def_stat = commands
    //     .spawn((
    //         Name::new("Def"),
    //         NodeBundle {
    //             style: get_stat_row_style(),
    //             background_color: Color::linear_rgb(0.7, 0.7, 0.7).into(),
    //             ..default()
    //         },
    //         DefStateNode,
    //     ))
    //     .id();

    // let def_display = commands
    //     .spawn((
    //         TextBundle {
    //             style: get_stat_text_style(),
    //             text: Text {
    //                 sections: vec![
    //                     TextSection::new(
    //                         format!("Def: "),
    //                         TextStyle {
    //                             font: asset_server.load("fonts/FiraMono-Medium.ttf"),
    //                             font_size: 20.0,
    //                             color: Color::WHITE,
    //                         },
    //                     ),
    //                     TextSection::new(
    //                         format!(""),
    //                         TextStyle {
    //                             font: asset_server.load("fonts/FiraMono-Medium.ttf"),
    //                             font_size: 15.0,
    //                             color: Color::WHITE,
    //                         },
    //                     ),
    //                 ],
    //                 justify: JustifyText::Center,
    //                 ..default()
    //             },
    //             ..default()
    //         },
    //         DefText {},
    //     ))
    //     .id();

    // // let def_button = commands
    // //     .spawn((
    // //         ButtonBundle {
    // //             background_color: NORMAL_BUTTON_COLOR.into(),
    // //             style: get_state_button_style(),
    // //             ..Default::default()
    // //         },
    // //         DefButton {},
    // //         ButtonClickEffect,
    // //         StatusBoardZone {},
    // //     ))
    // //     .with_children(|parent| {
    // //         parent.spawn((
    // //             TextBundle {
    // //                 text: Text {
    // //                     sections: vec![TextSection::new(
    // //                         "Half".to_string(),
    // //                         TextStyle {
    // //                             font_size: 15.0,
    // //                             color: Color::WHITE,
    // //                             ..default()
    // //                         },
    // //                     )],
    // //                     justify: JustifyText::Right
    // //                     ..default()
    // //                 },
    // //                 ..default()
    // //             },
    // //             DefButtonText {},
    // //         ));
    // //     })
    // //     .id();

    // let agi_stat = commands
    //     .spawn((
    //         Name::new("Agi"),
    //         NodeBundle {
    //             style: get_stat_row_style(),
    //             background_color: Color::linear_rgb(0.7, 0.7, 0.7).into(),
    //             ..default()
    //         },
    //         AgiStateNode,
    //     ))
    //     .id();

    // let agi_display = commands
    //     .spawn((
    //         TextBundle {
    //             style: get_stat_text_style(),
    //             text: Text {
    //                 sections: vec![
    //                     TextSection::new(
    //                         format!("Agi: "),
    //                         TextStyle {
    //                             font: asset_server.load("fonts/FiraMono-Medium.ttf"),
    //                             font_size: 20.0,
    //                             color: Color::WHITE,
    //                         },
    //                     ),
    //                     TextSection::new(
    //                         format!(""),
    //                         TextStyle {
    //                             font: asset_server.load("fonts/FiraMono-Medium.ttf"),
    //                             font_size: 15.0,
    //                             color: Color::WHITE,
    //                         },
    //                     ),
    //                 ],
    //                 justify: JustifyText::Center,
    //                 ..default()
    //             },
    //             ..default()
    //         },
    //         AgiText {},
    //     ))
    //     .id();

    // // let agi_button = commands
    // //     .spawn((
    // //         ButtonBundle {
    // //             background_color: NORMAL_BUTTON_COLOR.into(),
    // //             style: get_state_button_style(),
    // //             ..Default::default()
    // //         },
    // //         AgiButton {},
    // //         ButtonClickEffect,
    // //         StatusBoardZone {},
    // //     ))
    // //     .with_children(|parent| {
    // //         parent.spawn((
    // //             TextBundle {
    // //                 text: Text {
    // //                     sections: vec![TextSection::new(
    // //                         "Half".to_string(),
    // //                         TextStyle {
    // //                             font_size: 15.0,
    // //                             color: Color::WHITE,
    // //                             ..default()
    // //                         },
    // //                     )],
    // //                     justify: JustifyText::Right
    // //                     ..default()
    // //                 },
    // //                 ..default()
    // //             },
    // //             AgiButtonText {},
    // //         ));
    // //     })
    // //     .id();

    // let confirm_button_group = commands
    //     .spawn((
    //         Name::new("Confirm Button Group"),
    //         NodeBundle {
    //             style: Style {
    //                 flex_direction: FlexDirection::Row,
    //                 ..default()
    //             },
    //             ..default()
    //         },
    //     ))
    //     .id();

    // // let reset_button = commands
    // //     .spawn((
    // //         ButtonBundle {
    // //             background_color: NORMAL_BUTTON_COLOR.into(),
    // //             border_color: Color::WHITE.into(),
    // //             style: Style {
    // //                 justify_content: JustifyContent::Center,
    // //                 align_items: AlignItems::Center,
    // //                 width: Val::Px(70.0),
    // //                 height: Val::Px(40.0),
    // //                 border: UiRect {
    // //                     left: Val::Px(2.),
    // //                     right: Val::Px(2.),
    // //                     top: Val::Px(2.),
    // //                     bottom: Val::Px(0.),
    // //                 },
    // //                 ..Style::DEFAULT
    // //             },
    // //             ..Default::default()
    // //         },
    // //         ResetButton {},
    // //         ButtonClickEffect,
    // //         StatusBoardZone {},
    // //     ))
    // //     .with_children(|parent| {
    // //         parent.spawn((TextBundle {
    // //             text: Text {
    // //                 sections: vec![TextSection::new(
    // //                     "Reset".to_string(),
    // //                     TextStyle {
    // //                         font_size: 15.0,
    // //                         color: Color::WHITE,
    // //                         ..default()
    // //                     },
    // //                 )],
    // //                 justify: JustifyText::Right
    // //                 ..default()
    // //             },
    // //             ..default()
    // //         },));
    // //     })
    // //     .id();

    // // let confirm_button = commands
    // //     .spawn((
    // //         ButtonBundle {
    // //             background_color: NORMAL_BUTTON_COLOR.into(),
    // //             border_color: Color::WHITE.into(),
    // //             style: Style {
    // //                 justify_content: JustifyContent::Center,
    // //                 align_items: AlignItems::Center,
    // //                 width: Val::Px(70.0),
    // //                 height: Val::Px(40.0),
    // //                 border: UiRect {
    // //                     left: Val::Px(0.),
    // //                     right: Val::Px(2.),
    // //                     top: Val::Px(2.),
    // //                     bottom: Val::Px(0.),
    // //                 },
    // //                 ..Style::DEFAULT
    // //             },
    // //             ..Default::default()
    // //         },
    // //         ConfirmButton {},
    // //         ButtonClickEffect,
    // //         StatusBoardZone {},
    // //     ))
    // //     .with_children(|parent| {
    // //         parent.spawn((TextBundle {
    // //             text: Text {
    // //                 sections: vec![TextSection::new(
    // //                     "Confirm".to_string(),
    // //                     TextStyle {
    // //                         font_size: 15.0,
    // //                         color: Color::WHITE,
    // //                         ..default()
    // //                     },
    // //                 )],
    // //                 justify: JustifyText::Right
    // //                 ..default()
    // //             },
    // //             ..default()
    // //         },));
    // //     })
    // //     .id();

    // // let all_agi_button = commands
    // //     .spawn((
    // //         ButtonBundle {
    // //             background_color: NORMAL_BUTTON_COLOR.into(),
    // //             style: get_state_button_style(),
    // //             ..Default::default()
    // //         },
    // //         AllAgiButton {},
    // //         ButtonClickEffect,
    // //         StatusBoardZone {},
    // //     ))
    // //     .with_children(|parent| {
    // //         parent.spawn((TextBundle {
    // //             text: Text {
    // //                 sections: vec![TextSection::new(
    // //                     "All".to_string(),
    // //                     TextStyle {
    // //                         font_size: 15.0,
    // //                         color: Color::WHITE,
    // //                         ..default()
    // //                     },
    // //                 )],
    // //                 justify: JustifyText::Right
    // //                 ..default()
    // //             },
    // //             ..default()
    // //         },));
    // //     })
    // //     .id();

    // // let all_dex_button = commands
    // //     .spawn((
    // //         ButtonBundle {
    // //             background_color: NORMAL_BUTTON_COLOR.into(),
    // //             style: get_state_button_style(),
    // //             ..Default::default()
    // //         },
    // //         AllDefButton {},
    // //         ButtonClickEffect,
    // //         StatusBoardZone {},
    // //     ))
    // //     .with_children(|parent| {
    // //         parent.spawn((TextBundle {
    // //             text: Text {
    // //                 sections: vec![TextSection::new(
    // //                     "All".to_string(),
    // //                     TextStyle {
    // //                         font_size: 15.0,
    // //                         color: Color::WHITE,
    // //                         ..default()
    // //                     },
    // //                 )],
    // //                 justify: JustifyText::Right
    // //                 ..default()
    // //             },
    // //             ..default()
    // //         },));
    // //     })
    // //     .id();

    // // let all_str_button = commands
    // //     .spawn((
    // //         ButtonBundle {
    // //             background_color: NORMAL_BUTTON_COLOR.into(),
    // //             style: get_state_button_style(),
    // //             ..Default::default()
    // //         },
    // //         AllStrButton {},
    // //         ButtonClickEffect,
    // //         StatusBoardZone {},
    // //     ))
    // //     .with_children(|parent| {
    // //         parent.spawn((TextBundle {
    // //             text: Text {
    // //                 sections: vec![TextSection::new(
    // //                     "All".to_string(),
    // //                     TextStyle {
    // //                         font_size: 15.0,
    // //                         color: Color::WHITE,
    // //                         ..default()
    // //                     },
    // //                 )],
    // //                 justify: JustifyText::Right
    // //                 ..default()
    // //             },
    // //             ..default()
    // //         },));
    // //     })
    // //     .id();

    let mid_container = commands
        .spawn((
            Name::new("Mid Container"),
            BagGrid,
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    max_height: Val::Px(window_height - 300.),
                    margin: UiRect::horizontal(Val::Px(8.)),
                    row_gap: Val::Px(45.0),
                    grid_template_columns: RepeatedGridTrack::flex(5, 1.0),
                    grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
                    aspect_ratio: Some(1.0),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    commands
        .entity(layout_entity)
        .push_children(&[header_spacer, spacer]);
    commands
        .entity(spacer)
        .push_children(&[top_container, mid_container, board]);

    commands.entity(board).push_children(&[confirm_row, row]);

    commands
        .entity(top_container)
        .push_children(&[top_assets, features_buttons_container]);

    commands
        .entity(features_buttons_container)
        .add_child(features_buttons_list);

    commands
        .entity(features_buttons_list)
        .add_child(restart_button);

    commands.entity(top_assets).push_children(&[money]);

    commands.entity(confirm_row).push_children(&[exp_container]);

    commands.entity(exp_container).add_child(exp_text);

    commands.entity(row).add_child(terminal_node);

    // commands
    //     .entity(confirm_button_group)
    //     .push_children(&[reset_button, confirm_button]);

    // commands
    //     .entity(row)
    //     .push_children(&[hero_picture, statuses]);

    // commands
    //     .entity(statuses)
    //     .push_children(&[str_stat, def_stat, agi_stat]);

    // commands.entity(str_stat).push_children(&[str_display]);
    // commands.entity(def_stat).push_children(&[def_display]);
    // commands.entity(agi_stat).push_children(&[agi_display]);
}
