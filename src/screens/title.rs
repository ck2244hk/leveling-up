//! The title screen that appears when the game starts.

use bevy::prelude::*;

use crate::{
    preload::{
        images::{UiImageAsset, UiImageHandles},
        sprites::{HeroActionHandles, HeroActionTextureAtLasHandles},
    },
    state::{FirstTime, OverlayShopState, Screen, SimulationState},
    theme::prelude::*,
};

#[derive(Component)]
pub struct PlayButton {}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), show_title_screen);
}

fn show_title_screen(
    mut commands: Commands,
    image_set: Res<UiImageHandles>,
    // sprite_set: Res<HeroActionHandles>,
    // texture_atlas_set: Res<HeroActionTextureAtLasHandles>,
) {
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
    // let column = commands
    //     .spawn(NodeBundle {
    //         style: Style {
    //             flex_direction: FlexDirection::Column,
    //             width: Val::Percent(50.),
    //             ..default()
    //         },
    //         ..default()
    //     })
    //     .id();
    // let container = commands
    //     .spawn(
    //         (NodeBundle {
    //             style: Style {
    //                 width: Val::Px(250.),
    //                 max_width: Val::Percent(40.),
    //                 ..default()
    //             },
    //             ..default()
    //         }),
    //     )
    //     .id();

    // let sprite = commands
    //     .spawn((
    //         AnimationIndices::new(3, true),
    //         AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
    //         ImageBundle {
    //             image: UiImage {
    //                 texture: sprite_set
    //                     .0
    //                     .get(&HeroAction::MenuGreeting)
    //                     .expect("No Play Pic")
    //                     .clone(),
    //                 ..default()
    //             },
    //             style: Style {
    //                 width: Val::Px(250.),
    //                 position_type: PositionType::Absolute,
    //                 left: Val::Px(-50.),
    //                 bottom: Val::Px(25.),
    //                 ..default()
    //             },
    //             ..default()
    //         },
    //         TextureAtlas {
    //             layout: texture_atlas_set
    //                 .0
    //                 .get(&HeroAction::MenuGreeting)
    //                 .expect("No Play Texture atlas")
    //                 .clone(),
    //             index: 0,
    //         },
    //     ))
    //     .id();

    // commands.entity(container).add_child(sprite);

    // let content = commands
    //     .entity(row)
    //     .push_children(&[container, column])
    //     .id();

    // commands.entity(column).with_children(|children| {
    //     children.button("Play").observe(enter_playing);
    //     children.button("Credits").observe(enter_credits);
    //     #[cfg(not(target_family = "wasm"))]
    //     children.button("Exit").observe(exit_app);
    // });

    commands
        .ui_root_w_bg_image(image_set.0.get(&UiImageAsset::Cover).expect("No Cover Pic"))
        .insert(StateScoped(Screen::Title))
        .with_children(|children| {
            children.title("Leveling Up");
            children.button("Play").observe(enter_playing);
            children.button("Shop").observe(enter_shop);
            children.button("Credits").observe(enter_credits);
            #[cfg(not(target_family = "wasm"))]
            children.button("Exit").observe(exit_app);
        });
    // .add_child(content);
}

fn enter_shop(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<OverlayShopState>>) {
    next_screen.set(OverlayShopState::Opened);
}

fn enter_playing(
    _trigger: Trigger<OnPress>,
    mut commands: Commands,
    mut next_screen: ResMut<NextState<Screen>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
    first_time_query: Query<Entity, (With<FirstTime>, Without<PlayButton>)>,
) {
    next_simulation_state.set(SimulationState::Pause);
    next_screen.set(Screen::Playing);
    if let Ok(entity) = first_time_query.get_single() {
        commands.entity(entity).despawn();
    }
}

fn enter_credits(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_trigger: Trigger<OnPress>, mut app_exit: EventWriter<AppExit>) {
    app_exit.send(AppExit::Success);
}
