//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on WASM.

use bevy::{prelude::*, utils::HashMap};

use crate::{
    assets::{BgmHandles, ImageHandles, SfxHandles},
    model::monster,
    preload::{
        audio::{AudioEffectHandles, MusicHandles},
        data::{
            ArmorPreloadHandler, HelmetPreloadHandler, MonsterPreloadHandler, PlayerPreloadHandler,
            ShoesPreloadHandler, WeaponPreloadHandler,
        },
        fonts::FontHandles,
        images::{EquipmentHandles, UiImageHandles},
        map::MapHandles,
        sprites::{HeroActionHandles, MonsterHandles},
        MyAssetPath,
    },
    state::Screen,
    theme::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), show_loading_screen);
    app.add_systems(
        Update,
        continue_to_title.run_if(
            in_state(Screen::Loading)
                .and_then(all_assets_loaded)
                .and_then(all_data_loaded),
        ),
    );
}

fn show_loading_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Loading))
        .with_children(|children| {
            children.label("Loading...");
        });
}

fn all_data_loaded(
    asset_server: Res<AssetServer>,
    player_data_handles: Res<PlayerPreloadHandler>,
    weapon_handles: Res<WeaponPreloadHandler>,
    helmet_handles: Res<HelmetPreloadHandler>,
    armor_handles: Res<ArmorPreloadHandler>,
    shoes_handles: Res<ShoesPreloadHandler>,
    monster_data_handles: Res<MonsterPreloadHandler>,
) -> bool {
    player_data_handles.is_loaded(&asset_server)
        && weapon_handles.is_loaded(&asset_server)
        && helmet_handles.is_loaded(&asset_server)
        && armor_handles.is_loaded(&asset_server)
        && shoes_handles.is_loaded(&asset_server)
        && monster_data_handles.is_loaded(&asset_server)
}

fn all_assets_loaded(
    asset_server: Res<AssetServer>,
    image_handles: Res<ImageHandles>,
    sfx_handles: Res<SfxHandles>,
    bgm_handles: Res<BgmHandles>,
    map_handles: Res<MapHandles>,
    hero_action_handles: Res<HeroActionHandles>,
    monster_handles: Res<MonsterHandles>,
    equipment_handles: Res<EquipmentHandles>,
    ui_image_handles: Res<UiImageHandles>,
    font_handles: Res<FontHandles>,
    music_handles: Res<MusicHandles>,
    audio_effect_handles: Res<AudioEffectHandles>,
) -> bool {
    image_handles.is_loaded(&asset_server)
        && sfx_handles.is_loaded(&asset_server)
        && bgm_handles.is_loaded(&asset_server)
        && map_handles.is_loaded(&asset_server)
        && hero_action_handles.is_loaded(&asset_server)
        && monster_handles.is_loaded(&asset_server)
        && equipment_handles.is_loaded(&asset_server)
        && ui_image_handles.is_loaded(&asset_server)
        && font_handles.is_loaded(&asset_server)
        && music_handles.is_loaded(&asset_server)
        && audio_effect_handles.is_loaded(&asset_server)
}

fn continue_to_title(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

/// An extension trait to check if all the assets in an asset collection are
/// loaded.
trait AssetsExtension {
    fn is_loaded(&self, asset_server: &AssetServer) -> bool;
}

impl<T: Asset> AssetsExtension for HashMap<String, Handle<T>> {
    fn is_loaded(&self, asset_server: &AssetServer) -> bool {
        self.values()
            .all(|x| asset_server.is_loaded_with_dependencies(x))
    }
}

impl<T: Asset> AssetsExtension for HashMap<String, Vec<Handle<T>>> {
    fn is_loaded(&self, asset_server: &AssetServer) -> bool {
        self.values()
            .flatten()
            .all(|x| asset_server.is_loaded_with_dependencies(x))
    }
}

impl<T: Asset> AssetsExtension for Handle<T> {
    fn is_loaded(&self, asset_server: &AssetServer) -> bool {
        asset_server.is_loaded_with_dependencies(self.id())
    }
}

impl<T: Asset, N: MyAssetPath> AssetsExtension for HashMap<N, Handle<T>> {
    fn is_loaded(&self, asset_server: &AssetServer) -> bool {
        self.values()
            .all(|x| asset_server.is_loaded_with_dependencies(x))
    }
}

impl<T: Asset, N: MyAssetPath> AssetsExtension for HashMap<N, Vec<Handle<T>>> {
    fn is_loaded(&self, asset_server: &AssetServer) -> bool {
        self.values()
            .flatten()
            .all(|x| asset_server.is_loaded_with_dependencies(x))
    }
}
