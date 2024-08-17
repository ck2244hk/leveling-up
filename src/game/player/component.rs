use bevy::utils::HashMap;

use crate::{
    game::preload::Weather,
    model::{helmet::HelmetData, shoes::ShoesData, ItemType},
};

use super::*;

pub trait Savable {
    fn save(&self) -> bool;
}

#[derive(Component)]
pub struct DecendingHero;

#[derive(Component, Reflect)]
pub struct Storage {
    pub items: std::collections::HashMap<ItemType, Vec<DropEquipment>>,
}

#[derive(Component, Default, Reflect, PartialEq)]
pub struct PlayerEnv(pub Option<Weather>);

impl Storage {
    pub fn empty() -> Self {
        Self {
            items: std::collections::HashMap::new(),
        }
    }
    pub fn new(player_data: &PlayerData) -> Self {
        Self {
            items: player_data.owned.clone(),
        }
    }

    pub fn is_owned(&self, data: &dyn EquipmentData) -> Option<DropEquipment> {
        if let Some(item_list) = self.items.get(&data.item_type()) {
            item_list.iter().find(|item| item.id == data.id()).cloned()
        } else {
            None
        }
    }
}

#[derive(Component, Clone)]
pub struct EquipmentBelt {
    pub weapon: Option<Weapon>,
    pub armor: Option<Armor>,
    pub helmet: Option<Helmet>,
    pub shoes: Option<Shoes>,
}

impl EquipmentBelt {
    pub fn new() -> Self {
        Self {
            weapon: None,
            armor: None,
            helmet: None,
            shoes: None,
        }
    }

    pub fn save() {}

    pub fn is_equipped(&self, data: &dyn EquipmentData) -> Option<DropEquipment> {
        match data.item_type() {
            ItemType::Weapon => self
                .weapon
                .as_ref()
                .filter(|item| item.id() == data.id())
                .map(|item| item.feature.clone()),
            ItemType::Armor => self
                .armor
                .as_ref()
                .filter(|item| item.id() == data.id())
                .map(|item| item.feature.clone()),
            ItemType::Helmet => self
                .helmet
                .as_ref()
                .filter(|item| item.id() == data.id())
                .map(|item| item.feature.clone()),
            ItemType::Shoes => self
                .shoes
                .as_ref()
                .filter(|item| item.id() == data.id())
                .map(|item| item.feature.clone()),
            ItemType::Others => None,
        }
    }

    pub fn equip_weapon(
        &mut self,
        assets: &Res<Assets<WeaponData>>,
        player: &PlayerData,
    ) -> &mut Self {
        if let Some(feature) = player.eq_weapon.clone() {
            assert_eq!(
                feature.item_type,
                ItemType::Weapon,
                "Metadata and Feature data do not match"
            );
            self.weapon = Weapon::from(assets, feature);
        }

        self
    }

    pub fn attack(&self) -> f32 {
        if let Some(weapon) = &self.weapon {
            weapon.attack()
        } else {
            0.
        }
    }

    pub fn defense(&self) -> f32 {
        if let Some(armor) = &self.armor {
            armor.defense()
        } else {
            0.
        }
    }
}

#[derive(Clone)]
pub struct Weapon {
    base: WeaponData,
    pub feature: DropEquipment,
}

impl Weapon {
    pub fn from(assets: &Res<Assets<WeaponData>>, eq_weapon: DropEquipment) -> Option<Weapon> {
        if let Some((_, weapon_data)) = assets.iter().find(|(_, record)| eq_weapon.id == record.id)
        {
            Some(Self {
                base: weapon_data.clone(),
                feature: eq_weapon,
            })
        } else {
            None
        }
    }

    pub fn id(&self) -> u32 {
        assert_eq!(
            self.base.id, self.feature.id,
            "Equipped Weapon Id does not match"
        );
        self.base.id
    }

    pub fn attack(&self) -> f32 {
        self.feature.level as f32
    }
}

#[derive(Clone)]
pub struct Armor {
    base: ArmorData,
    pub feature: DropEquipment,
}

impl Armor {
    fn new(base: ArmorData, feature: DropEquipment) -> Self {
        Self { base, feature }
    }

    pub fn from(assets: &Res<Assets<ArmorData>>, eq: DropEquipment) -> Option<Armor> {
        if let Some((_, data)) = assets.iter().find(|(_, record)| eq.id == record.id) {
            Some(Self {
                base: data.clone(),
                feature: eq,
            })
        } else {
            None
        }
    }

    pub fn id(&self) -> u32 {
        assert_eq!(
            self.base.id, self.feature.id,
            "Equipped Weapon Id does not match"
        );
        self.base.id
    }

    pub fn defense(&self) -> f32 {
        self.feature.level as f32
    }
}

#[derive(Clone)]
pub struct Helmet {
    base: HelmetData,
    pub feature: DropEquipment,
}

impl Helmet {
    pub fn from(assets: &Res<Assets<HelmetData>>, eq: DropEquipment) -> Option<Helmet> {
        if let Some((_, data)) = assets.iter().find(|(_, record)| eq.id == record.id) {
            Some(Self {
                base: data.clone(),
                feature: eq,
            })
        } else {
            None
        }
    }
    pub fn id(&self) -> u32 {
        assert_eq!(
            self.base.id, self.feature.id,
            "Equipped Weapon Id does not match"
        );
        self.base.id
    }
    pub fn defense(&self) -> f32 {
        self.feature.level as f32
    }
}

#[derive(Clone)]
pub struct Shoes {
    base: ShoesData,
    pub feature: DropEquipment,
}

impl Shoes {
    pub fn from(assets: &Res<Assets<ShoesData>>, eq: DropEquipment) -> Option<Shoes> {
        if let Some((_, data)) = assets.iter().find(|(_, record)| eq.id == record.id) {
            Some(Self {
                base: data.clone(),
                feature: eq,
            })
        } else {
            None
        }
    }
    pub fn id(&self) -> u32 {
        assert_eq!(
            self.base.id, self.feature.id,
            "Equipped Weapon Id does not match"
        );
        self.base.id
    }
    pub fn defense(&self) -> f32 {
        self.feature.level as f32
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Movable;

#[derive(Component, Reflect)]
pub struct Money(pub u64);

impl Money {
    pub fn add(&mut self, n: u64) {
        if self.0 < u64::MAX {
            self.0 += n;
        } else {
            self.0 = u64::MAX;
        }
    }

    pub fn get(&self) -> u64 {
        self.0
    }

    pub fn get_f32(&self) -> f32 {
        self.0 as f32
    }
}

#[derive(Component, Default, PartialEq, Eq)]
pub enum FacingSide {
    #[default]
    Right,
    Left,
}

#[derive(Default, PartialEq, Eq, Reflect, Debug)]
pub enum Movement {
    #[default]
    Idle,
    Run,
}

#[derive(Component, Default, PartialEq, Eq, Reflect, Debug)]
pub struct CurrentMovement(pub Movement);

#[derive(Component, Default, Reflect)]
pub struct Encounter {
    pub field: Option<u32>,
    pub percentage: f32,
    pub immune: bool,
}

impl Encounter {
    pub fn reset(&mut self) {
        self.percentage = 0.0;
    }

    pub fn update(&mut self, id: u32) {
        self.field = Some(id);
        self.percentage;
    }
}
