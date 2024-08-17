use crate::game::monster::Monster;
use crate::game::TerminalBundle;
use crate::model::monster::MonsterData;
use crate::model::sub::Element;
use crate::model::ItemType;
use crate::preload::images::{EquipmentHandles, UiImageAsset, UiImageHandles};
use crate::state::{OverlayCombatState, TerminalState};
use bevy::{prelude::*, ui::FocusPolicy};

use crate::game::battle::component::Combat;
use crate::game::event::{SpawnBattleSceneEvent, SpawnDropSceneEvent};

pub mod component;
mod update;

use component::*;
use update::*;

#[derive(Component)]
pub struct BattleScene;
pub struct BattleScenePlugin;

impl Plugin for BattleScenePlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_systems(
            //     Update,
            //     (spawn_battle_scene, spawn_lv_up_text, spawn_drop_scene),
            // )
            .add_systems(
                Update,
                (
                    update_hero_health_bar,
                    damping_hero_health_bar,
                    update_monster_health_bar,
                    damping_monster_health_bar,
                    update_flag_on_finish,
                )
                    .run_if(in_state(TerminalState::Combating))
                    .run_if(in_state(OverlayCombatState::Opened)),
            )
            .add_systems(
                Update,
                (
                    spawn_battle_scene,
                    spawn_lv_up_text,
                    spawn_drop_scene,
                    despawn_lv_up_text_position,
                    update_lv_up_text_position,
                )
                    .run_if(in_state(OverlayCombatState::Opened)),
            )
            .add_systems(OnExit(OverlayCombatState::Opened), despawn_battle_scene)
            .register_type::<HealthBar>();
    }
}

#[derive(Component)]
pub struct Arena;

#[derive(Component)]
pub struct BattleHeroSpriteStand;

#[derive(Component)]
pub struct BattleHeroSprite;

#[derive(Component)]
pub struct BattleMonster {
    pub monster: Entity,
}

// #[derive(Component)]
// pub struct BattleTerminalSpacer;

/// The basic UI node
///
/// Useful as a container for a variety of child nodes.
#[derive(Bundle, Clone, Debug)]
pub struct CustomNodeBundle {
    /// Describes the logical size of the node
    pub node: Node,
    /// Styles which control the layout (size and position) of the node and it's children
    /// In some cases these styles also affect how the node drawn/painted.
    pub style: Style,
    /// The background color, which serves as a "fill" for this node
    pub background_color: BackgroundColor,
    /// The color of the Node's border
    pub border_color: BorderColor,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,

    /// Indicates the depth at which the node should appear in the UI
    pub z_index: ZIndex,
}

impl Default for CustomNodeBundle {
    fn default() -> Self {
        CustomNodeBundle {
            // Transparent background
            background_color: Color::NONE.into(),
            border_color: Color::NONE.into(),
            node: Default::default(),
            style: Default::default(),
            focus_policy: Default::default(),
            z_index: Default::default(),
        }
    }
}

pub fn spawn_battle_scene(
    mut commands: Commands,
    mut battle_event_reader: EventReader<SpawnBattleSceneEvent>,
    battle_query: Query<&Combat>,
    monster_query: Query<(&Name, &MonsterData), With<Monster>>,
    ui_images: Res<UiImageHandles>,
) {
    for ev in battle_event_reader.read() {
        let Ok(battle) = battle_query.get(ev.battle) else {
            continue;
        };
        // let default_name = Name::new("Monster");
        let Ok((monster_name, monster_data)) = monster_query.get(ev.monster) else {
            continue;
        };

        let hero_health_bar = HealthBarUI::build_hero(&mut commands, battle.player_hp_remain);

        let monster_health_bar =
            HealthBarUI::build_monster(&mut commands, battle.monster_hp_remain, monster_name);

        let bg_image = match monster_data.element {
            Element::Fire => ui_images[&UiImageAsset::Dry].clone(),
            Element::Water => ui_images[&UiImageAsset::Rainy].clone(),
            Element::Earth => ui_images[&UiImageAsset::Muddy].clone(),
            Element::Neutral => ui_images.0[&UiImageAsset::Solid].clone(),
        };

        let main_frame = commands
            .spawn((
                Name::new("Battle Scene"),
                BattleScene,
                NodeBundle {
                    background_color: Color::NONE.into(),
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),

                        padding: UiRect {
                            left: Val::Px(29.),
                            right: Val::Px(20.),
                            top: Val::Px(40.),
                            bottom: Val::Px(30.),
                        },
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,

                        ..Style::DEFAULT
                    },

                    z_index: ZIndex::Global(3),
                    ..default()
                },
            ))
            .id();

        let battle_ground = commands
            .spawn((
                Name::new("Arena"),
                Arena,
                ImageBundle {
                    background_color: Color::linear_rgb(0.5, 0.5, 0.5).into(),
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(100.0),
                        height: Val::Percent(50.0),
                        border: UiRect {
                            top: Val::Px(5.0),
                            left: Val::Px(5.0),
                            right: Val::Px(5.0),
                            bottom: Val::Px(5.),
                        },
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,

                        ..Style::DEFAULT
                    },
                    image: UiImage {
                        color: Color::WHITE.into(),
                        texture: bg_image,
                        flip_x: false,
                        flip_y: false,
                    },
                    // border_color: Color::BLACK.into(),
                    z_index: ZIndex::Global(4),
                    ..default()
                },
            ))
            .id();

        let monster_row = commands
            .spawn((
                Name::new("MonsterRow"),
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        width: Val::Percent(100.),
                        height: Val::Percent(50.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::top(Val::Px(10.)),
                        margin: UiRect::horizontal(Val::Px(10.)),
                        ..default()
                    },
                    ..Default::default()
                },
            ))
            .id();

        let hero_row = commands
            .spawn((
                Name::new("HeroRow"),
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        width: Val::Percent(100.),
                        height: Val::Percent(50.),
                        padding: UiRect::bottom(Val::Px(30.)),
                        margin: UiRect::horizontal(Val::Px(10.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..Default::default()
                },
            ))
            .id();

        let hero_stand = commands
            .spawn((
                Name::new("Hero"),
                BattleHeroSpriteStand,
                NodeBundle {
                    background_color: Color::NONE.into(),
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        min_width: Val::Percent(50.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Style::DEFAULT
                    },
                    z_index: ZIndex::Global(5),
                    ..default()
                },
                LvUpQueue(0),
                LvUpTextSpawnTimer::default(),
            ))
            .id();

        let monster_stand = commands
            .spawn((
                Name::new("Monster"),
                BattleMonster {
                    monster: ev.monster,
                },
                NodeBundle {
                    background_color: Color::NONE.into(),
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        min_width: Val::Percent(50.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Style::DEFAULT
                    },
                    ..default()
                },
            ))
            .id();

        let terminal_node = commands
            .spawn((
                Name::new("Terminal Spacer"),
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(200.0),
                        border: UiRect::all(Val::Px(8.0)),
                        padding: UiRect::all(Val::Px(5.0)),
                        margin: UiRect::top(Val::Px(30.)),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    border_color: Color::BLACK.into(),
                    background_color: Color::rgb(103., 102., 81.).into(),
                    ..default()
                },
                TerminalBundle::new(22.),
            ))
            .id();

        commands
            .entity(battle_ground)
            .push_children(&[monster_row, hero_row]);
        commands
            .entity(monster_row)
            .push_children(&[monster_health_bar, monster_stand]);

        // commands.entity(monster_stand).add_child(monster);
        commands
            .entity(hero_row)
            .push_children(&[hero_stand, hero_health_bar]);

        commands
            .entity(main_frame)
            .add_child(battle_ground)
            .add_child(terminal_node);

        info!("Spawned Battle Scene");
    }
}

