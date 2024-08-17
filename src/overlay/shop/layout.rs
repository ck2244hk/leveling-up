use crate::audio_effects::ButtonClickEffect;
use crate::game::player::{Money, Player};
use crate::model::ItemType;
use crate::overlay::component::*;
use crate::preload::images::{EquipmentHandles, UiImageAsset, UiImageHandles};
use bevy::a11y::accesskit::{NodeBuilder, Role};
use bevy::a11y::AccessibilityNode;
use bevy::prelude::*;

use super::ActiveShopTab;

pub fn spawn_shop(
    mut commands: Commands,
    equipment_handles: Res<EquipmentHandles>,
    ui_image_asset: Res<UiImageHandles>,
    asset_server: Res<AssetServer>,
    player_query: Query<&Money, With<Player>>,
) {
    // let money = player_query.get_single().expect("No Player is spawned");
    let layout = commands
        .spawn((
            Name::new("Shop Layout"),
            NodeBundle {
                background_color: Color::srgb(0.3, 0.3, 0.3).into(),
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Style::DEFAULT
                },
                z_index: ZIndex::Global(11),
                ..default()
            },
            ShopLayout {},
            Shop {},
            SelectedEquipment::default(),
            ActiveShopTab::default(),
        ))
        .id();

    let shop_image = commands
        .spawn((
            Name::new("Shop Pic"),
            ImageBundle {
                style: Style {
                    max_height: Val::Px(200.0),
                    ..default()
                },
                image: UiImage {
                    texture: ui_image_asset
                        .0
                        .get(&UiImageAsset::ShopImage)
                        .expect("Shop Image is not Loaded")
                        .clone(),
                    ..default()
                },
                ..Default::default()
            },
            ShopImage,
        ))
        .id();

    let shop_tabs_row = commands
        .spawn((
            Name::new("Shop Tabs Row"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    ..Style::DEFAULT
                },
                ..default()
            },
        ))
        .id();

    let shop_tabs = commands
        .spawn((
            Name::new("Shop Tabs"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    margin: UiRect {
                        left: Val::Px(3.),
                        right: Val::Px(3.),
                        top: Val::Px(3.),
                        bottom: Val::Px(0.),
                    },
                    ..Style::DEFAULT
                },
                ..default()
            },
        ))
        .id();

    let weapon_tab = commands
        .spawn((
            Name::new("Weapon Tab"),
            ButtonBundle {
                border_color: Color::NONE.into(),
                image: UiImage {
                    texture: equipment_handles
                        .0
                        .get(&ItemType::Weapon)
                        .expect("sword01 png failed to load")
                        .first()
                        .expect("sword01 png failed to load")
                        .clone(),
                    ..default()
                },
                style: Style {
                    border: UiRect::all(Val::Px(5.)),
                    width: Val::Px(64.),
                    height: Val::Px(64.),
                    ..default()
                },
                ..default()
            },
            ButtonClickEffect,
            ShopTab::Weapon,
        ))
        .id();

    let armor_tab = commands
        .spawn((
            Name::new("Armor Tab"),
            ButtonBundle {
                border_color: Color::NONE.into(),
                image: UiImage {
                    texture: equipment_handles[&ItemType::Armor]
                        .get(0)
                        .expect("armor01 png failed to load")
                        .clone(),
                    ..default()
                },
                style: Style {
                    border: UiRect::all(Val::Px(5.)),
                    width: Val::Px(64.),
                    height: Val::Px(64.),
                    ..default()
                },
                ..default()
            },
            ButtonClickEffect,
            ShopTab::Armor,
        ))
        .id();

    let helmet_tab = commands
        .spawn((
            Name::new("Helmet Tab"),
            ButtonBundle {
                border_color: Color::NONE.into(),
                image: UiImage {
                    texture: equipment_handles[&ItemType::Helmet]
                        .first()
                        .expect("Helmet01 png failed to load")
                        .clone(),
                    ..default()
                },
                style: Style {
                    border: UiRect::all(Val::Px(5.)),
                    width: Val::Px(64.),
                    height: Val::Px(64.),
                    ..default()
                },
                ..default()
            },
            ButtonClickEffect,
            ShopTab::Helmet,
        ))
        .id();

    let shoes_tab = commands
        .spawn((
            Name::new("Shoes Tab"),
            ButtonBundle {
                border_color: Color::NONE.into(),
                image: UiImage {
                    texture: equipment_handles[&ItemType::Shoes]
                        .get(0)
                        .unwrap_or(equipment_handles[&ItemType::Weapon].get(0).unwrap())
                        .clone(),
                    ..default()
                },
                style: Style {
                    border: UiRect::all(Val::Px(5.)),
                    width: Val::Px(64.),
                    height: Val::Px(64.),
                    ..default()
                },
                ..default()
            },
            ButtonClickEffect,
            ShopTab::Shoes,
        ))
        .id();

    let content_row = commands
        .spawn((
            Name::new("Shop Row"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    height: Val::Percent(100.),
                    ..Style::DEFAULT
                },
                ..default()
            },
        ))
        .id();

    let scrolling_container = commands
        .spawn((
            Name::new("Scrolling Container"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::Stretch,
                    width: Val::Percent(50.),
                    padding: UiRect::bottom(Val::Px(20.)),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let eq_list_title_row = commands
        .spawn((
            Name::new("Eq List Title"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    width: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
            ScrollingListTitleRow,
        ))
        .id();

    let scrolling_overflow = commands
        .spawn((
            Name::new("Scrolling Overflow Container"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::Stretch,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    overflow: Overflow::clip_y(),
                    ..default()
                },
                z_index: ZIndex::Global(99),
                ..default()
            },
            Interaction::default(),
            ScrollingListContainer,
        ))
        .id();

    let scrollable = commands
        .spawn((
            Name::new("Scrollable"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,

                    ..default()
                },
                ..default()
            },
            ScrollingList::default(),
            Interaction::default(),
            AccessibilityNode(NodeBuilder::new(Role::List)),
        ))
        .id();

    let eq_name_title = commands
        .spawn((
            Name::new("Eq name title"),
            TextBundle {
                style: Style {
                    width: Val::Percent(30.),
                    min_width: Val::Px(40.),
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
                            value: String::from("Name"),
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
            },
        ))
        .id();

    let eq_owned_title = commands
        .spawn((
            Name::new("Eq owned title"),
            NodeBundle {
                style: Style {
                    width: Val::Percent(20.),
                    min_width: Val::Px(20.),
                    max_width: Val::Px(200.),
                    margin: UiRect {
                        left: Val::Px(10.),
                        right: Val::Px(10.),
                        top: Val::ZERO,
                        bottom: Val::ZERO,
                    },
                    ..default()
                },
                // text: Text {
                //     sections: {
                //         vec![TextSection {
                //             value: String::from("Owned"),
                //             style: TextStyle {
                //                 font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                //                 font_size: 14.0,
                //                 color: Color::WHITE,
                //             },
                //         }]
                //     },
                //     ..default()
                // },
                ..Default::default()
            },
        ))
        .id();

    let eq_spacer = commands
        .spawn((
            Name::new("Eq button spacer"),
            NodeBundle {
                style: Style {
                    width: Val::Percent(30.),
                    min_width: Val::Px(80.),
                    max_width: Val::Px(200.),
                    margin: UiRect {
                        left: Val::Px(10.),
                        right: Val::Px(10.),
                        top: Val::ZERO,
                        bottom: Val::ZERO,
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let comparison = commands
        .spawn((
            Name::new("Comparison"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Start,
                    width: Val::Px(200.),
                    ..Style::DEFAULT
                },
                ..default()
            },
        ))
        .id();

    let selected_title = commands
        .spawn((
            Name::new("Tag"),
            TextBundle {
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
                            value: String::from("Selected"),
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
            },
        ))
        .id();

    let selected_eq = commands
        .spawn((
            Name::new("Selected"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    min_height: Val::Px(150.),
                    ..Style::DEFAULT
                },
                ..default()
            },
        ))
        .id();

    let selected_pic_attr = commands
        .spawn((
            Name::new("Pic and Attr"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    ..Style::DEFAULT
                },
                ..default()
            },
        ))
        .id();

    let selected_pic = commands
        .spawn((
            Name::new("Selected Pic"),
            ImageBundle {
                background_color: Color::NONE.into(),
                style: Style {
                    width: Val::Px(64.),
                    height: Val::Px(64.),
                    ..default()
                },
                image: UiImage::default(),
                ..default()
            },
            SelectedEqPic,
        ))
        .id();

    let selected_attr = commands
        .spawn((
            Name::new("Selected Attr"),
            TextBundle {
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
                            value: String::from(""),
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
            },
            SelectedEqAttr,
        ))
        .id();

    let selected_des = commands
        .spawn((
            Name::new("Selected Des"),
            TextBundle {
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
                            value: String::from(""),
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
            },
            SelectedEqDes,
        ))
        .id();

    let current_eq = commands
        .spawn((
            Name::new("Current"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    min_height: Val::Px(150.),
                    ..Style::DEFAULT
                },
                ..default()
            },
        ))
        .id();

    let current_title = commands
        .spawn((
            Name::new("Tag"),
            TextBundle {
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
                            value: String::from("Current"),
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
            },
        ))
        .id();

    let current_pic_attr = commands
        .spawn((
            Name::new("Pic and Attr"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    ..Style::DEFAULT
                },
                ..default()
            },
        ))
        .id();

    let current_pic = commands
        .spawn((
            Name::new("Current Pic"),
            ImageBundle {
                background_color: Color::NONE.into(),
                style: Style {
                    width: Val::Px(64.),
                    height: Val::Px(64.),
                    ..default()
                },
                image: UiImage::default(),
                ..default()
            },
            CurrentEqPic,
        ))
        .id();

    let current_attr = commands
        .spawn((
            Name::new("Current Attr"),
            TextBundle {
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
                            value: String::from(""),
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
            },
            CurrentEqAttr,
        ))
        .id();

    let current_des = commands
        .spawn((
            Name::new("Current Des"),
            TextBundle {
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
                            value: String::from(""),
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
            },
            CurrentEqDes,
        ))
        .id();

    let bottom_row = commands
        .spawn((
            Name::new("Bottom Row"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    height: Val::Px(50.),
                    margin: UiRect {
                        left: Val::ZERO,
                        right: Val::ZERO,
                        top: Val::Px(10.),
                        bottom: Val::Px(30.),
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let back_button = commands
        .spawn((
            Name::new("Back Button"),
            ButtonBundle {
                // background_color: NORMAL_BUTTON_COLOR.into(),
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Px(30.),
                    margin: UiRect::horizontal(Val::Px(50.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            BackButton,
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
                            value: String::from("Back"),
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

    // let total_money = commands
    //     .spawn((
    //         Name::new("TotalMoney Container"),
    //         NodeBundle {
    //             style: Style {
    //                 width: Val::Percent(50.),
    //                 justify_content: JustifyContent::Center,
    //                 align_items: AlignItems::Center,
    //                 ..default()
    //             },
    //             ..default()
    //         },
    //     ))
    //     .with_children(|builder| {
    //         builder.spawn((
    //             Name::new("Total Money"),
    //             TextBundle {
    //                 style: Style {
    //                     justify_content: JustifyContent::Center,
    //                     align_self: AlignSelf::Center,
    //                     width: Val::Percent(50.),
    //                     margin: UiRect {
    //                         left: Val::Auto,
    //                         right: Val::Auto,
    //                         top: Val::ZERO,
    //                         bottom: Val::ZERO,
    //                     },
    //                     ..default()
    //                 },
    //                 text: Text {
    //                     sections: {
    //                         vec![TextSection {
    //                             value: format!("Money: null"),
    //                             style: TextStyle {
    //                                 font: asset_server.load("fonts/FiraMono-Medium.ttf"),
    //                                 font_size: 30.0,
    //                                 color: Color::WHITE,
    //                             },
    //                         }]
    //                     },
    //                     justify: JustifyText::Center,
    //                     ..default()
    //                 },
    //                 ..Default::default()
    //             },
    //             TotalMoney,
    //         ));
    //     })
    //     .id();

    commands
        .entity(layout)
        .push_children(&[shop_image, shop_tabs_row, content_row, bottom_row]);

    commands.entity(shop_tabs_row).push_children(&[shop_tabs]);

    commands
        .entity(shop_tabs)
        .push_children(&[weapon_tab, armor_tab, helmet_tab, shoes_tab]);

    commands
        .entity(content_row)
        .push_children(&[comparison, scrolling_container]);

    commands
        .entity(scrolling_container)
        .push_children(&[eq_list_title_row, scrolling_overflow]);

    commands.entity(scrolling_overflow).add_child(scrollable);

    commands
        .entity(eq_list_title_row)
        .push_children(&[eq_name_title, eq_owned_title, eq_spacer]);

    commands
        .entity(comparison)
        .push_children(&[selected_eq, current_eq]);

    commands
        .entity(selected_eq)
        .push_children(&[selected_title, selected_pic_attr, selected_des]);

    commands
        .entity(selected_pic_attr)
        .push_children(&[selected_pic, selected_attr]);

    commands
        .entity(current_eq)
        .push_children(&[current_title, current_pic_attr, current_des]);

    commands
        .entity(current_pic_attr)
        .push_children(&[current_pic, current_attr]);

    commands.entity(bottom_row).push_children(&[back_button]);
}

pub fn despawn_shop(mut commands: Commands, layout_query: Query<Entity, With<ShopLayout>>) {
    for layout_entity in layout_query.iter() {
        commands.entity(layout_entity).despawn_recursive();
    }
}
