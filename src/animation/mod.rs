use battle_hero::BattleHeroAnimationPlugin;
use battle_monster::BattleMonsterAnimationPlugin;
use bevy::prelude::*;
use component::{AnimationIndices, AnimationTimer};
use ui_standard::StandardAnimationPlugin;

pub mod battle_hero;
pub mod battle_monster;
pub mod component;
pub mod ui_standard;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            BattleHeroAnimationPlugin,
            StandardAnimationPlugin,
            BattleMonsterAnimationPlugin,
        ))
        .add_systems(Update, (animate_sprite, handle_ui_silding, rotate));
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        if timer.tick(time.delta()).just_finished() {
            sprite.index = if sprite.index == indices.last && indices.will_repeat {
                indices.first
            } else {
                indices.last.min(sprite.index + 1)
            };
        }
    }
}

#[derive(Component)]
struct Rotate;

/// Rotates entities to demonstrate grid snapping.
fn rotate(time: Res<Time>, mut transforms: Query<&mut Transform, With<Rotate>>) {
    for mut transform in &mut transforms {
        let dt = time.delta_seconds();
        transform.rotate_z(dt);
    }
}

#[derive(Component, Reflect)]
pub struct Sliding {
    pub dest: Vec2,
    pub will_return: bool,
    pub speed: f32,
    pub offset: f32, // making destination reachable
    pub is_finished: bool,
    pub origin: Vec2,
}

impl Sliding {
    pub fn new(dest: Vec2, will_return: bool, speed: f32, origin: Vec2) -> Self {
        Self {
            dest,
            will_return,
            speed,
            offset: 5.,
            is_finished: false,
            origin,
        }
    }

    pub fn update_finished(&mut self, current: &Vec2) {
        let is_finished = (current.x - self.dest.x).abs() <= self.offset
            && (current.y - self.dest.y).abs() <= self.offset;

        // info!("current: {}, dest: {}", current, self.dest);
        if is_finished {
            self.is_finished = true;
        }
    }
}

pub fn handle_ui_silding(
    mut sliding_query: Query<(&mut Transform, &mut Sliding)>,
    time: Res<Time>,
) {
    for (mut transform, mut sliding) in &mut sliding_query.iter_mut() {
        let dest = sliding.dest;
        sliding.update_finished(&transform.translation.truncate());

        if !sliding.is_finished {
            transform.translation.x = transform
                .translation
                .x
                .lerp(dest.x, time.delta_seconds() * sliding.speed);
            transform.translation.y = transform
                .translation
                .y
                .lerp(dest.y, time.delta_seconds() * sliding.speed);
        }

        if sliding.is_finished && sliding.will_return {
            transform.translation.x = transform
                .translation
                .x
                .lerp(sliding.origin.x, time.delta_seconds() * sliding.speed);
            transform.translation.y = transform
                .translation
                .y
                .lerp(sliding.origin.y, time.delta_seconds() * sliding.speed);
        }
    }
}
