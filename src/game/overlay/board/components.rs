use bevy::prelude::*;

use crate::model::sub::DropEquipment;

#[derive(Component)]
pub struct StatusBoard {}

#[derive(Component)]
pub struct StatusBoardRow;

#[derive(Component)]
pub struct AvailablePointText;
#[derive(Component)]
pub struct HeroPicture;

#[derive(Component)]
pub struct StrText;

#[derive(Component)]
pub struct StrButton;

#[derive(Component)]
pub struct DefText;

#[derive(Component)]
pub struct DefButton;

#[derive(Component)]
pub struct AgiText;

#[derive(Component)]
pub struct AgiButton;

#[derive(Component)]
pub struct MoneyText(pub f32);

#[derive(Component)]
pub struct ExperienceText(pub f32);

#[derive(Component)]
pub struct ScoreBoard {}

#[derive(Component)]
pub struct LevelText(pub f32);

#[derive(Component)]
pub struct MenuButton {}

#[derive(Component)]
pub struct ShopButton {}

#[derive(Component)]
pub struct ShopButtonText {}

#[derive(Component)]
pub struct MenuButtonText {}

#[derive(Component)]
pub struct StatusBoardZone {}

#[derive(Component)]
pub struct StatusBoardFrame;

#[derive(Component)]
pub struct ConfirmButton;

#[derive(Component)]
pub struct AllStrButton;

#[derive(Component)]
pub struct AllDefButton;

#[derive(Component)]
pub struct AllAgiButton;

#[derive(Component)]
pub struct ResetButton;

#[derive(Component)]
pub struct RestartButton;

#[derive(Component)]
pub struct BagGrid;

#[derive(Component)]
pub struct SlotContainers(pub Vec<InteractiveSlot>);
impl SlotContainers {
    pub fn drop_list(&self) -> Vec<DropEquipment> {
        self.0.iter().map(|x| x.get_value()).collect()
    }
}

#[derive(Clone, Debug)]
pub enum InteractiveSlot {
    Selected(DropEquipment),
    Confirmed(DropEquipment),
}

impl InteractiveSlot {
    pub fn to_confirmed(&mut self) -> Self {
        Self::Confirmed(self.get_value())
    }

    pub fn check_if_select(&self) -> bool {
        match self {
            InteractiveSlot::Selected(_) => true,
            InteractiveSlot::Confirmed(_) => false,
        }
    }

    pub fn get_value(&self) -> DropEquipment {
        match self {
            InteractiveSlot::Selected(item) => item.clone(),
            InteractiveSlot::Confirmed(item) => item.clone(),
        }
    }
}

#[derive(Component)]
pub struct SelectedSlotID(pub usize);

#[derive(Component)]
pub struct DropNextButtonHolder;

#[derive(Component)]
pub struct DropBackButtonHolder;

#[derive(Component)]
pub struct SelectedGrid;

#[derive(Component)]
pub struct SingleDropEqGrid;

#[derive(Component)]
pub struct DropNextButton;

#[derive(Component)]
pub struct DropFinishButton;

#[derive(Component)]
pub struct DropBackButton;

#[derive(Component, Default)]
pub struct PickingBag();

#[derive(Component)]
pub struct NotSelectedEq {
    pub eq: DropEquipment,
    pub image: Handle<Image>,
    pub empty: bool,
}

#[derive(Component, Default)]
pub struct UpgradeBuffer {
    pub strength: u32,
    pub defense: u32,
    pub agi: u32,
}

impl UpgradeBuffer {
    pub fn get_total_point(&self) -> u32 {
        self.strength + self.agi + self.defense
    }
    pub fn reset(&mut self) {
        self.strength = 0;
        self.agi = 0;
        self.defense = 0;
    }
}

#[derive(Component)]
pub struct ExperienceUI;

impl ExperienceUI {
    pub fn build(commands: &mut Commands, exp: f32, require_exp: f32) -> Entity {
        let exp_bar = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        top: Val::Px(0.),
                        ..default()
                    },
                    ..default()
                },
                Name::new("Experience Bar"),
            ))
            .id();

        let exp_outline_node = commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(5.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::horizontal(Val::Px(3.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::WHITE),
                ..default()
            })
            .id();

        let exp_bar_background_node = commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.7, 0.7, 0.2)),
                ..default()
            })
            .id();

        let exp_bar_node = commands
            .spawn((NodeBundle {
                style: Style {
                    width: Val::Percent(exp / require_exp * 100.),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.4, 0.4, 0.2)),
                ..default()
            },))
            .id();

        commands
            .entity(exp_outline_node)
            .add_child(exp_bar_background_node);
        commands
            .entity(exp_bar_background_node)
            .add_child(exp_bar_node);
        commands.entity(exp_bar).add_child(exp_outline_node).id()
    }
}
