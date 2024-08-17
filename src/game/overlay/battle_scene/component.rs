use bevy::prelude::*;
use rand::random;

// Question for future
// Monster Health bar display follow inflation RPG or using one health bar percentage

pub struct HealthBarUI;

impl HealthBarUI {
    pub fn build_monster(commands: &mut Commands, health: f32, name: &Name) -> Entity {
        let name = commands
            .spawn(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(15.),
                    ..default()
                },
                text: Text {
                    sections: vec![TextSection {
                        value: name.as_str().to_string(),
                        style: TextStyle {
                            color: Color::WHITE,
                            font_size: 15.,
                            ..default()
                        },
                        ..default()
                    }],
                    ..default()
                },
                ..default()
            })
            .id();

        let frame = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(100.),
                        ..default()
                    },
                    ..default()
                },
                Name::new("Monster Health Bar"),
            ))
            .id();

        let health_bar = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        top: Val::Px(0.),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    ..default()
                },
                MonsterHealthNode {},
                Name::new("Monster Health Bar"),
            ))
            .id();

        let health_outline_node = commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(10.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            })
            .id();

        let health_bar_background_node = commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::WHITE),
                ..default()
            })
            .id();

        let health_bar_node = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::linear_rgb(0., 1., 0.)),
                    ..default()
                },
                MonsterHealthBar {},
                HealthBar {
                    max: health,
                    remain: health,
                },
            ))
            .id();

        commands
            .entity(health_outline_node)
            .add_child(health_bar_background_node)
            .add_child(name);
        commands
            .entity(health_bar_background_node)
            .add_child(health_bar_node);
        commands.entity(health_bar).add_child(health_outline_node);

        commands.entity(frame).push_children(&[health_bar]);
        frame
    }

    pub fn build_hero(commands: &mut Commands, health: f32) -> Entity {
        let name = commands
            .spawn(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(15.),
                    ..default()
                },
                text: Text {
                    sections: vec![TextSection {
                        value: "Casca".to_string(),
                        style: TextStyle {
                            color: Color::WHITE,
                            font_size: 15.,
                            ..default()
                        },
                        ..default()
                    }],
                    ..default()
                },
                ..default()
            })
            .id();

        let frame = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(100.),
                        ..default()
                    },
                    ..default()
                },
                Name::new("Monster Health Bar"),
            ))
            .id();

        let health_bar = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        position_type: PositionType::Absolute,
                        top: Val::Px(25.),
                        ..default()
                    },
                    ..default()
                },
                HeroHealthNode {},
                Name::new("Hero Health Bar"),
            ))
            .id();

        let health_outline_node = commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Px(100.0),
                    height: Val::Px(10.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            })
            .id();

        let health_bar_background_node = commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::WHITE),
                ..default()
            })
            .id();

        let health_bar_node = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::linear_rgb(0., 1., 0.)),
                    ..default()
                },
                HeroHealthBar {},
                HealthBar {
                    max: health,
                    remain: health,
                },
            ))
            .id();

        commands
            .entity(health_outline_node)
            .add_child(health_bar_background_node)
            .add_child(name);
        commands
            .entity(health_bar_background_node)
            .add_child(health_bar_node);
        commands.entity(health_bar).add_child(health_outline_node);

        commands.entity(frame).add_child(health_bar).id()
    }
}

#[derive(Component, Reflect)]
pub struct HealthBar {
    max: f32,
    pub remain: f32,
}

impl HealthBar {
    pub fn get_percentage(&self) -> f32 {
        0_f32.max(self.remain / self.max * 100.)
    }
}

#[derive(Component)]
pub struct MonsterHealthBar;

#[derive(Component)]

pub struct MonsterHealthNode;

#[derive(Component)]
pub struct HeroHealthBar;

#[derive(Component)]

pub struct HeroHealthNode;

#[derive(Component, Default)]
pub struct LvUpQueue(pub u32);

#[derive(Component)]
pub struct LvUpTextSpawnTimer(pub Timer);

impl Default for LvUpTextSpawnTimer {
    fn default() -> Self {
        LvUpTextSpawnTimer(Timer::from_seconds(0.7, TimerMode::Repeating))
    }
}

#[derive(Component)]
pub struct LvUpTextTimer(pub Timer);

impl Default for LvUpTextTimer {
    fn default() -> Self {
        LvUpTextTimer(Timer::from_seconds(10., TimerMode::Once))
    }
}

#[derive(Component)]
pub struct LvUpText;

impl LvUpText {
    pub fn build(commands: &mut Commands) -> Entity {
        commands
            .spawn((
                TextBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Val::Px(30. + random::<f32>() * 25.),
                        top: Val::Px(30.),
                        ..default()
                    },
                    text: Text {
                        sections: vec![TextSection {
                            value: "Level UP!".to_string(),
                            style: TextStyle {
                                font_size: 10.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        }],
                        ..default()
                    },
                    z_index: ZIndex::Local(99),
                    ..default()
                },
                LvUpTextTimer::default(),
                LvUpText,
            ))
            .id()
    }
}

// #[derive(Component)]
// pub struct DropScene;
