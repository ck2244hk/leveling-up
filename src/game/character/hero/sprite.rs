use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    animation::{
        component::{AnimationIndices, AnimationTimer},
        Sliding,
    },
    game::{
        battle_scene::{BattleHeroSprite, BattleHeroSpriteStand},
        character::component::{
            Hero, HeroClass, HeroSprite, HeroSpriteBundle, HeroSpriteNodeBundle,
        },
        CurrentMovement, DecendingHero, FacingSide, Player, PLAYER_START_POSITION,
    },
    preload::sprites::{HeroAction, HeroActionHandles, HeroActionTextureAtLasHandles},
    state::SimulationState,
};

use super::{BaseStates, SpawnHeroEvent};

pub fn spawn_scale_w_sprite(
    mut player_query: Query<(&mut Transform, Entity), With<Player>>,
    hero_query: Query<(&BaseStates, &Parent), Changed<BaseStates>>,
) {
    let Ok((mut transform, entity)) = player_query.get_single_mut() else {
        return;
    };
    for (base_state, parent) in hero_query.iter() {
        if parent.get() != entity {
            continue;
        }

        *transform = transform.with_scale(Vec3::new(
            0.3_f32.max(base_state.lv_f32() * 0.05),
            0.3_f32.max(base_state.lv_f32() * 0.05),
            1.,
        ));
    }
}

