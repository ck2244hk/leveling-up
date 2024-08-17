use bevy::{prelude::*, utils::HashMap};

use super::MyAssetPath;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<FontHandles>()
        .init_resource::<FontHandles>();
}

#[derive(PartialEq, Eq, Clone, Hash, Debug, Reflect)]
pub enum FontAsset {
    FiraMonoMedium,
    FiraMonoBold,
    Cursor,
}

impl MyAssetPath for FontAsset {
    fn path_name(&self) -> String {
        match self {
            Self::FiraMonoMedium => "fonts/FiraMono-Medium.ttf",
            Self::FiraMonoBold => "fonts/FiraSans-Bold.ttf",
            Self::Cursor => "fonts/Cursor.ttf",
        }
        .to_string()
    }
}

#[derive(Resource, Debug, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct FontHandles(pub HashMap<FontAsset, Handle<Font>>);

impl FromWorld for FontHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let list = vec![
            FontAsset::Cursor,
            FontAsset::FiraMonoMedium,
            FontAsset::FiraMonoBold,
        ];

        let map = list
            .into_iter()
            .map(|font| (font.clone(), asset_server.load(font.path_name())))
            .collect();
        Self(map)
    }
}
