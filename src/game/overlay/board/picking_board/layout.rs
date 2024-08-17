use bevy::prelude::*;

use crate::game::overlay::board::components::{
    BagGrid, DropBackButtonHolder, DropNextButtonHolder, PickingBag, SelectedSlotID, SlotContainers,
};

pub fn spawn_picking_board(mut commands: Commands) {
    info!("Spawning Picking Board");
    let layout = commands
        .spawn((
            Name::new("Picking Drop Items Layout"),
            PickingBag::default(),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::End,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    padding: UiRect::all(Val::Px(20.)),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let grid = commands
        .spawn((
            Name::new("Eq Grid"),
            BagGrid,
            NodeBundle {
                background_color: Color::linear_rgb(0.5, 0.5, 0.5).into(),
                style: Style {
                    display: Display::Grid,
                    height: Val::Px(400.),
                    max_height: Val::Percent(50.),
                    padding: UiRect {
                        top: Val::Px(10.),
                        left: Val::Px(10.),
                        ..default()
                    },
                    // margin: UiRect::horizontal(Val::Px(3.)),
                    row_gap: Val::Px(60.0),
                    grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                    grid_template_rows: RepeatedGridTrack::flex(5, 1.0),

                    aspect_ratio: Some(1.0),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let bottom_container = commands
        .spawn((
            Name::new("Bottom Container"),
            NodeBundle {
                background_color: Color::linear_rgb(0.7, 0.7, 0.7).into(),
                style: Style {
                    display: Display::Flex,
                    width: Val::Percent(100.),
                    height: Val::Percent(50.),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let button_row = commands
        .spawn((
            Name::new("Button Row"),
            NodeBundle {
                // background_color: Color::BLUE.into(),
                style: Style {
                    height: Val::Px(40.),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let back_button_ui = commands
        .spawn((
            Name::new("Back Button"),
            DropBackButtonHolder,
            NodeBundle {
                background_color: Color::WHITE.into(),
                border_color: Color::BLACK.into(),
                style: Style {
                    width: Val::Percent(33.),
                    border: UiRect::all(Val::Px(4.)),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let next_button_ui: Entity = commands
        .spawn((
            Name::new("Next Button"),
            DropNextButtonHolder,
            NodeBundle {
                background_color: Color::WHITE.into(),
                border_color: Color::BLACK.into(),
                style: Style {
                    width: Val::Percent(33.),
                    border: UiRect::all(Val::Px(4.)),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let selected_slots = commands
        .spawn((
            Name::new("Selected Slots"),
            SlotContainers(Vec::new()),
            NodeBundle {
                // background_color: Color::linear_rgb(0.5, 0.5, 0.5).into(),
                style: Style {
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let first_slot = commands
        .spawn((
            Name::new("1 Slot"),
            SelectedSlotID(0),
            NodeBundle {
                background_color: Color::linear_rgb(0.7, 0.7, 0.7).into(),
                border_color: Color::WHITE.into(),
                style: Style {
                    height: Val::Percent(100.),
                    width: Val::Percent(33.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_content: AlignContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::right(Val::Px(1.)),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let second_slot = commands
        .spawn((
            Name::new("2 Slot"),
            SelectedSlotID(1),
            NodeBundle {
                background_color: Color::linear_rgb(0.7, 0.7, 0.7).into(),
                border_color: Color::WHITE.into(),
                style: Style {
                    height: Val::Percent(100.),
                    width: Val::Percent(33.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_content: AlignContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::right(Val::Px(1.)),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let third_slot = commands
        .spawn((
            Name::new("3 Slot"),
            SelectedSlotID(2),
            NodeBundle {
                background_color: Color::linear_rgb(0.7, 0.7, 0.7).into(),
                style: Style {
                    height: Val::Percent(100.),
                    width: Val::Percent(33.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_content: AlignContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let empty_slot = commands
        .spawn(TextBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "Empty".to_string(),
                    style: TextStyle {
                        color: Color::WHITE,
                        ..Default::default()
                    },
                }],
                ..Default::default()
            },
            ..default()
        })
        .id();

    let empty_slot1 = commands
        .spawn(TextBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "Empty".to_string(),
                    style: TextStyle {
                        color: Color::WHITE,
                        ..Default::default()
                    },
                }],
                ..Default::default()
            },
            ..default()
        })
        .id();

    let empty_slot2 = commands
        .spawn(TextBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "Empty".to_string(),
                    style: TextStyle {
                        color: Color::WHITE,
                        ..Default::default()
                    },
                }],
                ..Default::default()
            },
            ..default()
        })
        .id();

    commands
        .entity(layout)
        .push_children(&[grid, bottom_container]);

    commands
        .entity(bottom_container)
        .push_children(&[button_row, selected_slots]);

    commands
        .entity(button_row)
        .push_children(&[back_button_ui, next_button_ui]);

    commands
        .entity(selected_slots)
        .push_children(&[first_slot, second_slot, third_slot]);

    commands.entity(first_slot).add_child(empty_slot);
    commands.entity(second_slot).add_child(empty_slot1);
    commands.entity(third_slot).add_child(empty_slot2);
}
