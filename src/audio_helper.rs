use bevy::{audio::Volume, prelude::*};

pub struct AudioHelperPlugin;

impl Plugin for AudioHelperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (fade_in, added_fade_in));
    }
}

#[derive(Component)]
pub struct AudioFadeIn;

pub fn added_fade_in(mut audio_query: Query<&mut PlaybackSettings, Added<AudioFadeIn>>) {
    for settings in audio_query.iter_mut() {
        settings.with_volume(settings.volume.new_volume(0.0));
    }
}

pub fn fade_in(
    mut audio_query: Query<(&mut PlaybackSettings, Entity), With<AudioFadeIn>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (settings, entity) in audio_query.iter_mut() {
        settings.with_volume(settings.volume.increased_volume(time.delta_seconds()));

        if settings.volume.get_level() >= 1. {
            commands.entity(entity).remove::<AudioFadeIn>();
        }
    }
}

pub trait VolumeExtension {
    fn increased_volume(&self, value: f32) -> Volume;
    fn new_volume(&self, value: f32) -> Volume;
    fn get_level(&self) -> f32;
}

impl VolumeExtension for Volume {
    fn increased_volume(&self, value: f32) -> Volume {
        Volume::new(0_f32.max(1_f32.min(self.get() + value)))
    }

    fn new_volume(&self, value: f32) -> Volume {
        Volume::new(value)
    }

    fn get_level(&self) -> f32 {
        self.get()
    }
}
