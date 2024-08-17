use bevy::prelude::*;

use bevy_rapier2d::pipeline::CollisionEvent;
use rand::Rng;

use crate::animation::component::{AnimationIndices, AnimationTimer};
use crate::game::battle_scene::BattleMonster;
use crate::game::preload::{SensorBundle, Weather};
use crate::game::PlayerEnv;
use crate::model::monster::MonsterData;
use crate::model::sub::Element;
use crate::model::Id;
use crate::state::{OverlayCombatState, Screen, SimulationState};

use crate::game::field::EncounterEvent;
use crate::game::{battle::event::BattleEvent, Player};
use crate::preload::sprites::{MonsterAsset, MonsterHandles, MonsterTextureAtLasHandles};

use super::component::{
    BaseStates, BattleMonsterSprite, Boss, BossRespawnBlinker, BossRespawnTimer, DespawnBossFlag,
    HealthPoint, MonsterSprite, MonsterSpriteBundle, MonsterSpriteNodeBundle, MonsterType,
};

use bevy_ecs_ldtk::{
    app::LdtkEntityAppExt, ldtk::ldtk_fields::LdtkFields, EntityInstance, LdtkEntity,
};

use crate::helper::Toggle;
use rand::seq::SliceRandom;

// #[derive(Event)]
// pub struct MonsterEvent(pub Entity);

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_monster
                .run_if(in_state(SimulationState::Running))
                .run_if(in_state(Screen::Playing)),
        )
        .add_systems(
            Update,
            (
                insert_boss_data,
                spawn_monster_sprite_battle,
                spawn_boss_sprite,
                detect_boss_fight,
                update_respawn_boss,
                blink_respawning_boss,
                flag_despawn_boss,
            ),
        )
        .add_systems(
            OnEnter(OverlayCombatState::Closed),
            (despawn_monster, respawn_boss, despawn_boss),
        )
        .add_systems(OnExit(Screen::Playing), despawn_monsters)
        .register_ldtk_entity::<BossBundle>("Boss");
    }
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct BossBundle {
    #[from_entity_instance]
    pub monster_bundle: MonsterBundle,
    #[from_entity_instance]
    pub sensor_bundle: SensorBundle,
    pub boss: Boss,
}

#[derive(Component, Clone, Default)]
pub struct Monster;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct MonsterBundle {
    id: Id,
    name: Name,
    base: BaseStates,
    monster_type: MonsterType,
}

impl From<&EntityInstance> for MonsterBundle {
    fn from(entity_instance: &EntityInstance) -> MonsterBundle {
        // ladder
        let lv = *entity_instance
            .get_int_field("Level")
            .expect("expected entity to have non-nullable level int field") as u32;

        let name = entity_instance
            .get_string_field("Name")
            .expect("expected entity to have non-nullable name string field")
            .clone();

        let id = entity_instance
            .get_int_field("Id")
            .expect("expected entity to have non-nullable id int field")
            .clone() as u32;

        MonsterBundle {
            id: Id::new(id),
            name: Name::new(name.clone()),
            base: BaseStates::new_monster(lv),
            monster_type: MonsterType::from_str("HellBat"),
        }
    }
}

pub fn spawn_monster(
    mut commands: Commands,
    mut encounter_event_reader: EventReader<EncounterEvent>,
    monster: Res<Assets<MonsterData>>,
    player_query: Query<&PlayerEnv, With<Player>>,
) {
    for ev in encounter_event_reader.read() {
        let Ok(player_env) = player_query.get_single() else {
            continue;
        };
        let mut rng = rand::thread_rng();

        let monster_id = ev.monster_id.choose(&mut rand::thread_rng()).unwrap_or(&0);

        let (_, monster) = monster
            .iter()
            .find(|(_, data)| data.id == *monster_id as u32)
            .expect(&format!("No Monster Found: ID {}", monster_id));

        let mut new_monster = monster.clone();

        new_monster.element = match player_env.0 {
            Some(Weather::Sunny) => Element::Fire,
            Some(Weather::Rainy) => Element::Water,
            Some(Weather::Muddy) => Element::Earth,
            None => monster.element.clone(),
        };

        build_monster(ev.lv + rng.gen_range(0..5), &new_monster, &mut commands);

        info!("Monster Spawned");
    }
}

