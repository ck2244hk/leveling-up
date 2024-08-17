use bevy::prelude::*;

use crate::{
    state::{OverlayScoreBoardState, Screen},
    model::sub::DropEquipment,
};

mod interactions;
mod layout;
mod update;

use interactions::*;
use layout::*;
use update::*;

#[derive(Event)]
pub struct SpawnScoreBoardEvent(pub Vec<DropEquipment>);

pub struct ScoreBoardPlugin;

impl Plugin for ScoreBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnScoreBoardEvent>()
            .add_systems(OnExit(OverlayScoreBoardState::Opened), despawn_score_board)
            .add_systems(
                Update,
                (
                    spawn_score_board,
                    interact_with_score_button,
                    interact_with_shop_button,
                    update_lv_text,
                    update_exp_text,
                    update_money_text,
                )
                    .run_if(in_state(OverlayScoreBoardState::Opened))
                    .run_if(in_state(Screen::GameOver)),
            );
    }
}
