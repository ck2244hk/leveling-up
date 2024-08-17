use bevy::prelude::*;

pub mod app_state;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>()
        .enable_state_scoped_entities::<Screen>()
        .init_state::<Screen>()
        .init_state::<SimulationState>()
        .init_state::<OverlayDroppingPickingState>()
        .init_state::<OverlayEndingCreditState>()
        .init_state::<OverlayShopState>()
        .init_state::<OverlayCombatState>()
        .init_state::<TerminalState>()
        .init_state::<OverlayStatusBoardState>()
        .init_state::<OverlayScoreBoardState>()
        .add_plugins(app_state::plugin);
}

#[derive(Component)]
pub struct FirstTime;

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    #[default]
    Splash,
    Loading,
    Title,
    Credits,
    Playing,
    GameOver,
}

// This state is mainly controlling Player ButtonInput during gameplay
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Pause,
    Running,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]

pub enum OverlayDroppingPickingState {
    Pause,
    Opened,
    #[default]
    Closed,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]

pub enum OverlayEndingCreditState {
    Pause,
    Opened,
    #[default]
    Closed,
}

// This overlay controls the opening and closing of the combat screen
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]

pub enum OverlayCombatState {
    Opened,
    #[default]
    Closed,
    Pause,
}

// This state controls different stage of
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]

pub enum TerminalState {
    Talking,
    Combating,
    Auto,
    #[default]
    None,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum OverlayStatusBoardState {
    Opened,
    #[default]
    Closed,
    Popup,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum OverlayScoreBoardState {
    Opened,
    #[default]
    Closed,
    Popup,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum OverlayShopState {
    Popup,
    Opened,
    #[default]
    Closed,
}
