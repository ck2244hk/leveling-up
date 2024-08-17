use bevy::prelude::*;

use crate::{
    audio_effects::ButtonClickEffect,
    game::{
        character::component::{Bag, Hero},
        overlay::board::components::{
            BagGrid, DropBackButton, DropBackButtonHolder, DropFinishButton, DropNextButton,
            DropNextButtonHolder, InteractiveSlot, NotSelectedEq, SelectedSlotID, SingleDropEqGrid,
            SlotContainers,
        },
    },
    model::{sub::DropEquipment, ItemType},
    preload::images::EquipmentHandles,
};

pub fn update_drop_bag_grid(
    mut commands: Commands,
    ui_query: Query<Entity, Added<BagGrid>>,
    hero_query: Query<&Bag, With<Hero>>,
    eq_pic_assets: Res<EquipmentHandles>,
) {
    if let Ok(ui) = ui_query.get_single() {
        let bag = hero_query.get_single().expect("No Hero Spawned");

        for equipment in bag.0.iter() {
            let source = eq_pic_assets
                .0
                .get(&equipment.item_type)
                .unwrap_or(eq_pic_assets.0.get(&ItemType::Weapon).unwrap())
                .get(equipment.id as usize)
                .unwrap_or(
                    eq_pic_assets
                        .0
                        .get(&ItemType::Weapon)
                        .unwrap()
                        .get(1)
                        .unwrap(),
                );

            let parent = commands
                .spawn((
                    NotSelectedEq {
                        image: source.clone(),
                        eq: equipment.clone(),
                        empty: false,
                    },
                    NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            width: Val::Px(64.),
                            height: Val::Px(64.),
                            padding: UiRect::all(Val::Px(3.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::BLACK),
                        ..default()
                    },
                ))
                .id();

            let image = commands
                .spawn((
                    ButtonClickEffect,
                    equipment.clone(),
                    SingleDropEqGrid,
                    ButtonBundle {
                        style: Style {
                            border: UiRect::all(Val::Px(1.0)),
                            ..Default::default()
                        },
                        image: UiImage::new(source.clone()),
                        ..default()
                    },
                ))
                .id();

            commands.entity(ui).add_child(parent);
            commands.entity(parent).add_child(image);
        }
    }
}

pub fn update_buttons(
    mut commands: Commands,
    back_button_query: Query<Entity, With<DropBackButtonHolder>>,
    next_button_query: Query<Entity, With<DropNextButtonHolder>>,
    container_query: Query<&SlotContainers, With<SlotContainers>>,
    hero_query: Query<&Bag, With<Hero>>,
) {
    if let Ok(container) = container_query.get_single() {
        let back_button_ui = back_button_query.single();
        let next_button_ui = next_button_query.single();
        let bag = hero_query.get_single().expect("No Hero Spawned");
        let num_of_items = bag.0.len();
        let slot_num = container.0.len();

        commands.entity(back_button_ui).despawn_descendants();
        commands.entity(next_button_ui).despawn_descendants();

        // The first reps if the container slot is filled
        // Second reps if the number of total equipment is exhausted
        // third reps if slots exhausted

        // info!(
        //     "If container slot filled: {}, if equipments exhausted: {}, if slots exhausted: {}",
        //     container.0.iter().len() == slot_num,
        //     slot_num == num_of_items,
        //     slot_num
        // );
        match (
            container.0.iter().any(|a| a.check_if_select()),
            slot_num == num_of_items,
            slot_num,
        ) {
            (true, false, 0) => {
                // ready for next slot and first slot
                let next_button = build_next_button(&mut commands);
                commands.entity(next_button_ui).add_child(next_button);
            }
            (_, true, 0) => {
                // fill the ready to submit
                let next_button = build_finish_button(&mut commands);
                commands.entity(next_button_ui).add_child(next_button);
            }
            (_, _, 3) | (_, true, _) => {
                // fill the ready to submit
                let next_button = build_finish_button(&mut commands);
                commands.entity(next_button_ui).add_child(next_button);
                let back_button = build_back_button(&mut commands);
                commands.entity(back_button_ui).add_child(back_button);
            }
            (false, false, 0) => {
                // Does not have to display anything
            }
            (false, _, _) => {
                // wait for to be filled
                let back_button = build_back_button(&mut commands);
                commands.entity(back_button_ui).add_child(back_button);
            }
            _ => {
                let next_button = build_next_button(&mut commands);
                commands.entity(next_button_ui).add_child(next_button);
                let back_button = build_back_button(&mut commands);
                commands.entity(back_button_ui).add_child(back_button);
            }
        }
    }
}

