use std::sync::Arc;

use bevy::{
    ecs::{
        entity::Entity,
        event::EventWriter,
        query::With,
        system::{Query, Res},
        world::World,
    },
    hierarchy::Children,
    input::{mouse::MouseButton, touch::Touches, ButtonInput},
    log::warn,
    math::{Rect, Vec2},
    transform::components::GlobalTransform,
    ui::{Node, PositionType, Style, Val},
    window::{PrimaryWindow, Window},
};

use super::{
    components::{
        TouchState, VirtualJoystickState, VirtualJoystickUIBackground, VirtualJoystickUIKnob,
    },
    VirtualJoystickEvent, VirtualJoystickEventType, VirtualJoystickNode,
};
use bevy::ecs::query::Without;

pub fn update_missing_state(world: &mut World) {
    let mut joysticks = world.query::<(Entity, &VirtualJoystickNode)>();
    let mut joystick_entities: Vec<Entity> = Vec::new();
    for (joystick_entity, _) in joysticks.iter(world) {
        joystick_entities.push(joystick_entity);
    }
    for joystick_entity in joystick_entities {
        let has_state = world.get::<VirtualJoystickState>(joystick_entity).is_some();
        if !has_state {
            // info!("Insert JoyStick State");
            world
                .entity_mut(joystick_entity)
                .insert(VirtualJoystickState::default());
        }
    }
}

pub fn update_input(
    mut joysticks: Query<(&Node, &GlobalTransform, &mut VirtualJoystickState)>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    for (joystick_node, joystick_global_transform, mut joystick_state) in &mut joysticks {
        joystick_state.just_released = false;
        if let Some(touch_state) = &mut joystick_state.touch_state {
            touch_state.just_pressed = false;
        }
        if joystick_state.touch_state.is_none() {
            // info!("JoyStick Touch State is none");
            let rect = joystick_node.logical_rect(joystick_global_transform);
            for touch in touches.iter() {
                if rect.contains(touch.position()) {
                    joystick_state.touch_state = Some(TouchState {
                        id: touch.id(),
                        is_mouse: false,
                        start: touch.position(),
                        current: touch.position(),
                        just_pressed: true,
                    });
                    break;
                }
            }
            if joystick_state.touch_state.is_none() && mouse_buttons.just_pressed(MouseButton::Left)
            {
                if let Some(mouse_pos) = q_windows.single().cursor_position() {
                    if rect.contains(mouse_pos) {
                        joystick_state.touch_state = Some(TouchState {
                            id: 0,
                            is_mouse: true,
                            start: mouse_pos,
                            current: mouse_pos,
                            just_pressed: true,
                        });
                    }
                }
            }
        } else {
            let mut clear_touch_state = false;
            if let Some(touch_state) = &joystick_state.touch_state {
                if touch_state.is_mouse {
                    if mouse_buttons.just_released(MouseButton::Left) {
                        clear_touch_state = true;
                    }
                } else if touches.just_released(touch_state.id) {
                    clear_touch_state = true;
                }
            }
            if clear_touch_state {
                joystick_state.touch_state = None;
                joystick_state.just_released = true;
            } else if let Some(touch_state) = &mut joystick_state.touch_state {
                if touch_state.is_mouse {
                    if let Some(new_current) = q_windows.single().cursor_position() {
                        if new_current != touch_state.current {
                            touch_state.current = new_current;
                        }
                    }
                } else if let Some(touch) = touches.get_pressed(touch_state.id) {
                    let touch_position = touch.position();
                    if touch_position != touch_state.current {
                        touch_state.current = touch_position;
                    }
                }
            }
        }
    }
}

pub fn update_behavior_knob_delta(world: &mut World) {
    let mut joysticks = world.query::<(Entity, &VirtualJoystickNode)>();
    let mut joystick_entities: Vec<Entity> = Vec::new();
    for (joystick_entity, _) in joysticks.iter(world) {
        joystick_entities.push(joystick_entity);
    }
    for joystick_entity in joystick_entities {
        let behavior;
        {
            let Some(virtual_joystick_node) = world.get::<VirtualJoystickNode>(joystick_entity)
            else {
                continue;
            };
            behavior = Arc::clone(&virtual_joystick_node.behavior);
        }
        behavior.update_at_delta_stage(world, joystick_entity);
    }
}

pub fn update_behavior_constraints(world: &mut World) {
    let mut joysticks = world.query::<(Entity, &VirtualJoystickNode)>();
    let mut joystick_entities: Vec<Entity> = Vec::new();
    for (joystick_entity, _) in joysticks.iter(world) {
        joystick_entities.push(joystick_entity);
    }
    for joystick_entity in joystick_entities {
        let behavior;
        {
            let Some(virtual_joystick_node) = world.get::<VirtualJoystickNode>(joystick_entity)
            else {
                continue;
            };
            behavior = Arc::clone(&virtual_joystick_node.behavior);
        }
        behavior.update_at_constraint_stage(world, joystick_entity);
    }
}

pub fn update_behavior(world: &mut World) {
    let mut joysticks = world.query::<(Entity, &VirtualJoystickNode)>();
    let mut joystick_entities: Vec<Entity> = Vec::new();
    for (joystick_entity, _) in joysticks.iter(world) {
        joystick_entities.push(joystick_entity);
    }
    for joystick_entity in joystick_entities {
        let behavior;
        {
            let Some(virtual_joystick_node) = world.get::<VirtualJoystickNode>(joystick_entity)
            else {
                continue;
            };
            behavior = Arc::clone(&virtual_joystick_node.behavior);
        }
        behavior.update(world, joystick_entity);
    }
}

