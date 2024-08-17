use bevy::prelude::*;

use crate::audio_effects::ButtonClickEffect;
use crate::game::player::{EquipmentBelt, Money, Player, Storage};
use crate::model::armor::ArmorData;
use crate::model::helmet::HelmetData;
use crate::model::player::PlayerData;
use crate::model::shoes::ShoesData;
use crate::model::weapon::WeaponData;
use crate::model::{EquipmentData, ItemType};
use crate::overlay::component::*;
use crate::overlay::styles::get_buy_button_style;
use crate::preload::data::PlayerPreloadHandler;
use crate::preload::fonts::{FontAsset, FontHandles};
use crate::preload::images::EquipmentHandles;

pub fn spawn_eq_list(
    mut commands: Commands,
    shop_query: Query<&ActiveShopTab, Changed<ActiveShopTab>>,
    scrolling_query: Query<Entity, With<ScrollingList>>,
    weapon_data: Res<Assets<WeaponData>>,
    armor_data: Res<Assets<ArmorData>>,
    helmet_data: Res<Assets<HelmetData>>,
    shoes_data: Res<Assets<ShoesData>>,
    player_query: Query<(&EquipmentBelt, &Storage)>,
    font_assets: Res<FontHandles>,
) {
    if let Ok(current_tab) = shop_query.get_single() {
        let scrollable = scrolling_query
            .get_single()
            .expect("No Scrollable been spawned");

        let (player_belt, player_storage) = player_query
            .get_single()
            .expect("Player Save is not loaded");

        // let eq_list: Vec<(&Equipment, Option<&Weapon>, Option<&Armor>, &Name, &Cost)> =
        //     match current_tab.0 {
        //         ShopTab::Weapon => equipment_query
        //             .iter()
        //             .filter(|(_, weapon, armor, _, _)| weapon.is_some() && armor.is_none())
        //             .collect(),
        //         ShopTab::Armor => equipment_query
        //             .iter()
        //             .filter(|(_, weapon, armor, _, _)| weapon.is_none() && armor.is_some())
        //             .collect(),
        //     };

        let builder = EquipListBuilder::new(
            font_assets,
            weapon_data,
            armor_data,
            helmet_data,
            shoes_data,
            scrollable,
            player_belt,
            player_storage,
        );
        builder.clean_up_container(&mut commands);

        match current_tab.0 {
            ShopTab::Weapon => builder.build_weapon(&mut commands),
            ShopTab::Armor => builder.build_armor(&mut commands),
            ShopTab::Helmet => builder.build_helmet(&mut commands),
            ShopTab::Shoes => builder.build_shoes(&mut commands),
        };

        // for (_, record) in weapon_data.iter() {}

        // for (eq, _, _, name, cost) in eq_list.iter() {
        //     let is_equired = eq.get() == equired_weapon.0 || eq.get() == equired_armor.0;
        //     let row = build_eq_list(
        //         name,
        //         player_owned,
        //         eq,
        //         cost,
        //         &mut commands,
        //         &asset_server,
        //         is_equired,
        //     );
        //     commands.entity(scrollable).add_child(row);
        // }
    }
}

