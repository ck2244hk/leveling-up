use bevy::{audio::Volume, prelude::*};

use crate::preload::audio::{AudioEffectAsset, AudioEffectHandles};

#[derive(Component)]
pub struct ButtonClickEffect;

pub struct AudioEffectPlugin;

impl Plugin for AudioEffectPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, interact_with_button);
    }
}

fn interact_with_button(
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<ButtonClickEffect>)>,
    mut commands: Commands,
    effect_assets: Res<AudioEffectHandles>,
) {
    for interaction in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                commands.spawn(AudioBundle {
                    source: effect_assets[&AudioEffectAsset::ButtonClick].clone(),
                    settings: PlaybackSettings {
                        volume: Volume::new(0.2),
                        mode: bevy::audio::PlaybackMode::Despawn,
                        ..default()
                    },
                });
            }

            _ => (),
        }
    }
}
