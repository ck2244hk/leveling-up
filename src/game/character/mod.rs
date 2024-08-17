use bevy::prelude::*;

pub mod component;
pub mod hero;
pub mod monster;

use hero::*;
use monster::*;

use self::component::BaseStates;

pub struct FighterPlugin;

impl Plugin for FighterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((HeroPlugin, MonsterPlugin))
            .register_type::<BaseStates>();
    }
}
