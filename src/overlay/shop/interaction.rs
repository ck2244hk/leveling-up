use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::{
    game::player::{Armor, EquipmentBelt, Helmet, Money, Player, Shoes, Weapon},
    model::{
        armor::ArmorData, helmet::HelmetData, player::PlayerData, shoes::ShoesData,
        weapon::WeaponData,
    },
    overlay::component::*,
    preload::data::PlayerPreloadHandler,
    state::OverlayShopState,
};

use super::{
    ActiveShopTab, NotEnoughMoneyPopupEvent, SpawnConfirmPopupEvent, SpawnEquipPopupEvent,
};

pub fn interact_with_eq_list(
    mut button_query: Query<
        (&Interaction, &mut BorderColor, &EqipmentRow),
        (Changed<Interaction>, With<EqipmentRow>),
    >,
    mut shop_query: Query<&mut SelectedEquipment, With<Shop>>,
) {
    for (interaction, _border_color, row) in button_query.iter_mut() {
        let Ok(mut selected) = shop_query.get_single_mut() else {
            continue;
        };

        match *interaction {
            // Interaction::Hovered => *border_color = Color::VIOLET.into(),
            Interaction::Pressed => {
                selected.set_if_neq(SelectedEquipment(Some(row.0.clone())));
                info!("Selected: {:?}", row.0);
            }
            _ => (),
        }
    }
}

pub fn interact_with_shop_tab(
    mut button_query: Query<(&Interaction, &ShopTab), (Changed<Interaction>, With<ShopTab>)>,
    mut shop_query: Query<(&mut ActiveShopTab, &mut SelectedEquipment), With<Shop>>,
) {
    for (interaction, tab) in button_query.iter_mut() {
        let Ok((mut activated, mut selected)) = shop_query.get_single_mut() else {
            continue;
        };

        match *interaction {
            // Interaction::Hovered => *border_color = Color::VIOLET.into(),
            Interaction::Pressed => {
                if activated.set_if_neq(ActiveShopTab(tab.clone())) {
                    selected.set_if_neq(SelectedEquipment(None));
                }

                info!("Selected: {:?}", tab);
            }
            _ => (),
        }
    }
}

pub fn interact_with_back_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BackButton>),
    >,
    mut next_shop_state: ResMut<NextState<OverlayShopState>>,
) {
    for (interaction, mut background_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // *background_color = PRESSED_BUTTON_COLOR.into();

                next_shop_state.set(OverlayShopState::Closed);
                info!("Close Shop");
            }

            // Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
            _ => (),
        }
    }
}

pub fn interact_with_equire_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &EquireButton),
        (Changed<Interaction>, With<EquireButton>),
    >,
    player_assets: Res<Assets<PlayerData>>,
    player_id: Res<PlayerPreloadHandler>,
    mut shop_query: Query<(&mut ActiveShopTab, &mut SelectedEquipment), With<Shop>>,
    mut next_shop_state: ResMut<NextState<OverlayShopState>>,
    mut player_query: Query<&mut EquipmentBelt, With<Player>>,
    weapon_data: Res<Assets<WeaponData>>,
    armor_data: Res<Assets<ArmorData>>,
    helmet_data: Res<Assets<HelmetData>>,
    shoes_data: Res<Assets<ShoesData>>,
) {
    for (interaction, mut background_color, button) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                let Ok(mut eq_belt) = player_query.get_single_mut() else {
                    continue;
                };

                // *background_color = PRESSED_BUTTON_COLOR.into();

                match button.0.item_type {
                    crate::model::ItemType::Weapon => {
                        eq_belt.weapon = Weapon::from(&weapon_data, button.0.clone());
                    }
                    crate::model::ItemType::Armor => {
                        info!("Wearing Armor: {:?}", button.0.id);
                        eq_belt.armor = Armor::from(&armor_data, button.0.clone());
                    }
                    crate::model::ItemType::Helmet => {
                        eq_belt.helmet = Helmet::from(&helmet_data, button.0.clone())
                    }
                    crate::model::ItemType::Shoes => {
                        eq_belt.shoes = Shoes::from(&shoes_data, button.0.clone())
                    }
                    crate::model::ItemType::Others => (),
                }
                let (mut active, mut selected) =
                    shop_query.get_single_mut().expect("No Shop been spawned");

                selected.set_if_neq(SelectedEquipment(None));
                active.set_changed();
                next_shop_state.set(OverlayShopState::Opened);
            }

            //     match equipment_query
            //         .iter()
            //         .find(|(eq, _, _)| eq.get() == button.0)
            //     {
            //         Some((_, Some(_wp), None)) => {
            //             eq_weapon.0 = button.0;
            //         }
            //         Some((_, _, Some(_ar))) => {
            //             eq_armor.0 = button.0;
            //         }
            //         _ => todo!(),
            //     }
            //     *background_color = PRESSED_BUTTON_COLOR.into();

            //     selected.set_if_neq(SelectedEquipment(0));
            //     active.set_changed();
            //     next_shop_state.set(OverlayShopState::Opened);
            //     info!("Equire Equipment{}", button.0);
            // }
            // Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
            _ => (),
        }
    }
}