pub fn update_action(world: &mut World) {
    let mut joysticks = world.query::<(Entity, &VirtualJoystickNode, &mut VirtualJoystickState)>();
    let mut joystick_entities: Vec<Entity> = Vec::new();
    for (joystick_entity, _, _) in joysticks.iter(world) {
        joystick_entities.push(joystick_entity);
    }
    enum DragAction {
        StartDrag,
        Drag,
        EndDrag,
    }
    for joystick_entity in joystick_entities {
        let drag_action: Option<DragAction>;
        {
            let Some(joystick_state) = world.get::<VirtualJoystickState>(joystick_entity) else {
                continue;
            };
            if joystick_state.just_released {
                drag_action = Some(DragAction::EndDrag);
            } else if let Some(touch_state) = &joystick_state.touch_state {
                if touch_state.just_pressed {
                    drag_action = Some(DragAction::StartDrag);
                } else {
                    drag_action = Some(DragAction::Drag);
                }
            } else {
                drag_action = None;
            }
        }
        let Some(drag_action) = drag_action else {
            continue;
        };
        let action;
        let joystick_state;
        {
            let Ok((_, virtual_joystick_node, joystick_state_2)) =
                joysticks.get_mut(world, joystick_entity)
            else {
                continue;
            };
            action = Arc::clone(&virtual_joystick_node.action);
            joystick_state = joystick_state_2.clone();
        }
        match drag_action {
            DragAction::StartDrag => {
                action.on_start_drag(joystick_state, world, joystick_entity);
            }
            DragAction::Drag => {
                action.on_drag(joystick_state, world, joystick_entity);
            }
            DragAction::EndDrag => {
                action.on_end_drag(joystick_state, world, joystick_entity);
            }
        }
    }
}

pub fn update_fire_events(
    joysticks: Query<&VirtualJoystickState>,
    mut send_values: EventWriter<VirtualJoystickEvent>,
) {
    for joystick_state in &joysticks {
        if joystick_state.just_released {
            send_values.send(VirtualJoystickEvent {
                event: VirtualJoystickEventType::Up,
                value: Vec2::ZERO,
                delta: joystick_state.delta,
            });
            continue;
        }
        if let Some(touch_state) = &joystick_state.touch_state {
            if touch_state.just_pressed {
                send_values.send(VirtualJoystickEvent {
                    event: VirtualJoystickEventType::Press,
                    value: touch_state.current,
                    delta: joystick_state.delta,
                });
            }
            send_values.send(VirtualJoystickEvent {
                event: VirtualJoystickEventType::Drag,
                value: touch_state.current,
                delta: joystick_state.delta,
            });
        }
    }
}

#[allow(clippy::complexity)]
pub fn update_ui(
    joysticks: Query<(&VirtualJoystickState, &Children)>,
    mut joystick_bases: Query<
        (&mut Style, &Node, &GlobalTransform),
        With<VirtualJoystickUIBackground>,
    >,
    mut joystick_knobs: Query<
        (&mut Style, &Node, &GlobalTransform),
        (
            With<VirtualJoystickUIKnob>,
            Without<VirtualJoystickUIBackground>,
        ),
    >,
) {
    for (joystick_state, children) in &joysticks {
        let mut joystick_base_rect: Option<Rect> = None;
        // info!("Begine Update JoyStick UI");
        for child in children.iter() {
            if joystick_bases.contains(*child) {
                let (mut joystick_base_style, joystick_base_node, joystick_base_global_transform) =
                    joystick_bases.get_mut(*child).unwrap();
                joystick_base_style.position_type = PositionType::Absolute;
                joystick_base_style.left = Val::Px(joystick_state.base_offset.x);
                joystick_base_style.top = Val::Px(joystick_state.base_offset.y);
                joystick_base_rect =
                    Some(joystick_base_node.logical_rect(joystick_base_global_transform));
            }
        }
        if joystick_base_rect.is_none() {
            warn!("No joystick base found");
            continue;
        }
        // info!("JoyStick base rect is some");
        let joystick_base_rect = joystick_base_rect.unwrap();
        let joystick_base_rect_half_size = joystick_base_rect.half_size();
        for child in children.iter() {
            if joystick_knobs.contains(*child) {
                // info!("Children in Joystick state");
                let (mut joystick_knob_style, joystick_knob_node, joystick_knob_global_transform) =
                    joystick_knobs.get_mut(*child).unwrap();
                let joystick_knob_rect =
                    joystick_knob_node.logical_rect(joystick_knob_global_transform);
                let joystick_knob_half_size = joystick_knob_rect.half_size();

                joystick_knob_style.position_type = PositionType::Absolute;
                joystick_knob_style.left = Val::Px(
                    joystick_state.base_offset.x
                        + joystick_base_rect_half_size.x
                        + joystick_knob_half_size.x
                        + (joystick_state.delta.x - 1.0) * joystick_base_rect_half_size.x,
                );
                joystick_knob_style.top = Val::Px(
                    joystick_state.base_offset.y
                        + joystick_base_rect_half_size.y
                        + joystick_knob_half_size.y
                        + (-joystick_state.delta.y - 1.0) * joystick_base_rect_half_size.y,
                );

                // info!(
                //     "joystick left: {:?}, top: {:?}",
                //     joystick_knob_style.left, joystick_knob_style.top
                // );
            }
        }
    }
}