pub fn insert_boss_data(
    mut commands: Commands,
    monster_query: Query<(Entity, &Id), Added<Boss>>,
    monster: Res<Assets<MonsterData>>,
) {
    for (entity, monster_id) in monster_query.iter() {
        let (_, monster) = monster
            .iter()
            .find(|(_, data)| data.id == monster_id.get())
            .expect(&format!("No Monster Found: ID {}", monster_id.get()));

        commands.entity(entity).insert(monster.clone());
    }
}

fn build_monster(lv: u32, monster: &MonsterData, commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Name::new("Monster".to_string()),
            Monster {},
            HealthPoint::monster(),
            BaseStates::new_monster(lv),
            MonsterType::Monster1,
            monster.clone(),
        ))
        .id()
}

pub fn spawn_monster_sprite_battle(
    mut commands: Commands,
    monster_query: Query<&MonsterType, With<Monster>>,
    monster_sprite: Res<MonsterHandles>,
    monster_texture_atlas: Res<MonsterTextureAtLasHandles>,
    battle_scene_query: Query<(Entity, &BattleMonster), Added<BattleMonster>>,
) {
    // println!("{:?}", monster_sprite.0);
    for (battle_scene, monster_entity) in battle_scene_query.iter() {
        if let Ok(monster_type) = monster_query.get(monster_entity.monster) {
            let monster_bundle = match monster_type {
                MonsterType::Monster1 => commands
                    .spawn(build_monster_sprite_node(
                        monster_texture_atlas[&MonsterAsset::Monster1].clone(),
                        monster_sprite[&MonsterAsset::Monster1].clone(),
                        *monster_type,
                        AnimationIndices {
                            first: 0,
                            last: 1,
                            will_repeat: true,
                        },
                    ))
                    .id(),
                MonsterType::HellBat => commands
                    .spawn(build_monster_sprite_node(
                        monster_texture_atlas[&MonsterAsset::Monster2].clone(),
                        monster_sprite[&MonsterAsset::Monster2].clone(),
                        *monster_type,
                        AnimationIndices {
                            first: 0,
                            last: 1,
                            will_repeat: true,
                        },
                    ))
                    .id(),
                MonsterType::Monster3 => commands
                    .spawn(build_monster_sprite_node(
                        monster_texture_atlas[&MonsterAsset::Monster3].clone(),
                        monster_sprite[&MonsterAsset::Monster3].clone(),
                        *monster_type,
                        AnimationIndices {
                            first: 0,
                            last: 1,
                            will_repeat: true,
                        },
                    ))
                    .id(),
            };

            commands.entity(battle_scene).add_child(monster_bundle);
            info!("Spawned Battle Monster Sprite");
        }
    }
}

pub fn spawn_boss_sprite(
    mut commands: Commands,
    monster_query: Query<(Entity, &MonsterType), Added<Boss>>,
    monster_sprite: Res<MonsterHandles>,
    monster_texture_atlas: Res<MonsterTextureAtLasHandles>,
) {
    // println!("{:?}", monster_sprite.0);
    for (boss_entity, monster_type) in monster_query.iter() {
        let monster_bundle = match monster_type {
            MonsterType::Monster1 => commands
                .spawn(build_boss_sprite(
                    monster_texture_atlas[&MonsterAsset::Monster1].clone(),
                    monster_sprite[&MonsterAsset::Monster1].clone(),
                    *monster_type,
                    AnimationIndices {
                        first: 0,
                        last: 1,
                        will_repeat: true,
                    },
                ))
                .id(),
            MonsterType::HellBat => commands
                .spawn(build_boss_sprite(
                    monster_texture_atlas[&MonsterAsset::Monster2].clone(),
                    monster_sprite[&MonsterAsset::Monster2].clone(),
                    *monster_type,
                    AnimationIndices {
                        first: 0,
                        last: 1,
                        will_repeat: true,
                    },
                ))
                .id(),
            MonsterType::Monster3 => commands
                .spawn(build_boss_sprite(
                    monster_texture_atlas[&MonsterAsset::Monster3].clone(),
                    monster_sprite[&MonsterAsset::Monster3].clone(),
                    *monster_type,
                    AnimationIndices {
                        first: 0,
                        last: 1,
                        will_repeat: true,
                    },
                ))
                .id(),
        };

        commands.entity(boss_entity).add_child(monster_bundle);
        info!("Spawned Boss Sprite");
    }
}

pub fn despawn_monster(
    mut commands: Commands,
    monster_query: Query<Entity, (With<Monster>, Without<Boss>)>,
) {
    for monster in monster_query.iter() {
        commands.entity(monster).despawn_recursive();
    }

    println!("despawned monsters");
}