pub fn update_current_equired_pic(
    shop_query: Query<&ActiveShopTab, Changed<ActiveShopTab>>,
    mut pic_query: Query<(&mut UiImage, &mut BackgroundColor), With<CurrentEqPic>>,
    player_query: Query<&EquipmentBelt, With<Player>>,
    equipment_asset: Res<EquipmentHandles>,
) {
    if let Ok(current_tab) = shop_query.get_single() {
        let belt = player_query.single();

        let (mut image, mut background_color) = pic_query
            .get_single_mut()
            .expect("No Current Equired Pic spawned");

        match current_tab.0 {
            ShopTab::Weapon => {
                if let Some(item) = &belt.weapon {
                    let source = equipment_asset
                        .0
                        .get(&ItemType::Weapon)
                        .unwrap_or(equipment_asset.0.get(&ItemType::Weapon).unwrap())
                        .get(item.id() as usize)
                        .unwrap_or(
                            equipment_asset
                                .0
                                .get(&ItemType::Weapon)
                                .unwrap()
                                .get(1)
                                .unwrap(),
                        );

                    image.texture = source.clone();
                    *background_color = Color::WHITE.into();
                } else {
                    *image = UiImage::default();
                    *background_color = Color::NONE.into();
                }
            }

            ShopTab::Armor => {
                if let Some(item) = &belt.armor {
                    let source = equipment_asset
                        .0
                        .get(&ItemType::Armor)
                        .unwrap_or(equipment_asset.0.get(&ItemType::Weapon).unwrap())
                        .get(item.id() as usize)
                        .unwrap_or(
                            equipment_asset
                                .0
                                .get(&ItemType::Weapon)
                                .unwrap()
                                .get(1)
                                .unwrap(),
                        );

                    image.texture = source.clone();
                    *background_color = Color::WHITE.into();
                } else {
                    *image = UiImage::default();
                    *background_color = Color::NONE.into();
                }
            }

            ShopTab::Helmet => {
                if let Some(item) = &belt.helmet {
                    let source = equipment_asset
                        .0
                        .get(&ItemType::Helmet)
                        .unwrap_or(equipment_asset.0.get(&ItemType::Weapon).unwrap())
                        .get(item.id() as usize)
                        .unwrap_or(
                            equipment_asset
                                .0
                                .get(&ItemType::Weapon)
                                .unwrap()
                                .get(1)
                                .unwrap(),
                        );

                    image.texture = source.clone();
                    *background_color = Color::WHITE.into();
                } else {
                    *image = UiImage::default();
                    *background_color = Color::NONE.into();
                }
            }

            ShopTab::Shoes => {
                if let Some(item) = &belt.shoes {
                    let source = equipment_asset
                        .0
                        .get(&ItemType::Shoes)
                        .unwrap_or(equipment_asset.0.get(&ItemType::Weapon).unwrap())
                        .get(item.id() as usize)
                        .unwrap_or(
                            equipment_asset
                                .0
                                .get(&ItemType::Weapon)
                                .unwrap()
                                .get(1)
                                .unwrap(),
                        );

                    image.texture = source.clone();
                    *background_color = Color::WHITE.into();
                } else {
                    *image = UiImage::default();
                    *background_color = Color::NONE.into();
                }
            }
        }
    }
}

pub fn update_current_equired_attr(
    shop_query: Query<&ActiveShopTab, Changed<ActiveShopTab>>,
    mut text_query: Query<&mut Text, With<CurrentEqAttr>>,
    player_query: Query<&EquipmentBelt, With<Player>>,
) {
    if let Ok(current_tab) = shop_query.get_single() {
        let belt = player_query.single();

        let mut text = text_query
            .get_single_mut()
            .expect("No Current Equired Attribute spawned");

        match current_tab.0 {
            ShopTab::Weapon => {
                if let Some(eq) = &belt.weapon {
                    text.sections[0].value = format!("Attack: {}", eq.attack());
                } else {
                    text.sections[0].value = format!("");
                }
            }

            ShopTab::Armor => {
                if let Some(eq) = &belt.armor {
                    text.sections[0].value = format!("Defend: {}", eq.defense());
                } else {
                    text.sections[0].value = format!("");
                }
            }

            ShopTab::Helmet => {
                if let Some(eq) = &belt.helmet {
                    text.sections[0].value = format!("Defend: {}", eq.defense());
                } else {
                    text.sections[0].value = format!("");
                }
            }

            ShopTab::Shoes => {
                if let Some(eq) = &belt.shoes {
                    text.sections[0].value = format!("Defend: {}", eq.defense());
                } else {
                    text.sections[0].value = format!("");
                }
            }
        }
    }
}

pub fn update_current_equired_description(
    shop_query: Query<&ActiveShopTab, Changed<ActiveShopTab>>,
    text_query: Query<&mut Text, With<CurrentEqDes>>,
    player_query: Res<Assets<PlayerData>>,
    player_id: Res<PlayerPreloadHandler>,
    weapon_query: Res<Assets<WeaponData>>,
    armor_query: Res<Assets<ArmorData>>,
) {
    // if let Ok(current_tab) = shop_query.get_single() {
    //     let mut text = text_query
    //         .get_single_mut()
    //         .expect("No Current Equired Description spawned");
    //     let player_data = player_query
    //         .get(player_id.0.id())
    //         .expect("Fail to load player data");

    //     match current_tab.0 {
    //         ShopTab::Weapon => {
    //             if let Some((_, weapon_data)) = weapon_query
    //                 .iter()
    //                 .find(|(_, record)| player_data.eq_weapon.is_some_and(|x| record.id == x.id))
    //             {
    //                 text.sections[0].value = format!("{}", weapon_data.description);
    //             } else {
    //                 text.sections[0].value = format!("");
    //             }
    //         }

    //         ShopTab::Armor => {
    //             if let Some((_, armor_data)) = armor_query
    //                 .iter()
    //                 .find(|(_, record)| player_data.eq_armor.is_some_and(|x| record.id == x.id))
    //             {
    //                 text.sections[0].value = format!("{}", armor_data.description);
    //             } else {
    //                 text.sections[0].value = format!("");
    //             }
    //         }
    //     }
    // }
}

