use std::sync::Arc;

use bevy::{
    ecs::{component::Component, reflect::ReflectComponent},
    prelude::Vec2,
    reflect::{std_traits::ReflectDefault, Reflect},
};
// #[cfg(feature = "inspect")]
// use bevy_inspector_egui::prelude::ReflectInspectorOptions;
// #[cfg(feature = "inspect")]
// use bevy_inspector_egui::InspectorOptions;

use super::{
    action::NoAction, behavior::JoystickFloating, VirtualJoystickAction, VirtualJoystickBehavior,
};

#[derive(Component, Copy, Clone, Debug, Default, Reflect)]
#[reflect(Component, Default)]
// #[cfg_attr(feature = "inspect", derive(InspectorOptions))]
// #[cfg_attr(feature = "inspect", reflect(InspectorOptions))]
pub struct VirtualJoystickUIKnob;

#[derive(Component, Copy, Clone, Debug, Default, Reflect)]
#[reflect(Component, Default)]
// #[cfg_attr(feature = "inspect", derive(InspectorOptions))]
// #[cfg_attr(feature = "inspect", reflect(InspectorOptions))]
pub struct VirtualJoystickUIBackground;

#[derive(Component, Reflect)]
#[reflect(Component, Default)]
pub struct VirtualJoystickNode {
    #[reflect(ignore)]
    pub behavior: Arc<dyn VirtualJoystickBehavior>,
    #[reflect(ignore)]
    pub action: Arc<dyn VirtualJoystickAction + Sync + Send>,
}

impl Default for VirtualJoystickNode {
    fn default() -> Self {
        Self {
            behavior: Arc::new(JoystickFloating),
            action: Arc::new(NoAction),
        }
    }
}

impl std::fmt::Debug for VirtualJoystickNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VirtualJoystickNode").finish()
    }
}

#[derive(Component, Clone, Debug, Default, Reflect)]
#[reflect(Component, Default)]
pub struct VirtualJoystickState {
    pub touch_state: Option<TouchState>,
    pub just_released: bool,
    pub base_offset: Vec2,
    pub delta: Vec2,
}

impl VirtualJoystickNode {
    pub fn with_behavior(mut self, behavior: impl VirtualJoystickBehavior) -> Self {
        self.behavior = Arc::new(behavior);
        self
    }

    pub fn with_action(
        mut self,
        action: impl VirtualJoystickAction + Send + Sync + 'static,
    ) -> Self {
        self.action = Arc::new(action);
        self
    }
}

#[derive(Clone, Debug, Default, Reflect)]
#[reflect(Default)]
pub struct TouchState {
    pub id: u64,
    pub is_mouse: bool,
    pub start: Vec2,
    pub current: Vec2,
    pub just_pressed: bool,
}