pub fn spawn_decending_hero(
    mut commands: Commands,
    hero_sprite: Res<HeroActionHandles>,
    hero_texture_atlas: Res<HeroActionTextureAtLasHandles>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let height = window_query.single().height();
    info!("Spawn decending, window heght: {}", height);

    commands.spawn((
        Name::new("Decending Hero"),
        DecendingHero,
        SpriteBundle {
            texture: hero_sprite[&HeroAction::Decending].clone(),
            transform: Transform::from_xyz(PLAYER_START_POSITION.x, height - 60., 999.),
            ..default()
        },
        TextureAtlas {
            layout: hero_texture_atlas[&HeroAction::Decending].clone(),
            ..default()
        },
        Sliding::new(
            Vec2 {
                x: PLAYER_START_POSITION.x,
                y: PLAYER_START_POSITION.y,
            },
            false,
            1.,
            Vec2 {
                x: PLAYER_START_POSITION.x,
                y: 0.,
            },
        ),
        AnimationIndices::new(3, true),
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

pub fn despawn_decending(
    mut commands: Commands,
    hero_query: Query<(&Sliding, Entity), With<DecendingHero>>,
    mut event: EventWriter<SpawnHeroEvent>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    for (sliding, entity) in hero_query.iter() {
        if sliding.is_finished {
            info!("Desspawn decending");
            commands.entity(entity).despawn();
            event.send(SpawnHeroEvent {});
            next_simulation_state.set(SimulationState::Running);
        }
    }
}

pub fn spawn_hero_sprite(
    mut commands: Commands,
    mut player_query: Query<Entity, With<Player>>,
    hero_sprite: Res<HeroActionHandles>,
    hero_texture_atlas: Res<HeroActionTextureAtLasHandles>,

    mut spawn_event: EventReader<SpawnHeroEvent>,
) {
    for _ in spawn_event.read() {
        // println!("{:?}", hero_sprite.0);
        let player_entity = player_query.get_single_mut().expect("No Player is spawned");
        // let hero_id: Entity = match ev.0 {
        let hero_id = commands
            .spawn(build_hero_sprite(
                hero_texture_atlas[&HeroAction::Idle].clone(),
                hero_sprite[&HeroAction::Idle].clone(),
                HeroClass::Paladin,
                AnimationIndices {
                    first: 0,
                    last: 9,
                    will_repeat: true,
                },
            ))
            .id();

        commands.entity(player_entity).add_child(hero_id);
    }
    // }
}

pub fn spawn_hero_sprite_battle(
    mut commands: Commands,
    mut player_query: Query<Entity, With<Player>>,
    hero_query: Query<(&Parent, &HeroClass), With<Hero>>,

    hero_sprite: Res<HeroActionHandles>,
    hero_texture_atlas: Res<HeroActionTextureAtLasHandles>,
    battle_scene_query: Query<Entity, Added<BattleHeroSpriteStand>>,
) {
    // println!("{:?}", hero_sprite.0);
    for battle_scene in battle_scene_query.iter() {
        let player_entity = player_query.get_single_mut().expect("No Player is spawned");
        for (parent, hero_class) in hero_query.iter() {
            if parent.get() == player_entity {
                let hero_bundle = commands
                    .spawn(build_hero_sprite_node(
                        hero_texture_atlas[&HeroAction::Idle].clone(),
                        hero_sprite[&HeroAction::Idle].clone(),
                        *hero_class,
                    ))
                    .id();

                commands.entity(battle_scene).add_child(hero_bundle);
                info!("Spawned Battle Hero Sprite");
            }
        }
    }
}

// pub fn animate_sprite(
//     time: Res<Time>,
//     mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
// ) {
//     for (indices, mut timer, mut sprite) in &mut query {
//         timer.tick(time.delta());
//         if timer.just_finished() {
//             sprite.index = if sprite.index == indices.last {
//                 indices.first
//             } else {
//                 sprite.index + 1
//             };
//         }
//     }
// }

pub fn update_sprite_anime(
    mut hero_query: Query<
        (
            &mut TextureAtlas,
            &mut Handle<Image>,
            &mut AnimationIndices,
            &HeroClass,
        ),
        With<HeroSprite>,
    >,
    player_query: Query<
        (&CurrentMovement, &Children),
        (Changed<CurrentMovement>, With<Player>, Without<HeroSprite>),
    >,
    hero_sprite: Res<HeroActionHandles>,
    hero_texture_atlas: Res<HeroActionTextureAtLasHandles>,
) {
    if let Ok((current_movement, children)) = player_query.get_single() {
        info!("Current Movment Changed to {:?}", current_movement);
        for child in children.iter() {
            if let Ok((mut sprite, mut image_handle, mut indices, hero_class)) =
                hero_query.get_mut(*child)
            {
                info!("Current Hero {:?}", hero_class);
                match current_movement.0 {
                    crate::game::Movement::Idle => {
                        info!("changing to idle position");
                        let animation_indices = AnimationIndices {
                            first: 0,
                            last: 9,
                            will_repeat: true,
                        };

                        indices.set_if_neq(animation_indices);

                        image_handle.set_if_neq(hero_sprite[&HeroAction::Idle].clone());
                        *sprite = TextureAtlas {
                            layout: hero_texture_atlas[&HeroAction::Idle].clone(),
                            index: 0,
                        };
                    }

                    crate::game::Movement::Run => {
                        info!("changing to run position");
                        // Use only the subset of sprites in the sheet that make up the run animation
                        let animation_indices = AnimationIndices {
                            first: 0,
                            last: 9,
                            will_repeat: true,
                        };

                        image_handle.set_if_neq(hero_sprite[&HeroAction::Run].clone());
                        *sprite = TextureAtlas {
                            layout: hero_texture_atlas[&HeroAction::Run].clone(),
                            index: 0,
                        };

                        indices.set_if_neq(animation_indices);
                    }
                }
            }
        }
    }
}

pub fn update_sprite_flip(
    mut hero_query: Query<&mut Transform, With<HeroSprite>>,
    player_query: Query<
        (&Children, &FacingSide),
        (With<FacingSide>, With<Player>, Without<HeroSprite>),
    >,
) {
    if let Ok((children, facing_side)) = player_query.get_single() {
        for child in children {
            if let Ok(mut transform) = hero_query.get_mut(*child) {
                match facing_side {
                    FacingSide::Right => {
                        transform.set_if_neq(transform.with_rotation(Quat::from_rotation_y(0.)));
                    }
                    FacingSide::Left => {
                        // transform.set_if_neq(transform.with_rotation(Quat::from_rotation_y(180.)));
                        transform.rotation = Quat::from_euler(
                            EulerRot::XYZ,
                            (-180.0_f32).to_radians(),
                            (0.0_f32).to_radians(),
                            (-180.0_f32).to_radians(),
                        )
                    }
                }
            }
        }
    }
}

// private
fn build_hero_sprite(
    texture_handle: Handle<TextureAtlasLayout>,
    image_handle: Handle<Image>,
    hero_class: HeroClass,
    animation_indices: AnimationIndices,
) -> HeroSpriteBundle {
    HeroSpriteBundle {
        name: Name::new("Sprite"),
        hero_sripte: HeroSprite,
        sprite_sheet_bundle: SpriteBundle {
            texture: image_handle.clone(),
            transform: Transform::from_xyz(0., 20., 5.),
            ..default()
        },
        texture_atlas: TextureAtlas {
            layout: texture_handle.clone(),
            ..default()
        },
        animation_indices,
        animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        hero_class,
    }
}

// private
fn build_hero_sprite_node(
    texture_handle: Handle<TextureAtlasLayout>,
    image_handle: Handle<Image>,
    hero_class: HeroClass,
) -> HeroSpriteNodeBundle {
    HeroSpriteNodeBundle {
        name: Name::new("Sprite"),
        // action: BattleHeroAction::default(),
        hero_sripte: BattleHeroSprite,
        sprite_sheet_bundle: ImageBundle {
            image: UiImage {
                color: Color::WHITE,
                texture: image_handle.clone(),
                flip_x: false,
                flip_y: false,
            },

            style: Style {
                position_type: PositionType::Absolute,

                width: Val::Percent(100.),
                height: Val::Percent(100.),

                ..default()
            },
            ..default()
        },

        hero_class,
        texture_atlas: TextureAtlas {
            layout: texture_handle,
            index: 0,
        },
    }
}
