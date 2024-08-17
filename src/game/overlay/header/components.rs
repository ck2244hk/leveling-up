use bevy::prelude::*;

#[derive(Component)]
pub struct Header {}

#[derive(Component)]
pub struct StatusBoardButton {}

#[derive(Component)]
pub struct StatusBoardButtonText {}

#[derive(Component)]
pub struct SignText {}

#[derive(Component)]
pub struct TurnsText {}

#[derive(Component)]
pub struct LevelDisplay {}

#[derive(Component)]
pub struct ExplorationLayout;

#[derive(Component)]
pub struct EncounterGaugeNode;

#[derive(Component)]
pub struct EncounterGaugeBar;

pub struct EncounterGaugeBuilder;

#[derive(Component)]
pub struct InDangerDisplay;

impl EncounterGaugeBuilder {
    pub fn build(commands: &mut Commands) -> Entity {
        let bar = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        top: Val::Px(0.),
                        ..default()
                    },
                    z_index: ZIndex::Local(1),
                    ..default()
                },
                EncounterGaugeNode,
                Name::new("Encounter Gauge"),
            ))
            .id();

        let outline_node = commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(25.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                // background_color: BackgroundColor(Color::),
                z_index: ZIndex::Local(2),
                ..default()
            })
            .id();

        let bar_background_node = commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                // background_color: BackgroundColor(Color::WHITE),
                z_index: ZIndex::Local(3),
                ..default()
            })
            .id();

        let bar_node = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(0.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgb(1.0, 0.0, 0.)),
                    z_index: ZIndex::Local(4),
                    ..default()
                },
                EncounterGaugeBar {},
            ))
            .id();

        commands.entity(outline_node).add_child(bar_background_node);
        commands.entity(bar_background_node).add_child(bar_node);
        commands.entity(bar).add_child(outline_node).id()
    }
}