pub fn interact_with_upgrade_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &UpgradeButton),
        (Changed<Interaction>, With<UpgradeButton>),
    >,
    player_assets: Res<Assets<PlayerData>>,
    player_id: Res<PlayerPreloadHandler>,
    mut shop_query: Query<(&mut ActiveShopTab, &mut SelectedEquipment), With<Shop>>,
    mut next_shop_state: ResMut<NextState<OverlayShopState>>,
    mut player_query: Query<&mut EquipmentBelt, With<Player>>,
    weapon_data: Res<Assets<WeaponData>>,
    armor_data: Res<Assets<ArmorData>>,
    helmet_data: Res<Assets<HelmetData>>,
    shoes_data: Res<Assets<ShoesData>>,
) {
    for (interaction, mut background_color, button) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                let Ok(mut eq_belt) = player_query.get_single_mut() else {
                    continue;
                };

                info!("Wearing Armor: {:?}", button.0.id);

                // *background_color = PRESSED_BUTTON_COLOR.into();

                match button.0.item_type {
                    crate::model::ItemType::Weapon => {
                        eq_belt.weapon = Weapon::from(&weapon_data, button.0.clone());
                    }
                    crate::model::ItemType::Armor => {
                        eq_belt.armor = Armor::from(&armor_data, button.0.clone());
                    }
                    crate::model::ItemType::Helmet => {
                        eq_belt.helmet = Helmet::from(&helmet_data, button.0.clone())
                    }
                    crate::model::ItemType::Shoes => {
                        eq_belt.shoes = Shoes::from(&shoes_data, button.0.clone())
                    }
                    crate::model::ItemType::Others => (),
                }
                let (mut active, mut selected) =
                    shop_query.get_single_mut().expect("No Shop been spawned");

                selected.set_if_neq(SelectedEquipment(None));
                active.set_changed();
                next_shop_state.set(OverlayShopState::Opened);
            }

            //     match equipment_query
            //         .iter()
            //         .find(|(eq, _, _)| eq.get() == button.0)
            //     {
            //         Some((_, Some(_wp), None)) => {
            //             eq_weapon.0 = button.0;
            //         }
            //         Some((_, _, Some(_ar))) => {
            //             eq_armor.0 = button.0;
            //         }
            //         _ => todo!(),
            //     }
            //     *background_color = PRESSED_BUTTON_COLOR.into();

            //     selected.set_if_neq(SelectedEquipment(0));
            //     active.set_changed();
            //     next_shop_state.set(OverlayShopState::Opened);
            //     info!("Equire Equipment{}", button.0);
            // }
            // Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
            _ => (),
        }
    }
}

pub fn interact_with_unequip_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &UnequipButton),
        (Changed<Interaction>, With<UnequipButton>),
    >,
    mut shop_query: Query<(&mut ActiveShopTab, &mut SelectedEquipment), With<Shop>>,
    mut next_shop_state: ResMut<NextState<OverlayShopState>>,
    mut player_query: Query<&mut EquipmentBelt, With<Player>>,
) {
    for (interaction, mut background_color, button) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                info!("Unequip button being pressed");
                let Ok(mut eq_belt) = player_query.get_single_mut() else {
                    continue;
                };

                // *background_color = PRESSED_BUTTON_COLOR.into();

                match button.0.item_type {
                    crate::model::ItemType::Weapon => {
                        eq_belt.weapon = None;
                    }
                    crate::model::ItemType::Armor => {
                        eq_belt.armor = None;
                    }
                    crate::model::ItemType::Helmet => {
                        eq_belt.helmet = None;
                    }
                    crate::model::ItemType::Shoes => {
                        eq_belt.shoes = None;
                    }
                    crate::model::ItemType::Others => (),
                }
                let (mut active, mut selected) =
                    shop_query.get_single_mut().expect("No Shop been spawned");

                selected.set_if_neq(SelectedEquipment(Some(button.0.clone())));
                active.set_changed();
                next_shop_state.set(OverlayShopState::Opened);
            }

            // Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
            _ => (),
        }
    }
}

