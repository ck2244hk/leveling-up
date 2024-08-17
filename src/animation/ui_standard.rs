use bevy::prelude::*;
use bevy::ui::UiRect;

use crate::helper::ValExtension;

pub struct StandardAnimationPlugin;

impl Plugin for StandardAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_ui_silding,
                handle_blinking_anime,
                handle_vertical_collapse,
            ),
        )
        .register_type::<UiSliding>()
        .register_type::<Blinking>();
    }
}

#[derive(Component)]
pub struct VerticalCollapse;

#[derive(Reflect)]
pub enum AnimateTerminateUnit {
    Second(f32),
    Counter(usize),
    None,
}

#[derive(Component, Reflect)]
pub struct UiSliding {
    pub dest: UiRect,
    pub will_return: bool,
    pub speed: f32,
    pub offset: f32, // making destination reachable
    pub is_finished: bool,
}

impl UiSliding {
    pub fn new(dest: UiRect, will_return: bool, speed: f32) -> Self {
        Self {
            dest,
            will_return,
            speed,
            offset: 20.,
            is_finished: false,
        }
    }

    pub fn update_finished(&mut self, current: &Style) {
        let is_finished = current.left.get().abs() >= self.dest.left.get().abs() - self.offset
            && current.bottom.get().abs() >= self.dest.bottom.get().abs() - self.offset
            && current.top.get().abs() >= self.dest.top.get().abs() - self.offset
            && current.right.get().abs() >= self.dest.right.get().abs() - self.offset;
        if is_finished {
            self.is_finished = true;
        }
    }
}

#[derive(Component, Reflect)]
pub struct Blinking {
    pub count: usize,
    pub timer: Timer,
    pub terminate: AnimateTerminateUnit,
    pub last_display: Display,
    pub paused: bool,
}

impl Blinking {
    pub fn new(interval: f32, terminate: AnimateTerminateUnit) -> Self {
        Self {
            count: 0,
            timer: Timer::from_seconds(interval, TimerMode::Repeating),
            terminate,
            last_display: Display::None,
            paused: true,
        }
    }

    pub fn reset(&mut self) {
        self.count = 0;
        self.timer.reset();
        self.paused = false;
    }

    pub fn is_finished(&mut self) -> bool {
        match self.terminate {
            AnimateTerminateUnit::Second(second) => self.timer.elapsed_secs() >= second,
            AnimateTerminateUnit::Counter(count) => self.count >= count,
            AnimateTerminateUnit::None => false,
        }
    }
}

pub fn handle_ui_silding(mut sliding_query: Query<(&mut Style, &mut UiSliding)>, time: Res<Time>) {
    for (mut style, mut ui_sliding) in &mut sliding_query.iter_mut() {
        let dest = ui_sliding.dest;
        ui_sliding.update_finished(&style);

        if !ui_sliding.is_finished {
            style.left = style
                .left
                .px_lerp(dest.left, time.delta_seconds() * ui_sliding.speed);
            style.bottom = style
                .bottom
                .px_lerp(dest.bottom, time.delta_seconds() * ui_sliding.speed);

            style.top = style.top.px_lerp(dest.top, time.delta_seconds());
            style.right = style
                .right
                .px_lerp(dest.right, time.delta_seconds() * ui_sliding.speed);
        }

        if ui_sliding.is_finished && ui_sliding.will_return {
            style.left = style
                .left
                .px_lerp(Val::ZERO, time.delta_seconds() * ui_sliding.speed);
            style.top = style
                .top
                .px_lerp(Val::ZERO, time.delta_seconds() * ui_sliding.speed);
            style.bottom = style
                .bottom
                .px_lerp(Val::ZERO, time.delta_seconds() * ui_sliding.speed);
            style.right = style
                .right
                .px_lerp(Val::ZERO, time.delta_seconds() * ui_sliding.speed);
        }
    }
}

fn handle_blinking_anime(mut input_query: Query<(&mut Style, &mut Blinking)>, time: Res<Time>) {
    for (mut style, mut blink) in &mut input_query {
        if !blink.paused && !blink.is_finished() && blink.timer.tick(time.delta()).just_finished() {
            let last = blink.last_display;
            blink.last_display = style.display.clone();
            style.display = match style.display {
                Display::Flex => Display::None,
                Display::Grid => Display::None,
                Display::Block => Display::None,
                Display::None => last,
            };

            blink.count += 1;
        }
        if blink.is_finished() || blink.paused {
            style.display = match style.display {
                Display::Flex => Display::Flex,
                Display::Grid => Display::Grid,
                Display::None => blink.last_display,
                Display::Block => Display::Block,
            }
        }
    }
}

fn handle_vertical_collapse(
    mut input_query: Query<&mut Style, With<VerticalCollapse>>,
    time: Res<Time>,
) {
    for mut style in &mut input_query {
        style.height = style.height.px_lerp(Val::Px(0.), time.delta_seconds() * 5.);
    }
}
