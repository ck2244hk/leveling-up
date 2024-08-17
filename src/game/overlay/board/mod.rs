use crate::game::{
    character::component::{Critical, Defense, Strength},
    Money,
};

use bevy::prelude::*;

pub mod components;
mod picking_board;
mod pop_up;
mod score_board;
pub mod status_board;
mod styles;

use picking_board::*;
use pop_up::*;
use score_board::*;
use status_board::*;

#[derive(Event)]
pub struct SpawnRestartConfirmPopupEvent;

pub struct UIBoardPlugin;

impl Plugin for UIBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnRestartConfirmPopupEvent>()
            .add_plugins((
                StatusBoardPlugin,
                ScoreBoardPlugin,
                MenuPopupPlugin,
                PickingBoardPlugin,
            ));
    }
}

struct StatusPack<'a> {
    pub strength: &'a Strength,
    pub defense: &'a Defense,
    pub critical: &'a Critical,
    pub money: &'a Money,
}

impl<'a> StatusPack<'a> {
    fn new(
        strength: &'a Strength,
        defense: &'a Defense,
        critical: &'a Critical,
        money: &'a Money,
    ) -> Self {
        StatusPack {
            strength,
            defense,
            critical,
            money,
        }
    }
}
