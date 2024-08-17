use bevy::{asset::Asset, reflect::TypePath};

use super::{
    sub::{Element, Tier, Weight},
    EquipmentData,
};

#[derive(serde::Deserialize, Asset, TypePath, Debug, Clone)]
pub struct ArmorData {
    pub id: u32,
    pub name: String,
    pub tier: Tier,
    pub weight: Weight,
    pub element: Element,
    pub description: String,
}

impl EquipmentData for ArmorData {
    fn name(&self) -> String {
        self.name.to_string()
    }
    fn id(&self) -> u32 {
        self.id
    }

    fn item_type(&self) -> super::ItemType {
        super::ItemType::Armor
    }
    fn tier(&self) -> Tier {
        self.tier.clone()
    }
    fn description(&self) -> String {
        self.description.clone()
    }
}
