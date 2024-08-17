use bevy::ecs::{entity::Entity, world::World};

use super::VirtualJoystickState;

pub trait VirtualJoystickAction {
    fn on_start_drag(&self, _data: VirtualJoystickState, _world: &mut World, _entity: Entity) {}
    fn on_drag(&self, _data: VirtualJoystickState, _world: &mut World, _entity: Entity) {}
    fn on_end_drag(&self, _data: VirtualJoystickState, _world: &mut World, _entity: Entity) {}
}

#[derive(Default)]
pub struct NoAction;

impl VirtualJoystickAction for NoAction {}