pub fn update_selected_equip_pic(
    shop_query: Query<&SelectedEquipment, Changed<SelectedEquipment>>,
    mut pic_query: Query<(&mut UiImage, &mut BackgroundColor), With<SelectedEqPic>>,
    equipment_asset: Res<EquipmentHandles>,
) {
    if let Ok(selected) = shop_query.get_single() {
        let (mut image, mut background_color) = pic_query
            .get_single_mut()
            .expect("No Selected Equired Pic spawned");
        if let Some(item) = &selected.0 {
            let source = equipment_asset
                .0
                .get(&item.item_type)
                .unwrap_or(equipment_asset.0.get(&ItemType::Weapon).unwrap())
                .get(item.id as usize)
                .unwrap_or(
                    equipment_asset
                        .0
                        .get(&ItemType::Weapon)
                        .unwrap()
                        .get(1)
                        .unwrap(),
                );
            image.texture = source.clone();
            *background_color = Color::WHITE.into();
        } else {
            image
                .set(Box::new(UiImage::default()))
                .expect("Cannot set image");
            *background_color = Color::NONE.into();
        }
    }
}

pub fn update_selected_equip_attr(
    shop_query: Query<&SelectedEquipment, Changed<SelectedEquipment>>,
    mut text_query: Query<&mut Text, With<SelectedEqAttr>>,
    player_query: Query<&mut EquipmentBelt, With<Player>>,
) {
    if let Ok(selected) = shop_query.get_single() {
        if let Some(item) = &selected.0 {
            let belt = player_query.single();

            let mut text = text_query
                .get_single_mut()
                .expect("No Selected text spawned");

            match item.item_type {
                ItemType::Weapon => {
                    if let Some(eq_item) = &belt.weapon {
                        text.sections[0].value = format!(
                            "Attack: {} ({:+.0})",
                            item.weapon(),
                            item.weapon() - eq_item.attack()
                        );
                    } else {
                        text.sections[0].value = format!("Attack: {}", item.weapon());
                    }
                }
                ItemType::Armor => {
                    if let Some(eq_item) = &belt.armor {
                        text.sections[0].value = format!(
                            "Defend: {} ({:+.0})",
                            item.armor(),
                            item.armor() - eq_item.defense()
                        );
                    } else {
                        text.sections[0].value = format!("Defend: {}", item.armor());
                    }
                }
                ItemType::Helmet => {
                    if let Some(eq_item) = &belt.helmet {
                        text.sections[0].value = format!(
                            "Defend: {} ({:+.0})",
                            item.armor(),
                            item.armor() - eq_item.defense()
                        );
                    } else {
                        text.sections[0].value = format!("Defend: {}", item.armor());
                    }
                }
                ItemType::Shoes => {
                    if let Some(eq_item) = &belt.shoes {
                        text.sections[0].value = format!(
                            "Defend: {} ({:+.0})",
                            item.armor(),
                            item.armor() - eq_item.defense()
                        );
                    } else {
                        text.sections[0].value = format!("Defend: {}", item.armor());
                    }
                }
                _ => (),
            }
        } else {
            let mut text = text_query
                .get_single_mut()
                .expect("No Selected text spawned");

            text.sections[0].value = format!("");
        }
    }
}

pub fn update_selected_equip_description(
    shop_query: Query<&SelectedEquipment, Changed<SelectedEquipment>>,
    text_query: Query<&mut Text, With<SelectedEqDes>>,
) {
    // if let Ok(selected) = shop_query.get_single() {
    //     let mut text = text_query
    //         .get_single_mut()
    //         .expect("No Selected Description spawned");

    //     if let Some((_, description)) = equipment_query
    //         .iter()
    //         .find(|(eq, _)| eq.get() == selected.0)
    //     {
    //         text.sections[0].value = description.get();
    //     } else {
    //         let mut text = text_query
    //             .get_single_mut()
    //             .expect("No Selected text spawned");

    //         text.sections[0].value = format!("");
    //     }
    // }
}

pub fn update_selected_row_color(
    button_query: Query<(&mut BorderColor, &EqipmentRow), With<EqipmentRow>>,
    shop_query: Query<&SelectedEquipment, (With<Shop>, Changed<SelectedEquipment>)>,
) {
    // if let Ok(selected) = shop_query.get_single() {
    //     for (mut border_color, eq) in button_query.iter_mut() {
    //         if eq.0 == selected.0 {
    //             *border_color = Color::INDIGO.into();
    //         } else {
    //             *border_color = Color::NONE.into();
    //         }
    //     }
    // }
}

