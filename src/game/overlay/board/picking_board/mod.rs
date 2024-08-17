use bevy::prelude::*;

mod interactions;
mod layout;
mod system;
mod update;

use interactions::*;
use layout::*;
use system::*;
use update::*;

use crate::state::OverlayDroppingPickingState;

use super::components::PickingBag;

#[derive(Event)]
pub struct SelectGridEvent(pub Option<Entity>); // SingleDropEqGrid Entity

#[derive(Event)]
pub struct ConfirmSlotEvent();

#[derive(Event)]
pub struct UnconfirmSlotEvent();

pub struct PickingBoardPlugin;

impl Plugin for PickingBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SelectGridEvent>()
            .add_event::<ConfirmSlotEvent>()
            .add_event::<UnconfirmSlotEvent>()
            .add_systems(
                OnEnter(OverlayDroppingPickingState::Opened),
                spawn_picking_board,
            )
            .add_systems(
                Update,
                (
                    update_drop_bag_grid,
                    interact_with_drop_grid,
                    update_buttons,
                    interact_with_next_button,
                    interact_with_back_button,
                    interact_with_finish_button,
                    handle_selecting_grid,
                    handle_unconfirm_slot,
                    handle_confirming_slot,
                    update_slots_with_containers,
                    update_grid_cell_with_containers,
                )
                    .run_if(in_state(OverlayDroppingPickingState::Opened)),
            )
            .add_systems(OnEnter(OverlayDroppingPickingState::Closed), despawn_layout);
    }
}

fn despawn_layout(mut commands: Commands, picking_scene_query: Query<Entity, With<PickingBag>>) {
    if let Ok(bag) = picking_scene_query.get_single() {
        commands.entity(bag).despawn_recursive();
    }
}
