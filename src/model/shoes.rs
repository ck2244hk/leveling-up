use bevy::{asset::Asset, reflect::TypePath};

use super::{sub::Tier, EquipmentData};

#[derive(serde::Deserialize, Asset, TypePath, Debug, Clone)]
pub struct ShoesData {
    pub id: u32,
    pub name: String,
    pub tier: Tier,
    pub description: String,
}

impl EquipmentData for ShoesData {
    fn name(&self) -> String {
        self.name.to_string()
    }
    fn id(&self) -> u32 {
        self.id
    }

    fn item_type(&self) -> super::ItemType {
        super::ItemType::Shoes
    }
    fn tier(&self) -> Tier {
        self.tier.clone()
    }
    fn description(&self) -> String {
        self.description.clone()
    }
}
