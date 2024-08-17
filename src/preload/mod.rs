pub mod audio;
pub mod data;
pub mod fonts;
pub mod images;
pub mod map;
pub mod sprites;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        audio::plugin,
        data::plugin,
        fonts::plugin,
        images::plugin,
        map::plugin,
        sprites::plugin,
    ));
}

pub trait MyAssetPath {
    fn path_name(&self) -> String {
        String::new()
    }

    fn path_names(&self) -> Vec<String> {
        Vec::new()
    }
}
