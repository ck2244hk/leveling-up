use crate::audio_effects::ButtonClickEffect;
use crate::game::board::components::StatusBoardZone;
use crate::game::overlay::header::components::{
    EncounterGaugeBuilder, Header, InDangerDisplay, LevelDisplay, StatusBoardButton,
    StatusBoardButtonText, TurnsText,
};
use crate::game::overlay::header::styles::*;
use crate::game::GameUIFrame;
use crate::state::OverlayStatusBoardState;
use bevy::prelude::*;

use super::components::ExplorationLayout;

pub fn spawn_header(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    status_board_state: Res<State<OverlayStatusBoardState>>,
    layout_query: Query<Entity, With<GameUIFrame>>,
) {
    let layout_entity = layout_query.get_single().expect("No Layout Wrapper");
    build_header(
        &mut commands,
        &asset_server,
        &status_board_state,
        layout_entity,
    );
}

pub fn despawn_header(
    mut commands: Commands,
    header_query: Query<Entity, With<ExplorationLayout>>,
    layout_query: Query<Entity, With<GameUIFrame>>,
) {
    if let Ok(layout_entity) = layout_query.get_single() {
        if let Ok(header_entity) = header_query.get_single() {
            commands
                .entity(layout_entity)
                .remove_children(&[header_entity]);
            commands.entity(header_entity).despawn_recursive();
        }
    }
}

pub fn build_header(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    status_board_state: &Res<State<OverlayStatusBoardState>>,
    layout_entity: Entity,
) {
    let space_between = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    height: Val::Vh(100.),
                    ..default()
                },
                z_index: ZIndex::Global(3),
                ..default()
            },
            Name::new("ExplorationLayout"),
            ExplorationLayout,
        ))
        .id();
    let header = commands
        .spawn((
            NodeBundle {
                style: get_main_menu_style(),
                background_color: Color::srgba(0.3, 0.3, 0.3, 0.7).into(),
                ..default()
            },
            Name::new("Header"),
            Header {},
        ))
        .id();

    let level_display = commands
        .spawn((
            TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Lv: ".to_string(),
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
            LevelDisplay {},
        ))
        .id();

    let in_danger_display = commands
        .spawn((
            TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Safe".to_string(),
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
            InDangerDisplay {},
        ))
        .id();

    let turn_left_text = commands
        .spawn((
            TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Turns: ".to_string(),
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
            TurnsText {},
        ))
        .id();

    let pause_button_text = if *status_board_state.get() == OverlayStatusBoardState::Opened {
        "Menu"
    } else {
        "Back"
    };

    println!("Display Pause Button");
    let status_board_button = commands
        .spawn((
            ButtonBundle {
                background_color: NORMAL_BUTTON_COLOR.into(),
                style: get_normal_button_style(),
                ..default()
            },
            StatusBoardButton {},
            StatusBoardZone {},
            ButtonClickEffect,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            pause_button_text,
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
                },
                StatusBoardButtonText {},
            ));
        })
        .id();

    let encounter_gauge_container = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            Name::new("Encounter Gauge Container"),
        ))
        .id();

    let encounter_text = commands
        .spawn((
            TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Encounter Gauge".to_string(),
                        style: TextStyle {
                            font_size: 15.,
                            ..default()
                        },
                    }],
                    justify: JustifyText::Center,
                    ..default()
                },
                style: Style {
                    margin: UiRect::left(Val::Px(15.)),
                    ..default()
                },
                ..default()
            },
            Name::new("Encounter Text"),
        ))
        .id();

    let encounter_gauge = EncounterGaugeBuilder::build(commands);

    commands.entity(layout_entity).add_child(space_between);
    commands
        .entity(space_between)
        .push_children(&[header, encounter_gauge_container]);
    commands.entity(header).push_children(&[
        level_display,
        turn_left_text,
        in_danger_display,
        status_board_button,
    ]);

    commands
        .entity(encounter_gauge_container)
        .push_children(&[encounter_text, encounter_gauge]);
}
