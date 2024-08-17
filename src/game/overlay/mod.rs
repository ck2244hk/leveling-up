pub mod battle_scene;
pub mod board;
mod ending_credit;
mod header;
pub mod terminal;

use battle_scene::BattleScenePlugin;
use bevy::app::{App, Plugin};
use board::UIBoardPlugin;
use ending_credit::EndingCreditPlugin;
// use hero_pick_ui::HeroPickingUIPlugin;
use header::HeaderPlugin;
use terminal::TerminalPlugin;

pub struct InGameOverlayPlugin;

impl Plugin for InGameOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TerminalPlugin,
            BattleScenePlugin,
            UIBoardPlugin,
            EndingCreditPlugin,
            HeaderPlugin,
        ));
    }
}
