use bevy::prelude::*;

use crate::game::{character::component::HeroClass, overlay::header::styles::NORMAL_BUTTON_COLOR};

#[derive(Component)]
pub struct HeroButton(HeroClass);

pub struct HeroPickingUIPlugin;

impl Plugin for HeroPickingUIPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(
        //     OnEnter(SimulationState::HeroPicking),
        //     spawn_hero_picking_scene,
        // )
        // .add_systems(OnExit(SimulationState::HeroPicking), despawn_hero_grid)
        // .add_systems(
        //     Update,
        //     interaction_with_hero_class_button.run_if(in_state(SimulationState::HeroPicking)),
        // );
    }
}

#[derive(Component)]
pub struct HeroPickingScene;

pub fn spawn_hero_picking_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let buttons = commands
        .spawn((
            HeroPickingScene,
            NodeBundle {
                style: Style {
                    // Make the height of the node fill its parent
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.),
                    // Make the grid have a 1:1 aspect ratio meaning it will scale as an exact square
                    // As the height is set explicitly, this means the width will adjust to match the height
                    aspect_ratio: Some(1.0),
                    // Use grid layout for this node
                    display: Display::Grid,
                    // Add 24px of padding around the grid
                    padding: UiRect::all(Val::Px(24.0)),
                    // Set the grid to have 4 columns all with sizes minmax(0, 1fr)
                    // This creates 4 exactly evenly sized columns
                    grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
                    // Set the grid to have 4 rows all with sizes minmax(0, 1fr)
                    // This creates 4 exactly evenly sized rows
                    grid_template_rows: RepeatedGridTrack::flex(1, 1.0),
                    // Set a 12px gap/gutter between rows and columns
                    row_gap: Val::Px(12.0),
                    column_gap: Val::Px(12.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::linear_rgb(0.7, 0.7, 0.7)),
                ..default()
            },
        ))
        .id();

    for hero_class in HeroClass::iterator() {
        println!("Spawned Field Button {}", hero_class.to_string());
        let button_id = commands
            .spawn((NodeBundle {
                style: Style {
                    display: Display::Grid,
                    padding: UiRect::all(Val::Px(3.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            },))
            .with_children(|builder| {
                builder
                    .spawn((
                        ButtonBundle {
                            background_color: NORMAL_BUTTON_COLOR.into(),
                            style: Style {
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..Style::DEFAULT
                            },
                            ..default()
                        },
                        HeroButton(*hero_class),
                    ))
                    .with_children(|button| {
                        button.spawn(TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    hero_class.to_string(),
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                )],
                                justify: JustifyText::Center,

                                ..default()
                            },
                            ..default()
                        });
                    });
            })
            .id();

        commands.entity(buttons).add_child(button_id);
    }

    println!("Spawned all Field Buttons");
}

pub fn despawn_hero_grid(
    mut commands: Commands,
    grid_query: Query<Entity, With<HeroPickingScene>>,
) {
    for entity in grid_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// pub fn interaction_with_hero_class_button(
//     mut button_query: Query<
//         (&Interaction, &mut BackgroundColor, &HeroButton),
//         (Changed<Interaction>, With<HeroButton>),
//     >,
//     mut spawn_hero_event_writer: EventWriter<SpawnHeroEvent>,
//     mut next_simulation_state: ResMut<NextState<SimulationState>>,
//     mut spawn_map_event_writer: EventWriter<SpawnMapEvent>,
//     mut spawn_header_event_writer: EventWriter<SpawnHeaderEvent>,
// ) {
//     for (interaction, mut background_color, hero_button) in button_query.iter_mut() {
//         match *interaction {
//             Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
//             Interaction::Pressed => {
//                 spawn_hero_event_writer.send(SpawnHeroEvent(hero_button.0));
//                 next_simulation_state.set(SimulationState::Running);
//                 spawn_map_event_writer.send(SpawnMapEvent {});
//                 spawn_header_event_writer.send(SpawnHeaderEvent {});
//             }

//             Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
//         }
//     }
// }
