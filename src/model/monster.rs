use crate::util::csv_helper::bool_from_str;
use bevy::{asset::Asset, ecs::component::Component, reflect::TypePath};

use super::sub::*;

// id	name	tier	armor_force	weight	sapien	beast	automatan	inanimate	mythical	spirit	element
#[derive(serde::Deserialize, Asset, TypePath, Debug, Clone, Component)]
pub struct MonsterData {
    pub id: u32,
    pub name: String,
    pub tier: Tier,
    pub armor_force: ArmorForce,
    pub weight: Weight,
    #[serde(deserialize_with = "bool_from_str")]
    pub sapien: bool,
    #[serde(deserialize_with = "bool_from_str")]
    pub beast: bool,
    #[serde(deserialize_with = "bool_from_str")]
    pub automatan: bool,
    #[serde(deserialize_with = "bool_from_str")]
    pub inanimate: bool,
    #[serde(deserialize_with = "bool_from_str")]
    pub mythical: bool,
    #[serde(deserialize_with = "bool_from_str")]
    pub spirit: bool,
    pub element: Element,
}

#[derive(Debug, serde::Deserialize, Default, Clone)]
pub enum ArmorForce {
    #[default]
    Infratry,
    Mounted,
    Flying,
}
