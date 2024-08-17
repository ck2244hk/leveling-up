use std::time::Duration;

use bevy::prelude::*;

#[derive(Component, PartialEq, Eq)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
    pub will_repeat: bool,
}

impl AnimationIndices {
    pub fn new(last: usize, will_repeat: bool) -> AnimationIndices {
        AnimationIndices {
            first: 0,
            last,
            will_repeat,
        }
    }
}

#[derive(Component, Deref, DerefMut, PartialEq)]
pub struct AnimationTimer(pub Timer);

impl AnimationTimer {
    pub fn repeat(sec: f32) -> Self {
        Self(Timer::new(
            Duration::from_secs_f32(sec),
            TimerMode::Repeating,
        ))
    }

    pub fn once(sec: f32) -> Self {
        Self(Timer::new(Duration::from_secs_f32(sec), TimerMode::Once))
    }
}
