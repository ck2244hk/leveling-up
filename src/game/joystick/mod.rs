use std::hash::Hash;

use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

mod action;
mod behavior;
mod bundles;
mod components;
mod systems;
mod utils;

pub use action::{NoAction, VirtualJoystickAction};
pub use behavior::{JoystickFixed, VirtualJoystickBehavior};
pub use bundles::VirtualJoystickBundle;
pub use components::{
    VirtualJoystickNode, VirtualJoystickState, VirtualJoystickUIBackground, VirtualJoystickUIKnob,
};
use systems::{
    update_action, update_behavior, update_behavior_constraints, update_behavior_knob_delta,
    update_fire_events, update_input, update_missing_state, update_ui,
};
pub use utils::create_joystick;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct UpdateKnobDelta;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConstrainKnobDelta;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FireEvents;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct UpdateUI;

#[derive(Default)]
pub struct VirtualJoystickPlugin;

// #[derive(Event)]
// pub enum InputEvent {
//     StartDrag { id: u64, pos: Vec2, is_mouse: bool },
//     Dragging { id: u64, pos: Vec2, is_mouse: bool },
//     EndDrag { id: u64, pos: Vec2, is_mouse: bool },
// }

impl Plugin for VirtualJoystickPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<VirtualJoystickNode>()
            .register_type::<VirtualJoystickEventType>()
            .add_event::<VirtualJoystickEvent>()
            .add_systems(
                PreUpdate,
                (
                    update_missing_state,
                    update_input.after(update_missing_state),
                ),
            )
            .add_systems(UpdateKnobDelta, update_behavior_knob_delta)
            .add_systems(ConstrainKnobDelta, update_behavior_constraints)
            .add_systems(FireEvents, update_fire_events)
            .add_systems(UpdateUI, (update_behavior, update_action, update_ui))
            .add_systems(Update, |world: &mut World| {
                world.run_schedule(UpdateKnobDelta);
                world.run_schedule(ConstrainKnobDelta);
                world.run_schedule(FireEvents);
                world.run_schedule(UpdateUI);
            });
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
#[reflect]
pub enum VirtualJoystickEventType {
    Press,
    Drag,
    Up,
}

#[derive(Event, Debug)]
pub struct VirtualJoystickEvent {
    event: VirtualJoystickEventType,
    value: Vec2,
    delta: Vec2,
}

impl VirtualJoystickEvent {
    /// Raw position of point (Mouse or Touch)
    pub fn value(&self) -> &Vec2 {
        &self.value
    }

    /// Delta value ranging from 0 to 1 in each vector (x and y)
    pub fn axis(&self) -> &Vec2 {
        &self.delta
    }

    /// Return the Type of Joystick Event
    pub fn get_type(&self) -> VirtualJoystickEventType {
        self.event
    }

    /// Delta value snaped
    /// the dead_zone is required for make more customizable
    /// the default of the dead_zone is 0.5
    pub fn snap_axis(&self, dead_zone: Option<f32>) -> Vec2 {
        let dead_zone = dead_zone.unwrap_or(0.);
        // info!("delta: {:?}", self.delta);
        Vec2::new(
            if self.delta.x.abs() < dead_zone {
                0.
            } else {
                self.delta.x
            },
            if self.delta.y.abs() < dead_zone {
                0.
            } else {
                self.delta.y
            },
            // if self.delta.x <= -dash_zone {
            //     // info!("Dashing");
            //     -1.
            // } else if self.delta.x >= dash_zone {
            //     // info!("Dashing");
            //     1.
            // } else if self.delta.x < -dead_zone {
            //     // info!("Walk");
            //     -0.7
            // } else if self.delta.x > dead_zone {
            //     // info!("Walk");
            //     0.7
            // } else {
            //     // info!("Dead");
            //     0.0
            // },
            // if self.delta.y <= -dash_zone {
            //     // info!("Dashing");
            //     -1.
            // } else if self.delta.y >= dash_zone {
            //     // info!("Dashing");
            //     1.
            // } else if self.delta.y < -dead_zone {
            //     // info!("Walk");
            //     -0.7
            // } else if self.delta.y > dead_zone {
            //     // info!("Walk");
            //     0.7
            // } else {
            //     // info!("Dead");
            //     0.0
            // },
        )
    }
}
