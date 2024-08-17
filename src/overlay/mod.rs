use bevy::prelude::*;

mod component;
mod shop;
mod styles;

use shop::ShopUIPlugin;

#[derive(Event)]
pub struct SpawnShopEvent {}

pub struct GeneralOverlayPlugin;

impl Plugin for GeneralOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnShopEvent>().add_plugins(ShopUIPlugin);
    }
}