pub fn update_activated_tab_color(
    mut button_query: Query<(&mut BorderColor, &ShopTab), With<ShopTab>>,
    shop_query: Query<&ActiveShopTab, (With<Shop>, Changed<ActiveShopTab>)>,
) {
    if let Ok(selected) = shop_query.get_single() {
        for (mut border_color, tab) in button_query.iter_mut() {
            if *tab == selected.0 {
                *border_color = Color::srgb(0.7, 0.4, 0.4).into();
            } else {
                *border_color = Color::NONE.into();
            }
        }
    }
}

pub fn update_total_money(
    mut text_query: Query<&mut Text, With<TotalMoney>>,
    player_query: Query<&Money, (With<Player>, Changed<Money>)>,
) {
    if let Ok(money) = player_query.get_single() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("Money {}", money.get());
        }
    }
}

struct EquipListBuilder<'a> {
    fonts: Res<'a, FontHandles>,
    weapons: Res<'a, Assets<WeaponData>>,
    armors: Res<'a, Assets<ArmorData>>,
    helmets: Res<'a, Assets<HelmetData>>,
    shoes: Res<'a, Assets<ShoesData>>,
    container: Entity,
    player_belt: &'a EquipmentBelt,
    player_storage: &'a Storage,
}

impl<'a> EquipListBuilder<'a> {
    pub fn new(
        fonts: Res<'a, FontHandles>,
        weapons: Res<'a, Assets<WeaponData>>,
        armors: Res<'a, Assets<ArmorData>>,
        helmets: Res<'a, Assets<HelmetData>>,
        shoes: Res<'a, Assets<ShoesData>>,
        container: Entity,
        player_belt: &'a EquipmentBelt,
        player_storage: &'a Storage,
    ) -> Self {
        Self {
            fonts,
            weapons,
            armors,
            helmets,
            shoes,
            container,
            player_belt,
            player_storage,
        }
    }

    pub fn clean_up_container(&self, commands: &mut Commands) {
        commands.entity(self.container).despawn_descendants();
    }

    pub fn build_armor(&self, commands: &mut Commands) {
        for (_, record) in self.armors.iter() {
            if let Some(child) = self.build_eq_list(record, commands) {
                commands.entity(self.container).add_child(child);
            }
        }
    }

    pub fn build_helmet(&self, commands: &mut Commands) {
        for (_, record) in self.helmets.iter() {
            if let Some(child) = self.build_eq_list(record, commands) {
                commands.entity(self.container).add_child(child);
            }
        }
    }

    pub fn build_shoes(&self, commands: &mut Commands) {
        for (_, record) in self.shoes.iter() {
            if let Some(child) = self.build_eq_list(record, commands) {
                commands.entity(self.container).add_child(child);
            }
        }
    }

    pub fn build_weapon(&self, commands: &mut Commands) {
        for (_, record) in self.weapons.iter() {
            if let Some(child) = self.build_eq_list(record, commands) {
                commands.entity(self.container).add_child(child);
            }
        }
    }

