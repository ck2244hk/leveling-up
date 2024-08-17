use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
    utils::hashbrown::HashMap,
};

use super::MyAssetPath;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<HeroActionHandles>()
        .init_resource::<HeroActionHandles>()
        .register_type::<MonsterHandles>()
        .init_resource::<MonsterHandles>()
        .register_type::<HeroActionTextureAtLasHandles>()
        .init_resource::<HeroActionTextureAtLasHandles>()
        .register_type::<MonsterTextureAtLasHandles>()
        .init_resource::<MonsterTextureAtLasHandles>();
}

trait MySpriteTextAtlas {
    fn texture_atlas(&self) -> TextureAtlasLayout;
}

#[derive(PartialEq, Eq, Clone, Hash, Debug, Reflect)]
pub enum HeroAction {
    MenuGreeting,
    Idle,
    Decending,
    Attack,
    Attack2,
    AttackNoMovement,
    Attack2NoMovement,
    Run,
    Hit,
    Death,
    DeathNoMovement,
}

impl MyAssetPath for HeroAction {
    fn path_name(&self) -> String {
        match self {
            HeroAction::MenuGreeting => "images/_MenuGreetingSprite.png",
            HeroAction::Idle => "sprites/hero/_Idle.png",
            HeroAction::Decending => "sprites/hero/_Decending.png",
            HeroAction::Attack => "sprites/hero/_Attack.png",
            HeroAction::Attack2 => "sprites/hero/_Attack2.png",
            HeroAction::AttackNoMovement => "sprites/hero/_AttackNoMovement.png",
            HeroAction::Attack2NoMovement => "sprites/hero/_Attack2NoMovement.png",
            HeroAction::Run => "sprites/hero/_Run.png",
            HeroAction::Hit => "sprites/hero/_Hit.png",
            HeroAction::Death => "sprites/hero/_Death.png",
            HeroAction::DeathNoMovement => "sprites/hero/_DeathNoMovement.png",
        }
        .to_string()
    }
}
impl MySpriteTextAtlas for HeroAction {
    fn texture_atlas(&self) -> TextureAtlasLayout {
        match self {
            HeroAction::MenuGreeting => {
                TextureAtlasLayout::from_grid(UVec2::new(240, 345), 4, 1, None, None)
            }
            HeroAction::Idle => {
                TextureAtlasLayout::from_grid(UVec2::new(120, 80), 10, 1, None, None)
            }
            HeroAction::Decending => {
                TextureAtlasLayout::from_grid(UVec2::new(120, 80), 4, 1, None, None)
            }
            HeroAction::Attack => {
                TextureAtlasLayout::from_grid(UVec2::new(120, 80), 4, 1, None, None)
            }
            HeroAction::Attack2 => {
                TextureAtlasLayout::from_grid(UVec2::new(120, 80), 6, 1, None, None)
            }
            HeroAction::AttackNoMovement => {
                TextureAtlasLayout::from_grid(UVec2::new(120, 80), 4, 1, None, None)
            }
            HeroAction::Attack2NoMovement => {
                TextureAtlasLayout::from_grid(UVec2::new(120, 80), 6, 1, None, None)
            }
            HeroAction::Run => {
                TextureAtlasLayout::from_grid(UVec2::new(120, 80), 10, 1, None, None)
            }
            HeroAction::Hit => TextureAtlasLayout::from_grid(UVec2::new(120, 80), 1, 1, None, None),
            HeroAction::Death => {
                TextureAtlasLayout::from_grid(UVec2::new(120, 80), 10, 1, None, None)
            }
            HeroAction::DeathNoMovement => {
                TextureAtlasLayout::from_grid(UVec2::new(120, 80), 10, 1, None, None)
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Debug, Reflect)]
pub enum MonsterAsset {
    Monster1,
    Monster2,
    Monster3,
}

impl MyAssetPath for MonsterAsset {
    fn path_name(&self) -> String {
        match self {
            MonsterAsset::Monster1 => "sprites/monster/BugBearSheet.png",
            MonsterAsset::Monster2 => "sprites/monster/BatSheet.png",
            MonsterAsset::Monster3 => "sprites/monster/SkeletonSheet.png",
        }
        .to_string()
    }
}

impl MySpriteTextAtlas for MonsterAsset {
    fn texture_atlas(&self) -> TextureAtlasLayout {
        TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 4, None, None)
    }
}

#[derive(Resource, Debug, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct HeroActionHandles(pub HashMap<HeroAction, Handle<Image>>);

impl FromWorld for HeroActionHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        let pixel_art_settings = |settings: &mut ImageLoaderSettings| {
            // Use `nearest` image sampling to preserve the pixel art style.
            settings.sampler = ImageSampler::nearest();
        };

