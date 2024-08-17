use std::collections::VecDeque;

use bevy::{
    ecs::component::Component,
    math::{FloatExt, Vec2},
    reflect::Reflect,
    render::view::Visibility,
    ui::Val,
};

pub fn string_to_char(s: String) -> VecDeque<char> {
    s.chars().collect()
}

pub trait ValExtension {
    fn add(&mut self, num: f32);
    fn get(&self) -> f32;
    fn px_lerp(&self, rhs: Val, delta: f32) -> Val;
}

impl ValExtension for Val {
    fn add(&mut self, num: f32) {
        let _ = self.set(match *self {
            Val::Auto => Box::new(Val::Auto),
            Val::Px(x) => Box::new(Val::Px(x + num)),
            Val::Percent(x) => Box::new(Val::Percent(x + num)),
            Val::Vw(x) => Box::new(Val::Vw(x + num)),
            Val::Vh(x) => Box::new(Val::Vh(x + num)),
            Val::VMin(x) => Box::new(Val::VMin(x + num)),
            Val::VMax(x) => Box::new(Val::VMax(x + num)),
        });
    }

    fn get(&self) -> f32 {
        match self {
            Val::Auto => 0.,
            Val::Px(x) => *x,
            Val::Percent(x) => *x,
            Val::Vw(x) => *x,
            Val::Vh(x) => *x,
            Val::VMin(x) => *x,
            Val::VMax(x) => *x,
        }
    }

    fn px_lerp(&self, rhs: Val, delta: f32) -> Val {
        let current = self.get();

        let new = current.lerp(rhs.get(), delta);

        if new == 0. {
            Val::Auto
        } else {
            Val::Px(new)
        }

        // match rhs {
        //     Val::Auto | Val::Px(_) => {}
        //     _ => {
        //         warn!("Wrong unit in Val lerping");
        //     }
        // }

        // match self {
        //     Val::Auto => Val::Px(0_f32.lerp(rhs.get(), delta)),
        //     Val::Px(x) => Val::Px(x.lerp(rhs.get(), delta)),
        //     _ => {
        //         warn!("Wrong unit in Val lerping");
        //         Val::Auto
        //     }
        // }
    }
}

pub trait Toggle {
    fn toggle(&mut self) -> ();
}

impl Toggle for Visibility {
    fn toggle(&mut self) -> () {
        let _ = self.set(match *self {
            Visibility::Inherited => Box::new(Visibility::Hidden),
            Visibility::Hidden => Box::new(Visibility::Visible),
            Visibility::Visible => Box::new(Visibility::Hidden),
        });
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component, Reflect)]
pub struct Zooming(pub f32);

impl Default for Zooming {
    fn default() -> Self {
        Zooming(0.1)
    }
}

#[derive(Component)]
pub struct SmoothDamp {
    pub acceleration: Vec2,
    smooth_time: f32,
    max_speed: f32,
    offset: Vec2,
}

impl SmoothDamp {
    pub fn new() -> Self {
        Self {
            acceleration: Vec2::splat(0.),
            smooth_time: 1.,
            max_speed: 9999.,
            offset: Vec2::splat(0.),
        }
    }

    pub fn update_acceleration(
        &mut self,
        current: Vec2,
        target: Vec2,
        current_speed: f32,
        delta_time: f32,
    ) {
        self.acceleration = self.calculate(current, target, current_speed, delta_time, self.offset);
    }

    // this return the acceleration needed for next frame
    fn calculate(
        &self,
        current: Vec2,
        target: Vec2,
        current_speed: f32,
        delta_time: f32,
        offset: Vec2,
    ) -> Vec2 {
        let target_w_offset = target + offset;
        let omega = 2. / self.smooth_time;

        let x = omega * delta_time;
        let exp = 1. / (1. + x + 0.48 * x * x + 0.235 * x * x * x);
        let mut change = current - target_w_offset;

        // Clamp maximum speed
        let max_change = Vec2::splat(self.max_speed * self.smooth_time);
        change = change.clamp(-max_change, max_change);

        let temp = (current_speed + omega * change) * delta_time;
        let next_speed = (current_speed - omega * temp) * exp;

        next_speed - current_speed
    }
}
