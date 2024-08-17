use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
    utils::hashbrown::HashMap,
};

use crate::{model::ItemType, preload::MyAssetPath};

// use crate::{character::component::MonsterType, model::ItemType, AssetsExtension, ReadyFlag};

// use crate::controller::Screen;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<EquipmentHandles>()
        .init_resource::<EquipmentHandles>()
        .register_type::<UiImageHandles>()
        .init_resource::<UiImageHandles>();
}
// pub struct HeroActionHandles(pub HashMap<HeroClass, ActionSet>);

impl MyAssetPath for ItemType {
    fn path_names(&self) -> Vec<String> {
        match self {
            ItemType::Weapon => vec![
                "images/equipment/1.png".to_string(), // 1
                "images/equipment/2.png".to_string(),
            ],
            ItemType::Armor => vec![
                "images/equipment/3.png".to_string(), // 1
                "images/equipment/4.png".to_string(),
            ],
            ItemType::Helmet => vec!["images/equipment/1.png".to_string()],
            ItemType::Shoes => vec!["images/equipment/1.png".to_string()],
            ItemType::Others => vec!["images/equipment/1.png".to_string()],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Reflect)]
pub enum UiImageAsset {
    ShopImage,
    Cover,
    Muddy,
    Rainy,
    Dry,
    Solid,
}

impl MyAssetPath for UiImageAsset {
    fn path_name(&self) -> String {
        match self {
            UiImageAsset::ShopImage => "images/weapon_shop.png".to_string(),
            UiImageAsset::Cover => "images/cover.png".to_string(),
            UiImageAsset::Muddy => "images/BattleGround/muddy.png".to_string(),
            UiImageAsset::Rainy => "images/BattleGround/rainy.png".to_string(),
            UiImageAsset::Dry => "images/BattleGround/dry.png".to_string(),
            UiImageAsset::Solid => "images/BattleGround/solid.png".to_string(),
        }
    }
}

#[derive(Resource, Debug, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct EquipmentHandles(pub HashMap<ItemType, Vec<Handle<Image>>>);

impl FromWorld for EquipmentHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        let pixel_art_settings = |settings: &mut ImageLoaderSettings| {
            // Use `nearest` image sampling to preserve the pixel art style.
            settings.sampler = ImageSampler::nearest();
        };

        let list = vec![
            ItemType::Weapon,
            ItemType::Armor,
            ItemType::Shoes,
            ItemType::Helmet,
            ItemType::Others,
        ];

        let map = list
            .into_iter()
            .map(|asset| {
                (
                    asset.clone(),
                    asset
                        .path_names()
                        .into_iter()
                        .map(|path| asset_server.load_with_settings(&path, pixel_art_settings))
                        .collect(),
                )
            })
            .collect();

        Self(map)
    }
}

#[derive(Resource, Debug, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct UiImageHandles(pub HashMap<UiImageAsset, Handle<Image>>);

impl FromWorld for UiImageHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let list = vec![
            UiImageAsset::ShopImage,
            UiImageAsset::Cover,
            UiImageAsset::Rainy,
            UiImageAsset::Muddy,
            UiImageAsset::Solid,
            UiImageAsset::Dry,
        ];
        let map = list
            .into_iter()
            .map(|asset| (asset.clone(), asset_server.load(asset.path_name())))
            .collect();

        Self(map)
    }
}