fn build_next_button(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Name::new("Next"),
            ButtonBundle {
                background_color: Color::linear_rgb(0.5, 0.5, 0.5).into(),
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Px(30.0),
                    ..default()
                },
                ..default()
            },
            ButtonClickEffect,
            DropNextButton {},
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Next".to_string(),
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
        .id()
}

fn build_finish_button(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Name::new("Finish"),
            ButtonBundle {
                background_color: Color::BLACK.into(),
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Px(30.0),
                    ..default()
                },
                ..default()
            },
            ButtonClickEffect,
            DropFinishButton {},
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Finished".to_string(),
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
        .id()
}

fn build_back_button(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Name::new("Back"),
            ButtonBundle {
                background_color: Color::linear_rgb(0.5, 0.5, 0.5).into(),
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Px(30.0),
                    ..default()
                },
                ..default()
            },
            ButtonClickEffect,
            DropBackButton {},
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Back".to_string(),
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
        .id()
}

pub fn update_grid_cell_with_containers(
    container_query: Query<&SlotContainers, Changed<SlotContainers>>,
    mut grid_query: Query<(&mut Visibility, &DropEquipment), With<SingleDropEqGrid>>,
) {
    if let Ok(container) = container_query.get_single() {
        // show all
        // let _ = grid_query
        //     .iter_mut()
        //     .map(|(mut visibility, _)| visibility.set(Box::new(Visibility::Inherited)));
        for (mut visibility, _) in grid_query.iter_mut() {
            let _ = visibility.set(Box::new(Visibility::Inherited));
        }

        for item in container.drop_list() {
            let Some((mut visibility, _)) = grid_query.iter_mut().find(|(_, eq)| item == **eq)
            else {
                continue;
            };

            let _ = visibility.set(Box::new(Visibility::Hidden));
        }
    }
}

pub fn update_slots_with_containers(
    mut commands: Commands,
    container_query: Query<&mut SlotContainers, Changed<SlotContainers>>,
    slots_query: Query<(&SelectedSlotID, Entity)>,
    eq_pic_assets: Res<EquipmentHandles>,
) {
    if let Ok(container) = container_query.get_single() {
        info!("{:?} in continer", container.0);
        // unload all picture first
        for (_, slot_entity) in slots_query.iter() {
            commands.entity(slot_entity).despawn_descendants();
        }
        for (index, item_slot) in container.0.iter().enumerate() {
            let item = match item_slot {
                InteractiveSlot::Selected(item) => item,
                InteractiveSlot::Confirmed(item) => item,
            };

            let source = eq_pic_assets
                .0
                .get(&item.item_type)
                .unwrap_or(eq_pic_assets.0.get(&ItemType::Weapon).unwrap())
                .get(item.id as usize)
                .unwrap_or(
                    eq_pic_assets
                        .0
                        .get(&ItemType::Weapon)
                        .unwrap()
                        .get(1)
                        .unwrap(),
                );

            if let Some((_, slot_entity)) = slots_query.iter().find(|(id, _)| id.0 == index) {
                let name = commands
                    .spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: format!("{}", item.name).to_string(),
                                style: TextStyle {
                                    color: Color::WHITE,
                                    font_size: 20.,
                                    ..default()
                                },
                            }],
                            justify: JustifyText::Center,
                            ..Default::default()
                        },
                        ..default()
                    })
                    .id();

                let image = commands
                    .spawn(ImageBundle {
                        image: UiImage::new(source.clone()),
                        ..default()
                    })
                    .id();

                let lv = commands
                    .spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: format!("Lv {}", item.level).to_string(),
                                style: TextStyle {
                                    font_size: 15.,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            }],
                            justify: JustifyText::Center,
                            ..Default::default()
                        },
                        ..default()
                    })
                    .id();

                commands
                    .entity(slot_entity)
                    .push_children(&[name, image, lv]);
                commands.entity(slot_entity).insert(item.clone());
            }
        }
    }
}
