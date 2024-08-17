use bevy::prelude::*;

use crate::{
    game::{
        overlay::board::{
            components::{
                DropBackButton, DropFinishButton, DropNextButton, SingleDropEqGrid, SlotContainers,
            },
            SpawnScoreBoardEvent,
        },
        Player, Storage,
    },
    state::{OverlayDroppingPickingState, OverlayScoreBoardState},
};

use super::{ConfirmSlotEvent, SelectGridEvent, UnconfirmSlotEvent};

pub fn interact_with_drop_grid(
    mut button_query: Query<(&Interaction, Entity), (Changed<Interaction>, With<SingleDropEqGrid>)>,
    mut event: EventWriter<SelectGridEvent>,
) {
    for (interaction, entity) in button_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            event.send(SelectGridEvent(Some(entity)));
        }
    }
}

pub fn interact_with_next_button(
    // commands: Commands,
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<DropNextButton>)>,
    // mut current_query: Query<&mut SlotContainers, With<SlotContainers>>,
    // mut picking_bag_query: Query<&mut PickingBag>,
    // slot_query: Query<(&DropEquipment, &SelectedSlotID)>,
    // mut selected_grid_query: Query<
    //     (&mut Visibility, Entity),
    //     (With<SelectedGrid>, Without<DropNextButton>),
    // >,
    mut confirm_event: EventWriter<ConfirmSlotEvent>,
) {
    for interaction in button_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            confirm_event.send(ConfirmSlotEvent());
            // let mut current = current_query.single_mut();
            // let mut bag = picking_bag_query.single_mut();
            // let Ok((mut selected_visibility, grid_entity)) = selected_grid_query.get_single_mut()
            // else {
            //     continue;
            // };

            // if let Some((drop, _)) = slot_query.iter().find(|(_, id)| id.0 == current.0) {
            //     bag.0.push(drop.clone());
            //     current.0 += 1;
            //     let _ = visibility.set(Box::new(Visibility::Hidden));
            //     let _ = selected_visibility.set(Box::new(Visibility::Hidden));

            //     info!("removed selectedGrid");
            //     commands.entity(grid_entity).remove::<SelectedGrid>();
            // }
        }
    }
}

pub fn interact_with_back_button(
    // commands: Commands,
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<DropBackButton>)>,
    current_query: Query<&mut SlotContainers, With<SlotContainers>>,
    // mut picking_bag_query: Query<&mut PickingBag>,
    // slot_query: Query<(Entity, &SelectedSlotID, &DropEquipment)>,
    // mut grid_query: Query<
    //     (&DropEquipment, &mut Visibility, Option<&SelectedGrid>),
    //     With<SingleDropEqGrid>,
    // >,
    mut select_event: EventWriter<SelectGridEvent>,
    mut unconfirm_event: EventWriter<UnconfirmSlotEvent>,
) {
    for interaction in button_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            // First Action Unselect non confirm(Unselect -> Pop slot container, Shift slot )
            // Second Undo Selection Confirmed(Pop slot container, Shift slot  -> Pop bag)

            // 1. Check if any selected
            // if yes, only first action
            let Ok(container) = current_query.get_single() else {
                continue;
            };
            let any_selected = container.0.iter().any(|slot| slot.check_if_select());

            if any_selected {
                select_event.send(SelectGridEvent(None));
            } else {
                unconfirm_event.send(UnconfirmSlotEvent());
            }

            // let mut current = current_query.single_mut();
            // let mut bag = picking_bag_query.single_mut();

            // if let Some((entity, _, selected_item)) =
            //     slot_query.iter().find(|(_, id, _)| id.0 == current.0)
            // {
            //     //Shifting bottom container one
            //     bag.0.pop();
            //     if current.0 > 0 {
            //         current.0 -= 1;
            //     }
            //     current.0 = 2_u8.min(current.0);

            //     commands.entity(entity).remove::<DropEquipment>();
            //     commands.entity(entity).despawn_descendants();

            //     if let Some((_, mut visibility, _)) = grid_query
            //         .iter_mut()
            //         .find(|(eq, _, _)| eq.id == selected_item.id)
            //     {
            //         info!("Respawn image grid");
            //         let _ = visibility.set(Box::new(Visibility::Inherited));
            //     }
            // } else {
            //     // First Action

            //     event.send(SelectGridEvent(None));
            // }
        }
    }
}

pub fn interact_with_finish_button(
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<DropFinishButton>)>,
    mut current_query: Query<&mut SlotContainers, With<SlotContainers>>,
    mut score_bord_event: EventWriter<SpawnScoreBoardEvent>,
    mut player_query: Query<&mut Storage, With<Player>>,
    mut next_drop_picking_state: ResMut<NextState<OverlayDroppingPickingState>>,
    mut next_score_board_state: ResMut<NextState<OverlayScoreBoardState>>,
) {
    for interaction in button_query.iter_mut() {
        let Ok(mut player_storage) = player_query.get_single_mut() else {
            continue;
        };

        if *interaction == Interaction::Pressed {
            let current = current_query.single_mut();

            for item in current.drop_list().iter() {
                if let Some(item_list) = player_storage.items.get_mut(&item.item_type) {
                    if let Some(eq) = item_list.iter_mut().find(|a| a.id == item.id) {
                        eq.set_level(item.level);
                    } else {
                        item_list.push(item.clone());
                    }
                } else {
                    player_storage
                        .items
                        .insert(item.item_type.clone(), vec![item.clone()]);
                }
            }

            info!("Naving to Score Board");
            score_bord_event.send(SpawnScoreBoardEvent(current.drop_list()));
            next_drop_picking_state.set(OverlayDroppingPickingState::Closed);
            next_score_board_state.set(OverlayScoreBoardState::Opened);
        }
    }
}