pub fn despawn_battle_scene(
    mut commands: Commands,
    battle_scene_query: Query<Entity, With<BattleScene>>,
) {
    for entity in battle_scene_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn spawn_lv_up_text(
    mut commands: Commands,
    mut lv_up_query: Query<(&mut LvUpQueue, &mut LvUpTextSpawnTimer, Entity), With<LvUpQueue>>,
    time: Res<Time>,
) {
    for (mut queue, mut timer, entity) in lv_up_query.iter_mut() {
        if queue.0 > 0 && timer.0.tick(time.delta()).just_finished() {
            queue.0 -= 1;
            let text = LvUpText::build(&mut commands);
            commands.entity(entity).insert_children(0, &[text]);
        }
    }
}

pub fn despawn_lv_up_text_position(
    mut commands: Commands,
    mut lv_up_query: Query<(&mut Style, &mut LvUpTextTimer, Entity, &Parent), With<LvUpText>>,
    time: Res<Time>,
) {
    for (_style, mut timer, entity, parent) in lv_up_query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            commands.entity(parent.get()).remove_children(&[entity]);
            commands.entity(entity).despawn();
        }
    }
}

pub fn spawn_drop_scene(
    mut commands: Commands,
    mut drop_scene_event: EventReader<SpawnDropSceneEvent>,
    scene_query: Query<Entity, With<BattleScene>>,
    eq_pic_assets: Res<EquipmentHandles>,
) {
    for ev in drop_scene_event.read() {
        let Ok(scene) = scene_query.get_single() else {
            continue;
        };

        let source = eq_pic_assets[&ev.eq.item_type]
            .get(ev.eq.id as usize)
            .unwrap_or(eq_pic_assets[&ItemType::Weapon].first().unwrap());
        // .0
        // .get(&ev.eq.item_type)
        // .unwrap_or(eq_pic_assets.0.get(&ItemType::Weapon).unwrap())
        // .get(&ev.eq.id.to_string())
        // .unwrap_or(eq_pic_assets[&ItemType::Weapon].first().unwrap());

        let parent = commands
            .spawn((
                Name::new("Drop"),
                NodeBundle {
                    background_color: Color::linear_rgb(0.7, 0.7, 0.7).into(),
                    z_index: ZIndex::Global(99),
                    border_color: Color::BLACK.into(),
                    style: Style {
                        border: UiRect::all(Val::Px(1.)),
                        width: Val::Px(64.),
                        flex_direction: FlexDirection::Column,
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::Center,
                        justify_self: JustifySelf::Center,
                        left: Val::Auto,
                        bottom: Val::Percent(50.),
                        ..default()
                    },
                    ..default()
                },
            ))
            .id();

        let text = commands
            .spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "GET!!".to_string(),
                        style: TextStyle {
                            font_size: 25.,
                            ..default()
                        },
                    }],
                    justify: JustifyText::Center,
                    ..Default::default()
                },
                ..default()
            })
            .id();

        let image = commands
            .spawn(ImageBundle {
                style: Style {
                    width: Val::Px(64.),
                    height: Val::Px(64.),
                    ..default()
                },
                image: UiImage::new(source.clone()),
                ..default()
            })
            .id();

        let lv_text = commands
            .spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: format!("Lv {}", ev.eq.level).to_string(),
                        style: TextStyle {
                            font_size: 15.,
                            ..default()
                        },
                    }],
                    justify: JustifyText::Center,
                    ..Default::default()
                },
                ..default()
            })
            .id();

        commands.entity(scene).add_child(parent);

        commands
            .entity(parent)
            .push_children(&[text, image, lv_text]);
    }
}
