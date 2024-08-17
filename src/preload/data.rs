use bevy::prelude::*;

use crate::{
    model::{
        armor::ArmorData, helmet::HelmetData, monster::MonsterData, player::PlayerData,
        shoes::ShoesData, weapon::WeaponData,
    },
    state::Screen,
    util::{
        csv_helper::{CsvAssetPlugin, LoadedCsv},
        json_helper::JsonAssetPlugin,
    },
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        CsvAssetPlugin::<WeaponData>::new(&["weapon.csv"]),
        CsvAssetPlugin::<ArmorData>::new(&["armor.csv"]),
        CsvAssetPlugin::<HelmetData>::new(&["helmet.csv"]),
        CsvAssetPlugin::<ShoesData>::new(&["shoes.csv"]),
        CsvAssetPlugin::<MonsterData>::new(&["monster.csv"]),
        JsonAssetPlugin::<PlayerData>::new(&["json"]),
    ))
    .add_systems(
        OnEnter(Screen::Loading),
        (
            load_weapon,
            load_player,
            load_armor,
            load_helmet,
            load_shoes,
            load_monster,
        ),
    );
}

#[derive(Resource, Debug, Deref, DerefMut, Reflect, Clone)]
pub struct PlayerPreloadHandler(pub Handle<PlayerData>);

fn load_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player = PlayerPreloadHandler(asset_server.load("data/player.json"));
    commands.insert_resource(player);
}

#[derive(Resource, Debug, Deref, DerefMut, Reflect)]
pub struct WeaponPreloadHandler(pub Handle<LoadedCsv<WeaponData>>);

fn load_weapon(mut commands: Commands, asset_server: Res<AssetServer>) {
    let weapons = WeaponPreloadHandler(asset_server.load("data/1.weapon.csv"));
    commands.insert_resource(weapons);
}

#[derive(Resource, Debug, Deref, DerefMut, Reflect)]
pub struct ArmorPreloadHandler(pub Handle<LoadedCsv<ArmorData>>);

fn load_armor(mut commands: Commands, asset_server: Res<AssetServer>) {
    let armor = ArmorPreloadHandler(asset_server.load("data/1.armor.csv"));
    commands.insert_resource(armor);
}

#[derive(Resource, Debug, Deref, DerefMut, Reflect)]
pub struct HelmetPreloadHandler(pub Handle<LoadedCsv<HelmetData>>);

fn load_helmet(mut commands: Commands, asset_server: Res<AssetServer>) {
    let helmet = HelmetPreloadHandler(asset_server.load("data/1.helmet.csv"));
    commands.insert_resource(helmet);
}

#[derive(Resource, Debug, Deref, DerefMut, Reflect)]
pub struct ShoesPreloadHandler(pub Handle<LoadedCsv<ShoesData>>);

fn load_shoes(mut commands: Commands, asset_server: Res<AssetServer>) {
    let shoes = ShoesPreloadHandler(asset_server.load("data/1.shoes.csv"));
    commands.insert_resource(shoes);
}

#[derive(Resource, Debug, Deref, DerefMut, Reflect)]
pub struct MonsterPreloadHandler(pub Handle<LoadedCsv<MonsterData>>);

fn load_monster(mut commands: Commands, asset_server: Res<AssetServer>) {
    let monster = MonsterPreloadHandler(asset_server.load("data/1.monster.csv"));
    commands.insert_resource(monster);
}
