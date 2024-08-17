use bevy::{asset::Asset, reflect::TypePath};

use crate::util::csv_helper::bool_from_str;

use super::{
    sub::{Element, Range, Tier, Weight},
    EquipmentData,
};

#[derive(serde::Deserialize, Asset, TypePath, Debug, Clone)]
pub struct WeaponData {
    pub id: u32,
    pub name: String,
    pub tier: Tier,
    pub range: Range,
    pub weight: Weight,
    pub slot: u32,
    #[serde(deserialize_with = "bool_from_str")]
    pub physical: bool,
    #[serde(deserialize_with = "bool_from_str")]
    pub sensory: bool,
    #[serde(deserialize_with = "bool_from_str")]
    pub fable: bool,
    #[serde(deserialize_with = "bool_from_str")]
    pub mental: bool,
    pub element: Element,
    pub description: String,
}

impl EquipmentData for WeaponData {
    fn name(&self) -> String {
        self.name.to_string()
    }
    fn id(&self) -> u32 {
        self.id
    }

    fn item_type(&self) -> super::ItemType {
        super::ItemType::Weapon
    }
    fn tier(&self) -> Tier {
        self.tier.clone()
    }
    fn description(&self) -> String {
        self.description.clone()
    }
}
