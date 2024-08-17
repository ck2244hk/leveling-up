use bevy::{asset::Asset, reflect::TypePath};

use super::{sub::Tier, EquipmentData};

#[derive(serde::Deserialize, Asset, TypePath, Debug, Clone)]
pub struct HelmetData {
    pub id: u32,
    pub name: String,
    pub tier: Tier,
    pub description: String,
}

impl EquipmentData for HelmetData {
    fn name(&self) -> String {
        self.name.to_string()
    }
    fn id(&self) -> u32 {
        self.id
    }

    fn item_type(&self) -> super::ItemType {
        super::ItemType::Helmet
    }
    fn tier(&self) -> Tier {
        self.tier.clone()
    }
    fn description(&self) -> String {
        self.description.clone()
    }
}
