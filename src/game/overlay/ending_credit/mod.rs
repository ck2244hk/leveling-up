use bevy::prelude::*;

mod update;

use crate::{helper::ValExtension, state::OverlayEndingCreditState};
use update::*;

pub struct EndingCreditPlugin;
impl Plugin for EndingCreditPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(OverlayEndingCreditState::Opened),
            spawn_credit_scene,
        )
        .add_systems(
            Update,
            update_credit_position.run_if(in_state(OverlayEndingCreditState::Opened)),
        )
        .add_systems(
            OnExit(OverlayEndingCreditState::Opened),
            despawn_ending_credit,
        );
    }
}

#[derive(Component)]
pub struct EndingCreditBackground;

#[derive(Component)]
pub struct EndingCreditScene;

#[derive(Component)]
struct CreditBlock;

#[derive(Component)]
struct CreditorsBlock;

struct CreditBlockBundle {
    title: String,
    creditors: Vec<String>,
}

impl CreditBlockBundle {
    pub fn new(title: String, creditors: Vec<String>) -> Self {
        CreditBlockBundle { title, creditors }
    }

    fn build(&self, commands: &mut Commands) -> Entity {
        let block = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_self: JustifySelf::Center,
                        align_self: AlignSelf::Center,
                        justify_content: JustifyContent::Center,
                        align_content: AlignContent::Center,
                        justify_items: JustifyItems::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(30.)),
                        ..default()
                    },

                    ..default()
                },
                CreditBlock,
            ))
            .id();

        let title = commands
            .spawn(get_title_template(self.title.to_string()))
            .id();

        let creditors_block = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        flex_wrap: FlexWrap::Wrap,
                        justify_self: JustifySelf::Center,
                        align_self: AlignSelf::Center,
                        justify_content: JustifyContent::Center,
                        align_content: AlignContent::Center,
                        justify_items: JustifyItems::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                },
                CreditorsBlock,
            ))
            .id();

        for creditor in &self.creditors {
            let child = commands
                .spawn(get_creditor_template(creditor.to_string()))
                .id();
            commands.entity(creditors_block).add_child(child);
        }

        commands
            .entity(block)
            .push_children(&[title, creditors_block]);

        block
    }
}

fn spawn_credit_scene(mut commands: Commands) {
    let background = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.6).into(),
                z_index: ZIndex::Global(99),
                ..default()
            },
            EndingCreditBackground,
        ))
        .id();

    let scene = commands
        .spawn((
            NodeBundle {
                style: Style {
                    bottom: Val::Vh(-100.),
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
            EndingCreditScene,
        ))
        .id();

    let boss = CreditBlockBundle::new("Sponsor".to_string(), vec!["Tornado".to_string()])
        .build(&mut commands);
    let design = CreditBlockBundle::new(
        "Game Design".to_string(),
        vec![
            "Tornado".to_string(),
            "ItalianDay".to_string(),
            "Fumhy".to_string(),
        ],
    )
    .build(&mut commands);
    let graphic = CreditBlockBundle::new(
        "Graphic".to_string(),
        vec!["ItalianDay".to_string(), "Fumhy".to_string()],
    )
    .build(&mut commands);
    let programmer =
        CreditBlockBundle::new("Programmer".to_string(), vec!["ItalianDay".to_string()])
            .build(&mut commands);
    let audio = CreditBlockBundle::new("Audio".to_string(), vec!["ItalianDay".to_string()])
        .build(&mut commands);

    commands.entity(background).add_child(scene);
    commands
        .entity(scene)
        .push_children(&[boss, design, graphic, programmer, audio]);
}

fn get_title_template(text: String) -> TextBundle {
    TextBundle {
        style: Style {
            align_self: AlignSelf::Center,

            ..default()
        },
        text: Text {
            sections: vec![TextSection {
                value: text,
                style: TextStyle {
                    font_size: 30.,
                    ..default()
                },
            }],
            justify: JustifyText::Center,
            linebreak_behavior: bevy::text::BreakLineOn::WordBoundary,
        },
        ..default()
    }
}

fn get_creditor_template(text: String) -> TextBundle {
    TextBundle {
        style: Style {
            margin: UiRect::horizontal(Val::Px(30.)),
            ..default()
        },
        text: Text {
            sections: vec![TextSection {
                value: text,
                style: TextStyle {
                    font_size: 30.,
                    ..default()
                },
            }],
            justify: JustifyText::Center,
            linebreak_behavior: bevy::text::BreakLineOn::WordBoundary,
        },
        ..default()
    }
}

pub fn despawn_ending_credit(
    mut commands: Commands,
    scene_query: Query<Entity, With<EndingCreditBackground>>,
) {
    if let Ok(entity) = scene_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
