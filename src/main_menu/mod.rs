mod audio;
mod components;
mod interactions;
pub mod styles;

use crate::component::AnimationIndices;
use crate::component::AnimationTimer;
use crate::controller::Screen;
use crate::graphic::HeroActions;
use crate::graphic::UiImageAsset;
use crate::ButtonClickEffect;
use crate::FirstTime;
use crate::OverlayShopState;
use crate::SimulationState;

use bevy::prelude::*;

use self::audio::*;
use self::components::*;
use self::interactions::*;
use self::styles::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(Screen::Title),
            (
                spawn_main_menu,
                // spawn_background_music
            ),
        )
        .add_systems(OnExit(Screen::Title), despawn_main_menu)
        .add_systems(
            Update,
            (
                interact_with_play_button,
                interact_with_quit_button,
                interact_with_shop_button,
            )
                .run_if(in_state(Screen::Title))
                .run_if(in_state(OverlayShopState::Closed)),
        )
        .add_systems(OnEnter(SimulationState::Running), despawn_background_music);
    }
}

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    image_set: Res<UiImageAsset>,
    first_time_query: Query<&FirstTime>,
    sprite_set: Res<HeroActions>,
) {
    let start_button_text = if first_time_query.is_empty() {
        "Continue"
    } else {
        "Start New Game"
    };

    let background = commands
        .spawn((
            Name::new("Menu"),
            ImageBundle {
                background_color: Color::linear_rgb(0.5, 0.5, 0.5).into(),
                style: get_main_menu_style(),
                image: UiImage {
                    texture: image_set.0.get("cover").expect("No Cover Pic").clone(),
                    ..default()
                },
                ..default()
            },
            MainMenu {},
        ))
        .id();

    let title = commands
        .spawn(NodeBundle {
            style: get_title_style(),
            ..default()
        })
        .with_children(|parent| {
            // Text
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Progression Overpower".to_string(),
                        TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 50.0,
                            color: Color::WHITE,
                        },
                    )],
                    justify: JustifyText::Center,
                    ..default()
                },
                ..default()
            });
        })
        .id();

    let row = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            ..default()
        })
        .id();

    let container = commands
        .spawn(
            (NodeBundle {
                style: Style {
                    width: Val::Px(250.),
                    max_width: Val::Percent(40.),
                    ..default()
                },
                ..default()
            }),
        )
        .id();

    let sprite = commands
        .spawn((
            AnimationIndices::new(3, true),
            AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
            AtlasImageBundle {
                texture_atlas: TextureAtlas {
                    layout: sprite_set
                        .0
                        .get("MenuGreeting")
                        .expect("No Play Pic")
                        .0
                        .clone(),
                    index: 0,
                },

                image: UiImage {
                    texture: sprite_set
                        .0
                        .get("MenuGreeting")
                        .expect("No Play Pic")
                        .1
                        .clone(),
                    ..default()
                },
                style: Style {
                    width: Val::Px(250.),
                    position_type: PositionType::Absolute,
                    left: Val::Px(-50.),
                    bottom: Val::Px(25.),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let column = commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(50.),
                ..default()
            },
            ..default()
        })
        .id();

    let play_button = commands
        .spawn((
            ButtonBundle {
                background_color: NORMAL_BUTTON_COLOR.into(),
                style: get_normal_button_style(),
                ..default()
            },
            PlayButton {},
            ButtonClickEffect,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: get_menu_button_text(start_button_text, &asset_server),
                ..default()
            });
        })
        .id();

    let shop_button = commands
        .spawn((
            ButtonBundle {
                background_color: NORMAL_BUTTON_COLOR.into(),
                style: get_normal_button_style(),
                ..default()
            },
            ShopButton {},
            ButtonClickEffect,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: get_menu_button_text("Shop", &asset_server),
                ..default()
            });
        })
        .id();

    let quit_button = commands
        .spawn((
            ButtonBundle {
                background_color: NORMAL_BUTTON_COLOR.into(),
                style: get_normal_button_style(),
                ..default()
            },
            QuitButton {},
            ButtonClickEffect,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: get_menu_button_text("Quit", &asset_server),
                ..default()
            });
        })
        .id();

    commands.entity(row).push_children(&[container, column]);

    commands.entity(container).add_child(sprite);

    commands
        .entity(column)
        .push_children(&[play_button, shop_button, quit_button]);

    commands.entity(background).push_children(&[title, row]);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}
