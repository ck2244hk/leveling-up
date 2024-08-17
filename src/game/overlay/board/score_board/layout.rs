use crate::animation::component::AnimationIndices;
use crate::animation::component::AnimationTimer;
use crate::audio_effects::ButtonClickEffect;
use crate::game::overlay::board::components::*;
use crate::game::overlay::board::styles::*;

use crate::game::GameUIFrame;
use crate::model::sub::DropEquipment;
use crate::model::ItemType;
use crate::preload::images::EquipmentHandles;
use crate::preload::sprites::HeroAction;
use crate::preload::sprites::HeroActionHandles;
use crate::preload::sprites::HeroActionTextureAtLasHandles;
use bevy::prelude::*;

use super::SpawnScoreBoardEvent;

pub fn spawn_score_board(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    layout_query: Query<Entity, With<GameUIFrame>>,
    mut spawn_event: EventReader<SpawnScoreBoardEvent>,
    hero_sprite: Res<HeroActionHandles>,
    eq_pic_assets: Res<EquipmentHandles>,
    hero_textureatlas: Res<HeroActionTextureAtLasHandles>,
) {
    for ev in spawn_event.read() {
        let layout_entity = layout_query.get_single().expect("No Layout in Game");

        // for hero_class in hero_query.iter() {
        info!("Status Board Spawned");

        // let image_handler: Handle<Image> = match hero_class {
        //     HeroClass::Paladin => asset_server.load("images/paladin.png"),
        //     HeroClass::Warrior => asset_server.load("images/paladin.png"),
        //     HeroClass::Rogue => asset_server.load("images/paladin.png"),
        // };

        build_score_board(
            &mut commands,
            &asset_server,
            layout_entity,
            &hero_sprite,
            &ev.0,
            &eq_pic_assets,
            &hero_textureatlas,
        );
        // }
    }
}

pub fn despawn_score_board(
    mut commands: Commands,
    score_board_query: Query<Entity, With<ScoreBoard>>,
    layout_query: Query<Entity, With<GameUIFrame>>,
) {
    let layout_entity = layout_query.get_single().unwrap();
    for score_board_entity in score_board_query.iter() {
        commands
            .entity(layout_entity)
            .remove_children(&[score_board_entity]);
        commands.entity(score_board_entity).despawn_recursive();
    }
}

