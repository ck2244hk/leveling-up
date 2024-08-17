use bevy::{ecs::component::Component, reflect::Reflect, time::Timer};

use crate::model::{sub::DropEquipment, EquipmentData};

#[derive(Component)]
pub struct ShopImage;

#[derive(Component, Reflect, Default, PartialEq, Eq, Clone, Debug)]
pub enum ShopTab {
    #[default]
    Weapon,
    Armor,
    Helmet,
    Shoes,
}

#[derive(Component)]
pub struct UnequipButton(pub DropEquipment);

#[derive(Component)]
pub struct UpgradeButton(pub DropEquipment);

#[derive(Component)]
pub struct SelectedEqPic;

#[derive(Component)]
pub struct SelectedEqAttr;

#[derive(Component)]
pub struct SelectedEqDes;

#[derive(Component)]
pub struct CurrentEqPic;

#[derive(Component)]
pub struct CurrentEqAttr;

#[derive(Component)]
pub struct CurrentEqDes;

#[derive(Component)]
pub struct BackButton;

#[derive(Component)]
pub struct TotalMoney;

#[derive(Component, Default)]
pub struct ScrollingList {
    pub position: f32,
}

#[derive(Component)]
pub struct ShopLayout;

#[derive(Component)]
pub struct ScrollingListTitleRow;

#[derive(Component)]
pub struct ScrollingListContainer;

#[derive(Component)]
pub struct EqipmentRow(pub DropEquipment);

#[derive(Component)]
pub struct BuyButton(pub DropEquipment);

#[derive(Component, Default, PartialEq, Eq)]
pub struct SelectedEquipment(pub Option<DropEquipment>);

#[derive(Component)]
pub struct Shop;

#[derive(Component, Default, Reflect, PartialEq, Eq)]
pub struct ActiveShopTab(pub ShopTab);

#[derive(Component)]
pub struct EquireButton(pub DropEquipment);

#[derive(Component)]
pub struct ConfirmPopup;

#[derive(Component)]
pub struct CancelButton;

#[derive(Component)]
pub struct ConfirmButton(pub u32);

#[derive(Component)]
pub struct WarningTextLayout(pub Timer);

#[derive(Component)]
pub struct WarningText;
