use bevy::prelude::*;

use crate::{
    audio_effects::ButtonClickEffect,
    game::{
        board::SelectGridEvent, character::component::*, overlay::board::components::*,
        terminal::component::AutoTerminalText, Money, Player,
    },
    model::{sub::DropEquipment, ItemType},
    preload::images::EquipmentHandles,
};

pub fn update_str_text(
    mut str_text_query: Query<&mut Text, With<StrText>>,
    player_query: Query<&Strength, (Changed<Strength>, With<Hero>)>,
) {
    if let Ok(mut str_text) = str_text_query.get_single_mut() {
        if let Ok(stat) = player_query.get_single() {
            str_text.sections[0].value = format!("Str: {0}", stat.get());
        }
    }
}

pub fn update_str_buffer_text(
    mut str_text_query: Query<&mut Text, With<StrText>>,
    mut def_text_query: Query<&mut Text, (With<DefText>, Without<StrText>)>,
    mut agi_text_query: Query<&mut Text, (With<AgiText>, Without<DefText>, Without<StrText>)>,
    buffer_query: Query<&UpgradeBuffer, Changed<UpgradeBuffer>>,
) {
    if let (Ok(mut str_text), Ok(mut def_text), Ok(mut agi_text)) = (
        str_text_query.get_single_mut(),
        def_text_query.get_single_mut(),
        agi_text_query.get_single_mut(),
    ) {
        if let Ok(buffer) = buffer_query.get_single() {
            if buffer.get_total_point() > 0 {
                str_text.sections[1].value = format!("+ ({})", buffer.strength);
                def_text.sections[1].value = format!("+ ({})", buffer.defense);
                agi_text.sections[1].value = format!("+ ({})", buffer.agi);
            } else {
                str_text.sections[1].value = format!("");
                def_text.sections[1].value = format!("");
                agi_text.sections[1].value = format!("");
            }
        }
    }
}

pub fn update_def_text(
    mut text_query: Query<&mut Text, With<DefText>>,
    player_query: Query<&Defense, (Changed<Defense>, With<Hero>)>,
) {
    if let Ok(mut text) = text_query.get_single_mut() {
        if let Ok(stat) = player_query.get_single() {
            text.sections[0].value = format!("Def: {0}", stat.get());
        }
    }
}

pub fn update_agi_text(
    mut text_query: Query<&mut Text, With<AgiText>>,
    player_query: Query<&Critical, (Changed<Critical>, With<Hero>)>,
) {
    if let Ok(mut text) = text_query.get_single_mut() {
        if let Ok(stat) = player_query.get_single() {
            text.sections[0].value = format!("Agi: {0}", stat.get());
        }
    }
}

pub fn update_state_point_text(
    mut text_query: Query<&mut Text, With<AvailablePointText>>,
    player_query: Query<&StatePoint, (Changed<StatePoint>, With<Hero>)>,
) {
    if let Ok(mut text) = text_query.get_single_mut() {
        if let Ok(stat) = player_query.get_single() {
            text.sections[0].value = format!("Point: {}", stat.get());
        }
    }
}

pub fn update_money_text(
    mut text_query: Query<&mut Text, With<MoneyText>>,
    player_query: Query<&Money, (Changed<Money>, With<Player>)>,
) {
    if let Ok(mut text) = text_query.get_single_mut() {
        if let Ok(stat) = player_query.get_single() {
            text.sections[0].value = format!("${}", stat.get());
        }
    }
}

pub fn return_state_points(
    mut buffer_query: Query<&mut UpgradeBuffer, Changed<UpgradeBuffer>>,
    mut player_query: Query<&mut StatePoint, With<Hero>>,
) {
    if let (Ok(mut buffer), Ok(mut state_point)) =
        (buffer_query.get_single_mut(), player_query.get_single_mut())
    {
        state_point.plus(buffer.get_total_point());
        buffer.reset();
    }
}

pub fn update_experience_gauge_ui(
    mut commands: Commands,
    ui_query: Query<Entity, Added<ExperienceUI>>,
    hero_query: Query<&BaseStates, With<Hero>>,
) {
    if let Ok(ui) = ui_query.get_single() {
        let hero = hero_query.get_single().expect("No Hero Spawned");

        info!(
            "exp: {}, exp for next: {}, percentage: {}",
            hero.exp_after_lv_up(),
            hero.exp_req_for_next_lv(),
            (hero.exp_after_lv_up() - 100.) / (hero.exp_req_for_next_lv() - 100.)
        );
        let child = ExperienceUI::build(
            &mut commands,
            hero.exp_after_lv_up(),
            hero.exp_req_for_next_lv(),
        );

        commands.entity(ui).add_child(child);
    }
}

