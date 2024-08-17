use bevy::prelude::*;

use crate::{
    animation::ui_standard::AnimateTerminateUnit,
    game::{
        battle::event::{AttackEvent, BattleEvent},
        character::component::BattleMonsterSprite,
    },
    state::OverlayCombatState,
};

use super::{
    battle_hero::BattleHeroAction,
    component::{AnimationIndices, AnimationTimer},
    ui_standard::{Blinking, UiSliding, VerticalCollapse},
};

#[derive(Component, Default, PartialEq, Reflect)]
pub enum BattleMonsterAction {
    #[default]
    Idle,
    Attack(usize),
}

#[derive(Bundle)]
pub struct BattleMonsterActionBundle {
    timer: AnimationTimer,
    index: AnimationIndices,
    image: UiImage,
    texture_atlas: TextureAtlas,
}

impl BattleMonsterActionBundle {
    // fn idle(image: Handle<Image>, texture_atlas: Handle<TextureAtlasLayout>) -> Self {
    //     BattleMonsterActionBundle {
    //         timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    //         index: AnimationIndices::new(9, true),
    //         image: UiImage::new(image),
    //         texture_atlas: TextureAtlas {
    //             layout: texture_atlas,
    //             index: 0,
    //         },
    //     }
    // }
    // fn attack(image: Handle<Image>, texture_atlas: Handle<TextureAtlasLayout>) -> Self {
    //     BattleMonsterActionBundle {
    //         timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    //         index: AnimationIndices::new(3, true),
    //         image: UiImage::new(image),
    //         texture_atlas: TextureAtlas {
    //             layout: texture_atlas,
    //             index: 0,
    //         },
    //     }
    // }
}

pub struct BattleMonsterAnimationPlugin;

impl Plugin for BattleMonsterAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_being_attacked,
                spawn_battle_monster_anime,
                trigger_battle_attack_anime,
                handle_battle_end,
            )
                .run_if(in_state(OverlayCombatState::Opened)),
        );
        // .register_type::<BattleMonsterAction>();
        //.add_system(spawn_battle_monster_anime.system());
    }
}

pub fn spawn_battle_monster_anime(
    mut commands: Commands,
    monster_query: Query<Entity, Added<BattleMonsterSprite>>,
) {
    for monster in &monster_query {
        info!("Spawning Idle Timer");
        commands
            .entity(monster)
            .insert(Blinking::new(0.3, AnimateTerminateUnit::Counter(4)));
    }
}

pub fn trigger_battle_attack_anime(
    mut commands: Commands,
    mut attack_event: EventReader<AttackEvent>,
    mut monster_query: Query<(Entity, Option<&mut UiSliding>), With<BattleMonsterSprite>>,
) {
    for ev in attack_event.read() {
        for (monster, sliding) in &mut monster_query {
            if !ev.record.is_player_turn {
                let sliding_anime = UiSliding::new(
                    UiRect {
                        bottom: Val::Px(-100.),
                        left: Val::Px(-100.),
                        ..default()
                    },
                    true,
                    5.,
                );

                if let Some(mut sliding) = sliding {
                    *sliding = sliding_anime;
                } else {
                    commands.entity(monster).insert(sliding_anime);
                }
            }
        }
    }
}

pub fn handle_being_attacked(
    mut commands: Commands,
    mut sprite_query: Query<
        (Entity, &mut Blinking, Option<&mut UiSliding>),
        With<BattleMonsterSprite>,
    >,

    hero_query: Query<&BattleHeroAction, Changed<BattleHeroAction>>,
) {
    for hero in hero_query.iter() {
        if *hero == BattleHeroAction::Attack(1) {
            for (entity, mut blinking, sliding) in sprite_query.iter_mut() {
                let sliding_anime = UiSliding::new(UiRect::left(Val::Px(30.)), true, 3.);
                if let Some(mut sliding) = sliding {
                    *sliding = sliding_anime;
                } else {
                    commands.entity(entity).insert(sliding_anime);
                }

                blinking.reset();
            }
        }
    }
}

pub fn handle_battle_end(
    mut commands: Commands,
    mut sprite_query: Query<(Entity, Option<&mut UiSliding>), With<BattleMonsterSprite>>,
    mut battle_event: EventReader<BattleEvent>,
) {
    for ev in battle_event.read() {
        if ev.is_player_victory {
            if let Ok((entity, sliding)) = sprite_query.get_single_mut() {
                commands.entity(entity).insert(VerticalCollapse);
                let sliding_anime = UiSliding::new(UiRect::bottom(Val::Px(50.)), false, 1.);
                if let Some(mut sliding) = sliding {
                    *sliding = sliding_anime;
                } else {
                    commands.entity(entity).insert(sliding_anime);
                }
            }
        }
    }
}

// pub fn handle_battle_monster_anime_update(
//     mut commands: Commands,
//     mut sprite_query: Query<
//         (
//             Entity,
//             &BattleMonsterAction,
//             &mut AnimationTimer,
//             &mut AnimationIndices,
//             &mut UiImage,
//             &mut TextureAtlas,
//         ),
//         Changed<BattleMonsterAction>,
//     >,
//     monster_sprite: Res<MonsterHandles>,
//     // time: Res<Time>,
// ) {
//     for (entity, action, mut timer, mut indices, mut image, mut texture_atlas) in &mut sprite_query
//     {
//         match *action {
//             BattleMonsterAction::Idle => {
//                 info!("Battle Inserting Idle");
//                 let bundle = BattleMonsterActionBundle::idle(
//                     monster_sprite[&MonsterAsset::].clone(),
//                     monster_sprite.0.get("Idle").unwrap().0.clone(),
//                 );

//                 *timer = bundle.timer;
//                 *indices = bundle.index;
//                 *image = bundle.image;
//                 *texture_atlas = bundle.texture_atlas;
//             }

//             BattleMonsterAction::Attack(0) => {
//                 info!("Battle Inserting Attack");
//                 let bundle = BattleMonsterActionBundle::attack(
//                     monster_sprite.0.get("Attack").unwrap().1.clone(),
//                     monster_sprite.0.get("Attack").unwrap().0.clone(),
//                 );

//                 *timer = bundle.timer;
//                 *indices = bundle.index;
//                 *image = bundle.image;
//                 *texture_atlas = bundle.texture_atlas;
//             }
//             _ => {
//                 info!("Battle Monster Sprite Moving Around");
//             } //     }
//         }
//     }
// }

// Change Event / Init Event -> Update
// Change
