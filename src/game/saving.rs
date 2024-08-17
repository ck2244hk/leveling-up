use bevy::{
    app::Plugin,
    ecs::{query::Changed, system::Query},
};

use super::EquipmentBelt;

pub struct SavingPlugin;

impl Plugin for SavingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {}
}

fn save_on_new_owned(player_belt: Query<&EquipmentBelt, Changed<EquipmentBelt>>) {}

// Todo: Player mainly for now.
