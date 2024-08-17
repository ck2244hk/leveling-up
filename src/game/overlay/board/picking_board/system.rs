use crate::game::overlay::board::components::InteractiveSlot;
use crate::game::overlay::board::components::InteractiveSlot::Confirmed;
use crate::game::overlay::board::components::InteractiveSlot::Selected;
use crate::game::overlay::board::components::SelectedSlotID;
use crate::game::overlay::board::components::{SelectedGrid, SingleDropEqGrid, SlotContainers};
use crate::model::sub::DropEquipment;

use super::*;

pub fn handle_selecting_grid(
    mut commands: Commands,
    mut grid_query: Query<
        (
            Entity,
            Option<&SelectedGrid>,
            &mut BorderColor,
            &DropEquipment,
        ),
        With<SingleDropEqGrid>,
    >,
    mut container_query: Query<&mut SlotContainers>,
    mut event: EventReader<SelectGridEvent>,
) {
    for ev in event.read() {
        info!("Select Event: {:?}", ev.0);

        // 1. unselected all
        for (cell_entity, selected, mut border_color, eq) in grid_query.iter_mut() {
            let Ok(mut container) = container_query.get_single_mut() else {
                continue;
            };

            if selected.is_some() {
                info!("Cleaning : {:?}", cell_entity);
                commands.entity(cell_entity).remove::<SelectedGrid>();
                border_color.0.with_alpha(0.);

                container.0.retain(|a| match a {
                    Selected(item) => item != eq,
                    Confirmed(_) => true,
                });
            }

            if ev.0.is_some_and(|a| a == cell_entity) {
                info!("Selecting : {:?}", cell_entity);
                commands.entity(cell_entity).insert(SelectedGrid {});

                border_color.0 = Color::srgb(0.4, 0.4, 0.2).into();
                container.0.push(Selected(eq.clone()));
            }

            container.is_changed();
        }
    }
}

pub fn handle_unconfirm_slot(
    mut commands: Commands,
    mut unconfirm_event: EventReader<UnconfirmSlotEvent>,
    mut grid_query: Query<(&mut Visibility, &DropEquipment), With<SingleDropEqGrid>>,
    slots_query: Query<(&SelectedSlotID, Entity)>,
    mut container_query: Query<&mut SlotContainers>,
) {
    for _ in unconfirm_event.read() {
        let Ok(mut container) = container_query.get_single_mut() else {
            continue;
        };

        loop {
            match container.0.pop() {
                Some(Selected(_)) => {}
                Some(Confirmed(item)) => {
                    if let Some((_, slot_entity)) = slots_query
                        .iter()
                        .find(|(id, _)| id.0 == slots_query.iter().len() - 1)
                    {
                        commands.entity(slot_entity).remove::<DropEquipment>();
                        commands.entity(slot_entity).despawn_descendants();
                    }

                    // restore pic on top
                    if let Some((mut visibility, _)) =
                        grid_query.iter_mut().find(|(_, eq)| eq.id == item.id)
                    {
                        info!("Respawn image grid");
                        let _ = visibility.set(Box::new(Visibility::Inherited));
                    }

                    break;
                }
                _ => {
                    break;
                }
            }
        }
    }
}

pub fn handle_confirming_slot(
    mut container_query: Query<&mut SlotContainers>,
    mut confirm_event: EventReader<ConfirmSlotEvent>,
    mut select_event: EventWriter<SelectGridEvent>,
) {
    for _ in confirm_event.read() {
        let Ok(mut container) = container_query.get_single_mut() else {
            continue;
        };

        container.0 = container
            .0
            .iter_mut()
            .map(|slot| slot.to_confirmed())
            .collect::<Vec<InteractiveSlot>>();

        select_event.send(SelectGridEvent(None));
    }
}
