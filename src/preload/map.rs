use bevy::{asset::LoadState, prelude::*, utils::hashbrown::HashMap};
use bevy_ecs_ldtk::{assets::LdtkProject, LdtkPlugin};

use super::MyAssetPath;

// pub struct MapPreloadPlugin;

// impl Plugin for MapPreloadPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Startup, load_map_asset)
//             .add_systems(Update, check_map.run_if(in_state(Screen::Loading)));
//     }
// }

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(LdtkPlugin);
    app.register_type::<MapHandles>();
    app.init_resource::<MapHandles>();
}

#[derive(PartialEq, Eq, Clone, Hash, Debug, Reflect)]
pub enum MapAsset {
    Forbidden,
}

impl MyAssetPath for MapAsset {
    fn path_name(&self) -> String {
        match self {
            Self::Forbidden => "maps/forbidden.ldtk".to_string(),
        }
    }
}

#[derive(Resource, Debug, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct MapHandles(pub HashMap<MapAsset, Handle<LdtkProject>>);

impl FromWorld for MapHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        MapHandles(HashMap::from([(
            MapAsset::Forbidden,
            asset_server.load("map/forbidden.ldtk"),
        )]))
    }
}