pub fn flag_despawn_boss(
    mut commands: Commands,
    boss_query: Query<Entity, With<Boss>>,
    mut battle_event: EventReader<BattleEvent>,
) {
    for ev in battle_event.read() {
        if ev.is_player_victory {
            let Ok(entity) = boss_query.get(ev.monster_entity) else {
                continue;
            };

            commands.entity(entity).insert(DespawnBossFlag {});
        }
    }
}

pub fn despawn_boss(
    mut commands: Commands,
    boss_query: Query<Entity, (With<Boss>, With<DespawnBossFlag>)>,
) {
    let Ok(entity) = boss_query.get_single() else {
        return;
    };
    info!("Despawning a Boss");

    commands.entity(entity).despawn_recursive();
}

pub fn respawn_boss(
    mut commands: Commands,
    boss_query: Query<Entity, (With<Boss>, With<Monster>)>,
) {
    for boss in boss_query.iter() {
        info!("Respawning a Boss");
        commands
            .entity(boss)
            .remove::<Monster>()
            .insert((BossRespawnTimer::default(), BossRespawnBlinker::default()));
    }
}

pub fn update_respawn_boss(
    mut commands: Commands,
    mut boss_query: Query<(&mut BossRespawnTimer, Entity), With<Boss>>,
    time: Res<Time>,
) {
    for (mut boss_timer, entity) in boss_query.iter_mut() {
        if boss_timer.0.tick(time.delta()).just_finished() {
            commands
                .entity(entity)
                .remove::<BossRespawnTimer>()
                .remove::<BossRespawnBlinker>();
        }
    }
}

pub fn blink_respawning_boss(
    mut boss_query: Query<(&mut BossRespawnBlinker, &mut Visibility), With<Boss>>,
    time: Res<Time>,
) {
    for (mut boss_blinker, mut visibility) in boss_query.iter_mut() {
        if boss_blinker.0.tick(time.delta()).just_finished() {
            visibility.toggle();
        }
    }
}

pub fn despawn_monsters(mut commands: Commands, monster_query: Query<Entity, With<Monster>>) {
    for monster in monster_query.iter() {
        commands.entity(monster).despawn_recursive();
    }

    println!("despawned monsters");
}

fn build_monster_sprite_node(
    texture_handle: Handle<TextureAtlasLayout>,
    image_handle: Handle<Image>,
    monster_type: MonsterType,
    animation_indices: AnimationIndices,
) -> MonsterSpriteNodeBundle {
    MonsterSpriteNodeBundle {
        name: Name::new("Sprite"),
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
        texture_atlas: TextureAtlas {
            layout: texture_handle.clone(),
            index: 0,
        },
        animation_indices,
        animation_timer: AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
        monster_type,
        ui_monster_sprite: BattleMonsterSprite {},
    }
}

fn build_boss_sprite(
    texture_handle: Handle<TextureAtlasLayout>,
    image_handle: Handle<Image>,
    monster_type: MonsterType,
    animation_indices: AnimationIndices,
) -> MonsterSpriteBundle {
    MonsterSpriteBundle {
        name: Name::new("Sprite"),
        monster_sprite: MonsterSprite {},
        sprite_sheet_bundle: SpriteBundle {
            texture: image_handle.clone(),
            ..default()
        },
        texture_atlas: TextureAtlas {
            layout: texture_handle.clone(),
            index: 0,
        },
        animation_indices,
        animation_timer: AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
        monster_type,
    }
}

pub fn detect_boss_fight(
    mut commands: Commands,
    mut player: Query<&mut Player>,
    mut bosses: Query<Entity, (With<Boss>, Without<BossRespawnTimer>)>,
    mut collisions: EventReader<CollisionEvent>,
) {
    for collision in collisions.read() {
        if let CollisionEvent::Started(collider_a, collider_b, _) = collision {
            info!(
                "Collision Start: collider _a: {:?}, collider_b: {:?}",
                collider_a, collider_b
            );
            if let (Ok(_player), Ok(monster_entity)) =
                (player.get_mut(*collider_a), bosses.get_mut(*collider_b))
            {
                commands.entity(monster_entity).insert(Monster);
            }
            if let (Ok(_player), Ok(monster_entity)) =
                (player.get_mut(*collider_b), bosses.get_mut(*collider_a))
            {
                commands.entity(monster_entity).insert(Monster);
            };
        }
    }
}
