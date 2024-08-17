use std::str::FromStr;

use bevy::{ecs::component::Component, reflect::Reflect};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use self::sub::{DropEquipment, Tier};

pub mod armor;
pub mod helmet;
pub mod monster;
pub mod player;
pub mod shoes;
pub mod sub;
pub mod weapon;

#[serde_as]
#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Reflect, Serialize, Deserialize)]
pub enum ItemType {
    Weapon,
    Armor,
    Helmet,
    Shoes,
    #[default]
    Others,
}

impl FromStr for ItemType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Weapon" => Ok(Self::Weapon),
            "Armor" => Ok(Self::Armor),
            "Helmet" => Ok(Self::Helmet),
            "Shoes" => Ok(Self::Shoes),
            "Others" => Ok(Self::Others),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Weapon => write!(f, "Weapon"),
            Self::Armor => write!(f, "Armor"),
            Self::Helmet => write!(f, "Helmet"),
            Self::Shoes => write!(f, "Shoes"),
            _ => write!(f, "Others"),
        }
    }
}

#[derive(Component, Clone, Default)]
pub struct Id(u32);

impl Id {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
    pub fn get(&self) -> u32 {
        self.0
    }
}

pub trait EquipmentData {
    fn name(&self) -> String;
    fn id(&self) -> u32;
    fn item_type(&self) -> ItemType;
    fn tier(&self) -> Tier;
    fn description(&self) -> String;
    fn to_drop_equipment(&self, lv: u32) -> DropEquipment {
        DropEquipment {
            id: self.id(),
            level: lv,
            name: self.name(),
            image: None,
            item_type: self.item_type(),
            description: self.description(),
        }
    }
}
