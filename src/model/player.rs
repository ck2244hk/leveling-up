use std::collections::HashMap;

use bevy::{asset::Asset, ecs::component::Component, reflect::TypePath};
use serde::{Deserialize, Serialize};

use super::{sub::DropEquipment, ItemType};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Asset, TypePath, Debug, Component, Clone, Deserialize, Serialize)]
pub struct PlayerData {
    #[serde_as(as = "HashMap<_, Vec<_>>")]
    pub owned: HashMap<ItemType, Vec<DropEquipment>>,
    pub eq_weapon: Option<DropEquipment>,

    pub eq_armor: Option<DropEquipment>,

    pub eq_helmet: Option<DropEquipment>,

    pub eq_shoes: Option<DropEquipment>,
}