        let list = vec![
            HeroAction::MenuGreeting,
            HeroAction::Idle,
            HeroAction::Decending,
            HeroAction::Attack,
            HeroAction::Attack2,
            HeroAction::AttackNoMovement,
            HeroAction::Attack2NoMovement,
            HeroAction::Run,
            HeroAction::Hit,
            HeroAction::Death,
            HeroAction::DeathNoMovement,
        ];

        let map = list
            .into_iter()
            .map(|asset| {
                (
                    asset.clone(),
                    asset_server.load_with_settings(asset.path_name(), pixel_art_settings),
                )
            })
            .collect();
        Self(map)
    }
}

#[derive(Resource, Debug, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct HeroActionTextureAtLasHandles(pub HashMap<HeroAction, Handle<TextureAtlasLayout>>);

impl FromWorld for HeroActionTextureAtLasHandles {
    fn from_world(world: &mut World) -> Self {
        let mut asset_server = world.resource_mut::<Assets<TextureAtlasLayout>>();

        let pixel_art_settings = |settings: &mut ImageLoaderSettings| {
            // Use `nearest` image sampling to preserve the pixel art style.
            settings.sampler = ImageSampler::nearest();
        };

        let list = vec![
            HeroAction::MenuGreeting,
            HeroAction::Idle,
            HeroAction::Decending,
            HeroAction::Attack,
            HeroAction::Attack2,
            HeroAction::AttackNoMovement,
            HeroAction::Attack2NoMovement,
            HeroAction::Run,
            HeroAction::Hit,
            HeroAction::Death,
            HeroAction::DeathNoMovement,
        ];

        let map = list
            .into_iter()
            .map(|asset| (asset.clone(), asset_server.add(asset.texture_atlas())))
            .collect();
        Self(map)
    }
}

#[derive(Resource, Debug, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct MonsterHandles(pub HashMap<MonsterAsset, Handle<Image>>);

impl FromWorld for MonsterHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        let pixel_art_settings = |settings: &mut ImageLoaderSettings| {
            // Use `nearest` image sampling to preserve the pixel art style.
            settings.sampler = ImageSampler::nearest();
        };

        let list = vec![
            MonsterAsset::Monster1,
            MonsterAsset::Monster2,
            MonsterAsset::Monster3,
        ];

        let map = list
            .into_iter()
            .map(|asset| {
                (
                    asset.clone(),
                    asset_server.load_with_settings(asset.path_name(), pixel_art_settings),
                )
            })
            .collect();

        Self(map)
    }
}

#[derive(Resource, Debug, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct MonsterTextureAtLasHandles(pub HashMap<MonsterAsset, Handle<TextureAtlasLayout>>);

impl FromWorld for MonsterTextureAtLasHandles {
    fn from_world(world: &mut World) -> Self {
        let mut asset_server = world.resource_mut::<Assets<TextureAtlasLayout>>();

        let pixel_art_settings = |settings: &mut ImageLoaderSettings| {
            // Use `nearest` image sampling to preserve the pixel art style.
            settings.sampler = ImageSampler::nearest();
        };

        let list = vec![
            MonsterAsset::Monster1,
            MonsterAsset::Monster2,
            MonsterAsset::Monster3,
        ];

        let map = list
            .into_iter()
            .map(|asset| (asset.clone(), asset_server.add(asset.texture_atlas())))
            .collect();
        Self(map)
    }
}
