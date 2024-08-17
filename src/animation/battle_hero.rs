use bevy::prelude::*;

use crate::{
    animation::ui_standard::AnimateTerminateUnit,
    game::{
        battle::event::{AttackEvent, BattleEvent},
        overlay::battle_scene::BattleHeroSprite,
    },
    preload::sprites::{HeroAction, HeroActionHandles, HeroActionTextureAtLasHandles},
    state::OverlayCombatState,
};

use super::{
    component::{AnimationIndices, AnimationTimer},
    ui_standard::{Blinking, UiSliding},
};

#[derive(Component, Default, PartialEq, Reflect)]
pub enum BattleHeroAction {
    #[default]
    Idle,
    Attack(usize),
    Death,
}

#[derive(Bundle)]
pub struct BattleHeroActionBundle {
    timer: AnimationTimer,
    index: AnimationIndices,
    image: UiImage,
    texture_atlas: TextureAtlas,
}

impl BattleHeroActionBundle {
    fn idle(image: Handle<Image>, texture_atlas: Handle<TextureAtlasLayout>) -> Self {
        BattleHeroActionBundle {
            timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            index: AnimationIndices::new(9, true),
            image: UiImage::new(image),
            texture_atlas: TextureAtlas {
                layout: texture_atlas,
                index: 0,
            },
        }
    }
    fn attack(image: Handle<Image>, texture_atlas: Handle<TextureAtlasLayout>) -> Self {
        BattleHeroActionBundle {
            timer: AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            index: AnimationIndices::new(3, false),
            image: UiImage::new(image),
            texture_atlas: TextureAtlas {
                layout: texture_atlas,
                index: 0,
            },
        }
    }

    fn death(image: Handle<Image>, texture_atlas: Handle<TextureAtlasLayout>) -> Self {
        BattleHeroActionBundle {
            timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            index: AnimationIndices::new(9, false),
            image: UiImage::new(image),
            texture_atlas: TextureAtlas {
                layout: texture_atlas,
                index: 0,
            },
        }
    }
}

pub struct BattleHeroAnimationPlugin;

impl Plugin for BattleHeroAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                trigger_battle_attack_anime,
                handle_action,
                handle_battle_hero_anime_update,
                spawn_battle_hero_anime,
                handle_battle_end,
            )
                .run_if(in_state(OverlayCombatState::Opened)),
        )
        .register_type::<BattleHeroAction>();
        //.add_system(spawn_battle_hero_anime.system());
    }
}

pub fn spawn_battle_hero_anime(
    mut commands: Commands,
    hero_query: Query<Entity, Added<BattleHeroSprite>>,
    hero_sprite: Res<HeroActionHandles>,
    hero_texture_atlas: Res<HeroActionTextureAtLasHandles>,
) {
    for hero in &hero_query {
        info!("Spawning Idle Timer");
        commands.entity(hero).insert((
            BattleHeroAction::Idle,
            BattleHeroActionBundle::idle(
                hero_sprite[&HeroAction::Idle].clone(),
                hero_texture_atlas[&HeroAction::Idle].clone(),
            ),
        ));
    }
}

pub fn trigger_battle_attack_anime(
    mut commands: Commands,
    mut attack_event: EventReader<AttackEvent>,
    mut hero_query: Query<
        (
            Entity,
            &mut BattleHeroAction,
            Option<&mut Blinking>,
            Option<&mut UiSliding>,
        ),
        With<BattleHeroSprite>,
    >,
) {
    for ev in attack_event.read() {
        for (entity, mut hero, blinking, sliding) in &mut hero_query {
            if ev.record.is_player_turn {
                hero.set_if_neq(BattleHeroAction::Attack(0));
            } else {
                info!("Hero were attacked");
                let sliding_anime = UiSliding::new(UiRect::left(Val::Px(-30.)), true, 3.);
                if let Some(mut sliding) = sliding {
                    *sliding = sliding_anime;
                } else {
                    commands.entity(entity).insert(sliding_anime);
                }

                let mut blink_anime = Blinking::new(0.2, AnimateTerminateUnit::Counter(5));
                blink_anime.reset();
                if let Some(mut blinking) = blinking {
                    *blinking = blink_anime;
                } else {
                    commands.entity(entity).insert(blink_anime);
                }
            }
        }
    }
}