fn build_score_board(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    layout_entity: Entity,
    hero_sprite: &Res<HeroActionHandles>,
    drops: &Vec<DropEquipment>,
    eq_pic_assets: &Res<EquipmentHandles>,
    hero_textureatlas: &Res<HeroActionTextureAtLasHandles>,
) {
    let board = commands
        .spawn((
            Name::new("ScoringBoard"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Auto,
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    padding: UiRect {
                        left: Val::Px(10.),
                        right: Val::Px(10.),
                        top: Val::Px(20.),
                        bottom: Val::Px(20.),
                    },
                    margin: UiRect {
                        left: Val::Px(10.),
                        right: Val::Px(10.),
                        top: Val::Px(10.),
                        bottom: Val::Px(10.),
                    },
                    border: UiRect::all(Val::Px(2.0)),
                    ..Style::DEFAULT
                },
                z_index: ZIndex::Global(10),
                background_color: Color::linear_rgb(0.7, 0.7, 0.7).into(),
                ..default()
            },
            ScoreBoard {},
        ))
        .id();

    let title_text = commands
        .spawn((
            Name::new("Title Text"),
            TextBundle {
                style: Style {
                    width: Val::Auto,
                    height: Val::Auto,
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    justify_items: JustifyItems::Center,
                    margin: UiRect {
                        left: Val::Px(0.),
                        right: Val::Px(0.),
                        top: Val::Px(20.),
                        bottom: Val::Px(0.),
                    },
                    ..Style::DEFAULT
                },
                text: Text {
                    sections: vec![TextSection::new(
                        format!("Round Summary"),
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
        ))
        .id();

    let level_text = commands
        .spawn((
            Name::new("Level Text"),
            TextBundle {
                style: Style {
                    width: Val::Auto,
                    height: Val::Auto,
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    justify_items: JustifyItems::Center,
                    margin: UiRect {
                        left: Val::ZERO,
                        right: Val::ZERO,
                        top: Val::Px(20.),
                        bottom: Val::ZERO,
                    },
                    ..Style::DEFAULT
                },
                text: Text {
                    sections: vec![TextSection::new(
                        format!("{}Lv", 0),
                        TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 23.0,
                            color: Color::WHITE,
                        },
                    )],
                    justify: JustifyText::Center,

                    ..default()
                },
                ..default()
            },
            LevelText(0.),
        ))
        .id();

    let row = commands
        .spawn((
            Name::new("Mid Row"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    width: Val::Percent(100.),
                    height: Val::Percent(40.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect {
                        left: Val::Px(4.),
                        right: Val::Px(4.),
                        top: Val::Px(30.),
                        bottom: Val::Px(30.),
                    },
                    ..Style::DEFAULT
                },
                background_color: Color::linear_rgb(0.7, 0.7, 0.7).into(),
                ..default()
            },
            StatusBoardRow,
        ))
        .id();

    let total_experience = commands
        .spawn((
            Name::new("Total Exp"),
            TextBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(30.0),
                    padding: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Px(2.),
                        bottom: Val::Px(2.),
                    },
                    ..Style::DEFAULT
                },
                text: Text {
                    sections: vec![TextSection::new(
                        format!("Exp: {:.0}", 0_f32),
                        TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    )],
                    justify: JustifyText::Center,
                    ..default()
                },
                ..default()
            },
            ExperienceText(0.),
        ))
        .id();

    // let money = commands
    //     .spawn((
    //         Name::new("Available Point"),
    //         TextBundle {
    //             style: Style {
    //                 width: Val::Percent(100.0),
    //                 height: Val::Px(30.0),
    //                 padding: UiRect {
    //                     left: Val::Auto,
    //                     right: Val::Auto,
    //                     top: Val::Px(2.),
    //                     bottom: Val::Px(2.),
    //                 },
    //                 ..Style::DEFAULT
    //             },
    //             text: Text {
    //                 sections: vec![TextSection::new(
    //                     format!("Money: "),
    //                     TextStyle {
    //                         font: asset_server.load("fonts/FiraMono-Medium.ttf"),
    //                         font_size: 25.0,
    //                         color: Color::WHITE,
    //                     },
    //                 )],
    //                 justify: JustifyText::Center,
    //                 ..default()
    //             },
    //             ..default()
    //         },
    //         MoneyText(0.),
    //     ))
    //     .id();

    let bottom_row = commands
        .spawn((
            Name::new("Bottom Row"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    width: Val::Percent(90.0),
                    height: Val::Px(200.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    margin: UiRect::top(Val::Px(2.)),
                    padding: UiRect {
                        left: Val::Px(0.),
                        right: Val::Px(0.),
                        top: Val::Px(2.),
                        bottom: Val::Px(10.),
                    },
                    ..Style::DEFAULT
                },
                background_color: Color::linear_rgb(0.7, 0.7, 0.7).into(),
                ..default()
            },
            StatusBoardRow,
        ))
        .id();

    let shop_button = commands
        .spawn((
            Name::new("Back"),
            ButtonBundle {
                background_color: NORMAL_BUTTON_COLOR.into(),
                style: get_normal_button_style(),
                ..default()
            },
            ButtonClickEffect,
            ShopButton {},
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "Shop".to_string(),
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
                ShopButtonText {},
            ));
        })
        .id();

    let back_button = commands
        .spawn((
            Name::new("Back"),
            ButtonBundle {
                background_color: NORMAL_BUTTON_COLOR.into(),
                style: get_normal_button_style(),
                ..default()
            },
            ButtonClickEffect,
            MenuButton {},
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
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
                },
                MenuButtonText {},
            ));
        })
        .id();

    let hero_picture = commands
        .spawn((
            // replace with image bundle later on
            Name::new("HeroPic"),
            AnimationIndices {
                first: 0,
                last: 3,
                will_repeat: true,
            },
            AnimationTimer::repeat(0.15),
            ImageBundle {
                style: Style {
                    width: Val::Percent(30.0),
                    min_width: Val::Px(150.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    ..Style::DEFAULT
                },
                image: hero_sprite
                    .0
                    .get(&HeroAction::AttackNoMovement)
                    .unwrap()
                    .clone()
                    .into(),
                ..default()
            },
            TextureAtlas {
                layout: hero_textureatlas
                    .0
                    .get(&HeroAction::AttackNoMovement)
                    .unwrap()
                    .clone(),
                index: 0,
            },
            HeroPicture,
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
                    max_width: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
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
                    max_width: Val::Percent(33.),
                    width: Val::Px(999.),
                    min_width: Val::Percent(33.),
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
                    max_width: Val::Percent(33.),
                    width: Val::Px(999.),
                    min_width: Val::Percent(33.),
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
                    max_width: Val::Percent(33.),
                    min_width: Val::Percent(33.),
                    width: Val::Px(999.),
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

    let slots = [first_slot, second_slot, third_slot];

    // let statuses = commands
    //     .spawn((
    //         Name::new("All Status"),
    //         NodeBundle {
    //             style: Style {
    //                 flex_direction: FlexDirection::Column,
    //                 width: Val::Percent(100.0),
    //                 height: Val::Percent(100.0),
    //                 align_items: AlignItems::Center,
    //                 justify_content: JustifyContent::SpaceBetween,
    //                 padding: UiRect {
    //                     left: Val::Px(0.),
    //                     right: Val::Px(0.),
    //                     top: Val::Px(2.),
    //                     bottom: Val::Px(2.),
    //                 },
    //                 ..Style::DEFAULT
    //             },
    //             background_color: Color::linear_rgb(0.7, 0.7, 0.7).into(),
    //             ..default()
    //         },
    //         StatusBoardStatues,
    //     ))
    //     .id();

    // let str_stat = commands
    //     .spawn((
    //         Name::new("Str"),
    //         NodeBundle {
    //             style: Style {
    //                 flex_direction: FlexDirection::Row,
    //                 align_items: AlignItems::Center,
    //                 justify_content: JustifyContent::SpaceBetween,
    //                 padding: UiRect {
    //                     left: Val::Px(0.),
    //                     right: Val::Px(0.),
    //                     top: Val::Px(2.),
    //                     bottom: Val::Px(2.),
    //                 },
    //                 ..Style::DEFAULT
    //             },
    //             background_color: Color::linear_rgb(0.7, 0.7, 0.7).into(),
    //             ..default()
    //         },
    //         StrStateNode,
    //     ))
    //     .id();

    // let str_display = commands
    //     .spawn((
    //         TextBundle {
    //             style: Style {
    //                 padding: UiRect {
    //                     left: Val::Px(0.),
    //                     right: Val::Px(0.),
    //                     top: Val::Px(2.),
    //                     bottom: Val::Px(2.),
    //                 },
    //                 ..Style::DEFAULT
    //             },
    //             text: Text {
    //                 sections: vec![TextSection::new(
    //                     format!("Str: "),
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
    //         StrText {},
    //     ))
    //     .id();

    // let def_stat = commands
    //     .spawn((
    //         Name::new("Def"),
    //         NodeBundle {
    //             style: Style {
    //                 flex_direction: FlexDirection::Row,
    //                 align_items: AlignItems::Center,
    //                 justify_content: JustifyContent::SpaceBetween,
    //                 padding: UiRect {
    //                     left: Val::Px(0.),
    //                     right: Val::Px(0.),
    //                     top: Val::Px(2.),
    //                     bottom: Val::Px(2.),
    //                 },
    //                 ..Style::DEFAULT
    //             },
    //             background_color: Color::linear_rgb(0.7, 0.7, 0.7).into(),
    //             ..default()
    //         },
    //         DefStateNode,
    //     ))
    //     .id();

    // let def_display = commands
    //     .spawn((
    //         TextBundle {
    //             style: Style {
    //                 padding: UiRect {
    //                     left: Val::Px(0.),
    //                     right: Val::Px(0.),
    //                     top: Val::Px(2.),
    //                     bottom: Val::Px(2.),
    //                 },
    //                 ..Style::DEFAULT
    //             },
    //             text: Text {
    //                 sections: vec![TextSection::new(
    //                     format!("Def: "),
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
    //         DefText {},
    //     ))
    //     .id();

    // let agi_stat = commands
    //     .spawn((
    //         Name::new("Agi"),
    //         NodeBundle {
    //             style: Style {
    //                 flex_direction: FlexDirection::Row,
    //                 align_items: AlignItems::Center,
    //                 justify_content: JustifyContent::SpaceBetween,
    //                 padding: UiRect {
    //                     left: Val::Px(0.),
    //                     right: Val::Px(0.),
    //                     top: Val::Px(2.),
    //                     bottom: Val::Px(2.),
    //                 },
    //                 ..Style::DEFAULT
    //             },
    //             background_color: Color::linear_rgb(0.7, 0.7, 0.7).into(),
    //             ..default()
    //         },
    //         AgiStateNode,
    //     ))
    //     .id();

    // let agi_display = commands
    //     .spawn((
    //         TextBundle {
    //             style: Style {
    //                 padding: UiRect {
    //                     left: Val::Px(0.),
    //                     right: Val::Px(0.),
    //                     top: Val::Px(2.),
    //                     bottom: Val::Px(2.),
    //                 },
    //                 ..Style::DEFAULT
    //             },
    //             text: Text {
    //                 sections: vec![TextSection::new(
    //                     format!("Agi: "),
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
    //         AgiText {},
    //     ))
    //     .id();

    commands.entity(layout_entity).add_child(board);

    commands.entity(board).push_children(&[
        title_text,
        level_text,
        row,
        selected_slots,
        total_experience,
        // money,
        bottom_row,
    ]);

    commands.entity(row).push_children(&[hero_picture]);

    commands
        .entity(bottom_row)
        .push_children(&[back_button, shop_button]);

    commands
        .entity(selected_slots)
        .push_children(&[first_slot, second_slot, third_slot]);

    for (index, equipment) in drops.iter().enumerate() {
        if index > 2 {
            continue;
        }

        let slot = slots[index];

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
        let name = commands
            .spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: format!("{}", equipment.name).to_string(),
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
                        value: format!("Lv {}", equipment.level).to_string(),
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

        commands.entity(slot).push_children(&[name, image, lv]);
    }

    // commands
    //     .entity(statuses)
    //     .push_children(&[str_stat, def_stat, agi_stat]);

    // commands.entity(str_stat).push_children(&[str_display]);
    // commands.entity(def_stat).push_children(&[def_display]);
    // commands.entity(agi_stat).push_children(&[agi_display]);
}
