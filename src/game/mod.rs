use bevy::prelude::*;

pub mod battle;
pub mod character;
mod joystick;
pub mod map;
pub mod overlay;
pub mod player;
mod saving;
use battle::*;
use character::*;
use joystick::VirtualJoystickPlugin;
use map::*;
use overlay::*;
use player::*;
use saving::*;
use terminal::*;

pub(super) fn plugin(app: &mut App) {
    app
        // .add_systems(Update, set_simulation_state)
        .add_plugins((
            PlayerPlugin,
            BattlePlugin,
            FighterPlugin,
            MapPlugin,
            SavingPlugin,
            VirtualJoystickPlugin,
            InGameOverlayPlugin,
        ))
        .add_systems(Startup, spawn_game_ui_layout);
}

#[derive(Component)]
pub struct GameUIFrame;

pub fn spawn_game_ui_layout(mut commands: Commands) {
    commands.spawn((
        Name::new("Layout"),
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Style::DEFAULT
            },
            z_index: ZIndex::Global(3),
            ..default()
        },
        GameUIFrame {},
    ));
}

// pub fn set_simulation_state(
//     app_state: Res<State<Screen>>,
//     mut next_simulation_state: ResMut<NextState<SimulationState>>,
// ) {
//     if app_state.is_changed() {
//         println!("{:?}", app_state.get());
//         match app_state.get() {
//             Screen::Playing => {
//                 // next_simulation_state.set(SimulationState::Running);
//                 println!("Set State to Exploration");
//             }
//             _ => {
//                 next_simulation_state.set(SimulationState::Pause);
//                 println!("Set State to Pause");
//             }
//         }
//     }
// }