pub fn handle_action(
    mut sprite_query: Query<
        (
            &mut BattleHeroAction,
            &AnimationIndices,
            &TextureAtlas,
            &UiSliding,
        ),
        With<BattleHeroSprite>,
    >,
) {
    for (mut action, index, texture_atlas, sliding) in &mut sprite_query {
        match *action {
            BattleHeroAction::Attack(0) => {
                if sliding.is_finished {
                    *action = BattleHeroAction::Attack(1);
                }
            }
            BattleHeroAction::Attack(1..) => {
                if index.last == texture_atlas.index {
                    *action = BattleHeroAction::Idle;
                }
            }
            _ => {}
        }
    }
}

pub fn handle_battle_hero_anime_update(
    mut commands: Commands,
    mut sprite_query: Query<
        (
            Entity,
            &BattleHeroAction,
            &mut AnimationTimer,
            &mut AnimationIndices,
            &mut UiImage,
            &mut TextureAtlas,
            Option<&mut UiSliding>,
        ),
        Changed<BattleHeroAction>,
    >,
    hero_sprite: Res<HeroActionHandles>,
    hero_texture_atlas: Res<HeroActionTextureAtLasHandles>, // time: Res<Time>,
) {
    for (entity, action, mut timer, mut indices, mut image, mut texture_atlas, sliding) in
        &mut sprite_query
    {
        match *action {
            BattleHeroAction::Idle => {
                info!("Battle Inserting Idle");
                let bundle = BattleHeroActionBundle::idle(
                    hero_sprite[&HeroAction::Idle].clone(),
                    hero_texture_atlas[&HeroAction::Idle].clone(),
                );

                *timer = bundle.timer;
                *indices = bundle.index;
                *image = bundle.image;
                *texture_atlas = bundle.texture_atlas;
            }

            BattleHeroAction::Attack(0) => {
                info!("Battle Inserting Attack");
                let bundle = BattleHeroActionBundle::attack(
                    hero_sprite[&HeroAction::Attack].clone(),
                    hero_texture_atlas[&HeroAction::Attack].clone(),
                );

                let sliding_anime = UiSliding::new(
                    UiRect {
                        left: Val::Px(80.),
                        bottom: Val::Px(80.),
                        top: Val::ZERO,
                        right: Val::ZERO,
                    },
                    true,
                    3.,
                );
                if let Some(mut sliding) = sliding {
                    *sliding = sliding_anime;
                } else {
                    commands.entity(entity).insert(sliding_anime);
                }

                *timer = bundle.timer;
                *indices = bundle.index;
                *image = bundle.image;
                *texture_atlas = bundle.texture_atlas;
            }
            BattleHeroAction::Death => {
                info!("Battle Inserting Death");
                let bundle = BattleHeroActionBundle::death(
                    hero_sprite[&HeroAction::Death].clone(),
                    hero_texture_atlas[&HeroAction::Death].clone(),
                );

                *timer = bundle.timer;
                *indices = bundle.index;
                *image = bundle.image;
                *texture_atlas = bundle.texture_atlas;
            }
            _ => {
                info!("Battle Hero Sprite Moving Around");
            }
        }
    }
}

pub fn handle_battle_end(
    mut sprite_query: Query<&mut BattleHeroAction, With<BattleHeroSprite>>,
    mut battle_event: EventReader<BattleEvent>,
) {
    for ev in battle_event.read() {
        if !ev.is_player_victory {
            for mut action in sprite_query.iter_mut() {
                *action = BattleHeroAction::Death;
            }
        }
    }
}

// Change Event / Init Event -> Update
// Change