    fn build_eq_list(
        &self,
        equipment: &dyn EquipmentData,
        commands: &mut Commands,
    ) -> Option<Entity> {
        let is_owned = self.player_storage.is_owned(equipment);

        let is_equipped = self.player_belt.is_equipped(equipment);

        let can_upgrade = is_equipped.clone().is_some_and(|equipped| {
            is_owned
                .clone()
                .is_some_and(|owned| owned.level > equipped.level)
        });

        if let Some(owned) = is_owned {
            let row = commands
                .spawn((
                    Name::new(equipment.name()),
                    ButtonBundle {
                        background_color: Color::NONE.into(),
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            width: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            border: UiRect::all(Val::Px(2.)),
                            ..default()
                        },
                        border_color: Color::NONE.into(),
                        ..default()
                    },
                    EqipmentRow(owned.clone()),
                ))
                .id();
            let name = commands
                .spawn((
                    Name::new("Eq name"),
                    TextBundle {
                        style: Style {
                            padding: UiRect {
                                left: Val::Px(10.),
                                right: Val::Px(10.),
                                top: Val::ZERO,
                                bottom: Val::ZERO,
                            },
                            width: Val::Percent(40.),
                            min_width: Val::Px(80.),
                            max_width: Val::Px(200.),
                            ..default()
                        },
                        text: Text {
                            sections: {
                                vec![TextSection {
                                    value: equipment.name(),
                                    style: TextStyle {
                                        font: self
                                            .fonts
                                            .0
                                            .get(&FontAsset::FiraMonoMedium)
                                            .unwrap()
                                            .clone(),
                                        font_size: 17.0,
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

            // let owned = commands
            //     .spawn((
            //         Name::new("Eq owned"),
            //         TextBundle {
            //             style: Style {
            //                 padding: UiRect {
            //                     left: Val::Px(10.),
            //                     right: Val::Px(10.),
            //                     top: Val::ZERO,
            //                     bottom: Val::ZERO,
            //                 },
            //                 width: Val::Percent(20.),
            //                 min_width: Val::Px(20.),
            //                 max_width: Val::Px(200.),
            //                 ..default()
            //             },
            //             text: Text {
            //                 sections: {
            //                     vec![TextSection {
            //                         value: (is_owned as i32).to_string(),
            //                         style: TextStyle {
            //                             font: self.fonts.0.get(&FontAsset::FiraMonoMedium).unwrap().clone(),
            //                             font_size: 20.0,
            //                             color: Color::WHITE,
            //                         },
            //                     }]
            //                 },
            //                 ..default()
            //             },
            //             ..Default::default()
            //         },
            //     ))
            //     .id();

            let button = match (is_equipped, can_upgrade) {
                (None, _) => commands
                    .spawn((
                        Name::new("Equip"),
                        ButtonBundle {
                            // background_color: NORMAL_BUTTON_COLOR.into(),
                            style: get_buy_button_style(),
                            ..default()
                        },
                        ButtonClickEffect,
                        EquireButton(owned.clone()),
                    ))
                    .with_children(|builder| {
                        builder.spawn((TextBundle {
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
                                        value: format!("Equip"),
                                        style: TextStyle {
                                            font: self
                                                .fonts
                                                .0
                                                .get(&FontAsset::FiraMonoMedium)
                                                .unwrap()
                                                .clone(),
                                            font_size: 14.0,
                                            color: Color::WHITE,
                                        },
                                    }]
                                },
                                ..default()
                            },
                            ..Default::default()
                        },));
                    })
                    .id(),
                (Some(eq), false) => commands
                    .spawn((
                        UnequipButton(eq),
                        Name::new("Unequip"),
                        ButtonBundle {
                            // background_color: NORMAL_BUTTON_COLOR.into(),
                            style: get_buy_button_style(),
                            ..default()
                        },
                        ButtonClickEffect,
                    ))
                    .with_children(|builder| {
                        builder.spawn((TextBundle {
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
                                        value: format!("Unequip"),
                                        style: TextStyle {
                                            font: self
                                                .fonts
                                                .0
                                                .get(&FontAsset::FiraMonoMedium)
                                                .unwrap()
                                                .clone(),
                                            font_size: 14.0,
                                            color: Color::WHITE,
                                        },
                                    }]
                                },
                                ..default()
                            },
                            ..Default::default()
                        },));
                    })
                    .id(),
                (Some(eq), true) => commands
                    .spawn((
                        UpgradeButton(owned),
                        Name::new("Upgrade"),
                        ButtonBundle {
                            // background_color: NORMAL_BUTTON_COLOR.into(),
                            style: get_buy_button_style(),
                            ..default()
                        },
                        ButtonClickEffect,
                    ))
                    .with_children(|builder| {
                        builder.spawn((TextBundle {
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
                                        value: format!("Upgrade"),
                                        style: TextStyle {
                                            font: self
                                                .fonts
                                                .0
                                                .get(&FontAsset::FiraMonoMedium)
                                                .unwrap()
                                                .clone(),
                                            font_size: 14.0,
                                            color: Color::WHITE,
                                        },
                                    }]
                                },
                                ..default()
                            },
                            ..Default::default()
                        },));
                    })
                    .id(),
                _ => commands
                    .spawn((
                        Name::new("Not Owned"),
                        ButtonBundle {
                            // background_color: NORMAL_BUTTON_COLOR.into(),
                            style: get_buy_button_style(),
                            ..default()
                        },
                        ButtonClickEffect,
                    ))
                    .with_children(|builder| {
                        builder.spawn((TextBundle {
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
                                        value: format!("Not Owned"),
                                        style: TextStyle {
                                            font: self
                                                .fonts
                                                .0
                                                .get(&FontAsset::FiraMonoMedium)
                                                .unwrap()
                                                .clone(),
                                            font_size: 14.0,
                                            color: Color::WHITE,
                                        },
                                    }]
                                },
                                ..default()
                            },
                            ..Default::default()
                        },));
                    })
                    .id(),
            };

            info!("added eq record: {:?}", name);

            Some(commands.entity(row).push_children(&[name, button]).id())
        } else {
            None
        }
    }
}
