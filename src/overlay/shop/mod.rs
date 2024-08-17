use bevy::prelude::*;

mod interaction;
mod layout;
mod popup;
mod update;
use interaction::*;
use layout::*;
use popup::*;
use update::*;

use crate::{model::sub::DropEquipment, state::OverlayShopState};

use super::component::ActiveShopTab;

#[derive(Event)]
pub struct SpawnConfirmPopupEvent(DropEquipment);

#[derive(Event)]
pub struct SpawnEquipPopupEvent(DropEquipment);

#[derive(Event)]
pub struct NotEnoughMoneyPopupEvent;

pub struct ShopUIPlugin;

impl Plugin for ShopUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnConfirmPopupEvent>()
            .add_event::<NotEnoughMoneyPopupEvent>()
            .add_event::<SpawnEquipPopupEvent>()
            .add_systems(OnExit(OverlayShopState::Closed), spawn_shop)
            .add_systems(OnEnter(OverlayShopState::Closed), despawn_shop)
            .add_systems(
                Update,
                (
                    input_touch_pressed_move,
                    mouse_scroll,
                    spawn_eq_list,
                    update_selected_row_color,
                    interact_with_eq_list,
                    interact_with_shop_tab,
                    update_activated_tab_color,
                    update_total_money,
                    interact_with_back_button,
                    update_current_equired_pic,
                    update_current_equired_attr,
                    // update_current_equired_description,
                    update_selected_equip_pic,
                    update_selected_equip_attr,
                    update_selected_equip_description,
                    interact_with_buy_button,
                    interact_with_unequip_button,
                    interact_with_upgrade_button,
                )
                    .run_if(in_state(OverlayShopState::Opened)),
            )
            .add_systems(
                Update,
                (spawn_buy_pop_up, update_warning_text, spawn_warning_text),
            )
            .add_systems(
                Update,
                interact_with_equire_button.run_if(not(in_state(OverlayShopState::Closed))),
            )
            .add_systems(
                Update,
                (
                    interact_with_confirm_buy_button,
                    interact_with_cancel_buy_button,
                    spawn_equip_pop_up,
                )
                    .run_if(in_state(OverlayShopState::Popup)),
            )
            .add_systems(OnExit(OverlayShopState::Popup), despawn_pop_up)
            .register_type::<ActiveShopTab>();
    }
}