pub fn interact_with_buy_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &BuyButton),
        (Changed<Interaction>, With<BuyButton>),
    >,
    mut next_shop_state: ResMut<NextState<OverlayShopState>>,
    mut pop_up_event: EventWriter<SpawnConfirmPopupEvent>,
) {
    for (interaction, mut background_color, button) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // *background_color = PRESSED_BUTTON_COLOR.into();
                pop_up_event.send(SpawnConfirmPopupEvent(button.0.clone()));
                next_shop_state.set(OverlayShopState::Popup);
                info!("Confirm Window");
            }

            // Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
            _ => (),
        }
    }
}

pub fn interact_with_confirm_buy_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &ConfirmButton),
        (Changed<Interaction>, With<ConfirmButton>),
    >,
    next_shop_state: ResMut<NextState<OverlayShopState>>,
    player_query: Query<&mut Money, With<Player>>,
    not_enough_event: EventWriter<NotEnoughMoneyPopupEvent>,
    shop_query: Query<(&mut ActiveShopTab, &mut SelectedEquipment), With<Shop>>,
    spawn_equip_event: EventWriter<SpawnEquipPopupEvent>,
) {
    for (interaction, mut background_color, button) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // *background_color = PRESSED_BUTTON_COLOR.into();
                // let (mut money, mut bought_equipment) =
                //     player_query.get_single_mut().expect("No Player been found");
                // if let Some((equipment, cost)) =
                //     equipment_query.iter().find(|(eq, _)| eq.get() == button.0)
                // {
                //     if money.pay(cost) {
                //         bought_equipment.0.insert(equipment.get());
                //         let (mut active, mut selected) =
                //             shop_query.get_single_mut().expect("No Shop been spawned");

                //         selected.set_changed();
                //         active.set_changed();
                //         spawn_equip_event.send(SpawnEquipPopupEvent(equipment.get()));
                //     } else {
                //         not_enough_event.send(NotEnoughMoneyPopupEvent);
                //         next_shop_state.set(OverlayShopState::Opened);
                //     }
                // }

                // info!("Confirm Window");
            }

            // Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
            _ => (),
        }
    }
}

pub fn interact_with_cancel_buy_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<CancelButton>),
    >,
    mut next_shop_state: ResMut<NextState<OverlayShopState>>,
) {
    for (interaction, mut background_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // *background_color = PRESSED_BUTTON_COLOR.into();
                next_shop_state.set(OverlayShopState::Opened);

                info!("Close Window");
            }

            // Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
            _ => (),
        }
    }
}

pub fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            let items_height = list_node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.top = Val::Px(scrolling_list.position);
        }
    }
}

pub fn input_touch_pressed_move(
    touches: Res<Touches>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node, &Children)>,
    query_node: Query<(&Interaction, &Node)>,
) {
    for t in touches.iter() {
        let Some(touch) = touches.get_pressed(t.id()) else {
            info!("Get Pressed");
            continue;
        };

        // info!("Is touching");

        for (mut scrolling_list, mut style, parent, list_node, children) in query_list.iter_mut() {
            // info!("Found scrolling list");
            let mut is_started_on_list: bool = false;

            for child in children {
                if let Ok((interaction, _)) = query_node.get(*child) {
                    if *interaction == Interaction::Pressed {
                        is_started_on_list = true;
                    }
                }
            }

            if let Ok((interaction, container)) = query_node.get(parent.get()) {
                // info!("Found scrolling list container");
                if *interaction != Interaction::Pressed && !is_started_on_list {
                    // info!("Not Starting with position of List");
                    continue;
                }

                // info!("Is touching and found scrolling list");

                let container_height = container.size().y;
                let items_height = list_node.size().y;

                let max_scroll = (items_height - container_height).max(0.);

                let dy = touch.delta().y;
                info!("Delta Y: touch {}", dy);

                scrolling_list.position += dy;
                scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
                style.top = Val::Px(scrolling_list.position);
            }
        }
    }
}
