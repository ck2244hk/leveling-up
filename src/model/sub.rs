use bevy::{ecs::component::Component, reflect::Reflect};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::{EquipmentData, ItemType};

#[serde_as]
#[derive(
    Component, Debug, Default, Clone, Reflect, Hash, PartialEq, Eq, Deserialize, Serialize,
)]
pub struct DropEquipment {
    pub id: u32,
    pub level: u32,
    pub name: String,
    pub image: Option<String>,
    pub description: String,
    pub item_type: ItemType,
}

// all item ability is related to it assigned level
impl DropEquipment {
    pub fn set_level(&mut self, lv: u32) {
        self.level = self.level.max(lv);
    }

    // for weapon true damage
    pub fn weapon(&self) -> f32 {
        self.level as f32
    }

    // for armor damage reduction
    pub fn armor(&self) -> f32 {
        self.level as f32
    }
}

#[derive(Debug, serde::Deserialize, Default, Clone)]
pub enum Range {
    Long,
    #[default]
    Mid,
    Close,
}

#[derive(Debug, serde::Deserialize, Default, Clone)]
pub enum Weight {
    Light,
    #[default]
    Balance,
    Heavy,
}

#[derive(Debug, serde::Deserialize, Default, Clone, PartialEq)]
pub enum Element {
    Fire,
    Water,
    Earth,
    #[default]
    Neutral,
}

#[derive(Debug, serde::Deserialize, Default, Clone)]
pub struct Tier(String);

impl Tier {
    pub fn contain(&self, tier: &Tier) -> bool {
        let (container_num, container_letter) = self.split_alphanumberic();
        let (num, letter) = tier.split_alphanumberic();
        let num_pass = num.is_none() || container_num.is_some_and(|a| num.is_some_and(|b| a == b));
        let letter_pass = letter.is_empty() || container_letter.find(&letter).is_some();
        num_pass && letter_pass
    }

    fn split_alphanumberic(&self) -> (Option<i32>, String) {
        let mut n = String::new();
        let mut s = String::new();
        for character in self.0.chars() {
            if character.is_alphabetic() {
                s.push(character);
            } else if character.is_numeric() {
                n.push(character);
            }
        }

        (n.parse().ok(), s)
    }
}
