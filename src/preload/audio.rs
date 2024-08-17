use bevy::{prelude::*, utils::hashbrown::HashMap};

use super::MyAssetPath;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<MusicHandles>()
        .init_resource::<MusicHandles>()
        .register_type::<AudioEffectHandles>()
        .init_resource::<AudioEffectHandles>();
}

#[derive(PartialEq, Eq, Clone, Hash, Debug, Reflect)]
pub enum MusicAsset {
    Nostalgia,
}

impl MyAssetPath for MusicAsset {
    fn path_name(&self) -> String {
        match self {
            MusicAsset::Nostalgia => "musics/nostalgia.ogg",
        }
        .to_string()
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Debug, Reflect)]
pub enum AudioEffectAsset {
    ButtonClick,
}

impl MyAssetPath for AudioEffectAsset {
    fn path_name(&self) -> String {
        match self {
            AudioEffectAsset::ButtonClick => "tracks/button_click.ogg",
        }
        .to_string()
    }
}

#[derive(Resource, Debug, Deref, DerefMut, Reflect)]
pub struct MusicHandles(pub HashMap<MusicAsset, Handle<AudioSource>>);

impl FromWorld for MusicHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let list = vec![MusicAsset::Nostalgia];

        let map = list
            .into_iter()
            .map(|assets| (assets.clone(), asset_server.load(assets.path_name())))
            .collect();
        Self(map)
    }
}

#[derive(Resource, Debug, Deref, DerefMut, Reflect)]
pub struct AudioEffectHandles(pub HashMap<AudioEffectAsset, Handle<AudioSource>>);
impl FromWorld for AudioEffectHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let list = vec![AudioEffectAsset::ButtonClick];

        let map = list
            .into_iter()
            .map(|assets| (assets.clone(), asset_server.load(assets.path_name())))
            .collect();
        Self(map)
    }
}
