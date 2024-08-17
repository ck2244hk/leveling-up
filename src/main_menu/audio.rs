use bevy::prelude::*;

use crate::{audio::MusicAsset, AudioFadeIn};

#[derive(Component)]
pub struct MenuBackgroundMusic;

pub fn spawn_background_music(
    mut commands: Commands,
    music_assets: Res<MusicAsset>,
    background_query: Query<&MenuBackgroundMusic>,
) {
    if background_query.is_empty() {
        commands.spawn((
            AudioBundle {
                source: music_assets.0.get("nostalgia").unwrap().clone(),
                settings: PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Loop,
                    ..default()
                },
            },
            AudioFadeIn,
            MenuBackgroundMusic,
        ));
    }
}

pub fn despawn_background_music(
    mut commands: Commands,
    music_query: Query<Entity, With<MenuBackgroundMusic>>,
) {
    for entity in music_query.iter() {
        commands.entity(entity).despawn();
    }
}