pub fn update_bag_grid(
    mut commands: Commands,
    ui_query: Query<Entity, Added<BagGrid>>,
    hero_query: Query<&Bag, With<Hero>>,
    eq_pic_assets: Res<EquipmentHandles>,
) {
    if let Ok(ui) = ui_query.get_single() {
        let bag = hero_query.get_single().expect("No Hero Spawned");
        for equipment in bag.0.iter() {
            let parent = commands
                .spawn((NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        width: Val::Px(64.),
                        height: Val::Px(64.),
                        padding: UiRect::all(Val::Px(3.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::BLACK),
                    ..default()
                },))
                .id();
            let source = eq_pic_assets
                .0
                .get(&ItemType::Weapon)
                .expect("Sword 01 png failed to load")
                .get(equipment.id as usize)
                .unwrap_or(
                    eq_pic_assets
                        .0
                        .get(&ItemType::Weapon)
                        .unwrap()
                        .get(1)
                        .unwrap(),
                );

            let image = commands
                .spawn((
                    ButtonClickEffect,
                    StatusBoardZone {},
                    ImageBundle {
                        style: Style {
                            border: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        image: UiImage::new(source.clone()),
                        ..default()
                    },
                    // BorderColor(Color::WHITE),
                    Interaction::None,
                    equipment.clone(),
                    SingleDropEqGrid,
                ))
                .id();

            commands.entity(ui).add_child(parent);
            commands.entity(parent).add_child(image);
        }
    }
}

pub fn handle_selecting_grid(
    mut commands: Commands,
    mut grid_query: Query<
        (Entity, Option<&SelectedGrid>, &DropEquipment, &Parent),
        With<SingleDropEqGrid>,
    >,
    mut terminal_query: Query<&mut AutoTerminalText>,
    mut event: EventReader<SelectGridEvent>,
    mut background_query: Query<&mut BackgroundColor, Without<SingleDropEqGrid>>,
) {
    for ev in event.read() {
        info!("Select Event: {:?}", ev.0);
        let Ok(mut auto_text) = terminal_query.get_single_mut() else {
            warn!("No AutoText in StatusBoard");
            continue;
        };

        // 1. unselected all
        let mut any_selected = false;
        for (cell_entity, selected, eq, parent) in grid_query.iter_mut() {
            let Ok(mut bg_color) = background_query.get_mut(parent.get()) else {
                warn!("No BackgroundColor in Grid {:?} ", cell_entity);
                continue;
            };

            if selected.is_some() {
                info!("Cleaning : {:?}", cell_entity);
                commands.entity(cell_entity).remove::<SelectedGrid>();
                *bg_color = Color::BLACK.into();
                // border_color.0 = Color::WHITE.into();
            } else if ev.0.is_some_and(|a| a == cell_entity) {
                any_selected = true;
                info!("Selecting : {:?}", cell_entity);
                commands.entity(cell_entity).insert(SelectedGrid {});
                *bg_color = Color::srgb(0.4, 0.4, 0.2).into();

                auto_text.text = vec![
                    eq.description.clone(),
                    format!("A {} with Level {} not too bad", eq.name, eq.level),
                ];

                // border_color.0 = Color::srgb(0.4, 0.4, 0.2).into();
            }
        }

        if !any_selected {
            auto_text.reset();
        }
    }
}

// pub fn clear_auto_text(
//     mut grid_query: Query<(Entity, Option<&SelectedGrid>, &DropEquipment), Changed<SelectedGrid>>,
//     mut terminal_query: Query<&mut AutoTerminalText>,
// ) {
//     if !grid_query.iter().any(|(_, selected, _)| selected.is_some()) && !grid_query.is_empty() {
//         let Ok(mut auto_text) = terminal_query.get_single_mut() else {
//             warn!("No AutoText in StatusBoard");
//             return;
//         };
//         auto_text.reset();
//     }
// }

// pub fn update_selected_grid(
//     mut grid_query: Query<(&Style, &GlobalTransform), With<SelectedGrid>>,
//     mut gizmos: Gizmos,
// ) {
//     for (style, g_transform) in grid_query.iter() {
//         gizmos.rect_2d(
//             g_transform.compute_transform().translation.truncate(),
//             0.,
//             Vec2 {
//                 x: style.width.get(),
//                 y: style.height.get(),
//             },
//             Color::srgb(0.4, 0.4, 0.2),
//         );
//     }
// }
