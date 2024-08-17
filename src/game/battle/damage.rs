use crate::game::{character::component::BaseStates, player::component::EquipmentBelt};

use super::*;

#[derive(Debug, Clone, Reflect, Copy)]
pub enum DamageOut {
    NormalHit(f32),
    CriticalHit(f32),
}

impl DamageOut {
    pub fn get(&self) -> f32 {
        match self {
            DamageOut::NormalHit(damage) => *damage,
            DamageOut::CriticalHit(damage) => *damage,
        }
    }

    pub fn set(&mut self, damage: f32) {
        match self {
            DamageOut::NormalHit(ref mut value) | DamageOut::CriticalHit(ref mut value) => {
                *value = damage;
            }
        }
    }

    pub fn add(&mut self, damage: f32) {
        match self {
            DamageOut::NormalHit(ref mut value) | DamageOut::CriticalHit(ref mut value) => {
                *value = 0_f32.max(f32::MAX.min(*value + damage));
            }
        }
    }

    pub fn multiply(&mut self, multiplier: f32) {
        match self {
            DamageOut::NormalHit(ref mut value) | DamageOut::CriticalHit(ref mut value) => {
                *value *= multiplier;
            }
        }
    }
}

pub struct DamageBuilder<'a> {
    attacker: &'a BaseStates,
    defender: &'a BaseStates,
    damage: DamageOut,
}

impl<'a> DamageBuilder<'a> {
    // 1. M1
    pub fn build(attacker: &'a BaseStates, defender: &'a BaseStates) -> Self {
        Self {
            attacker,
            defender,
            damage: DamageOut::NormalHit(1. + 2_f32.powf(attacker.lv_f32() / 20.)),
        }
    }

    // 2. Cal Critical
    pub fn crit_hit(&mut self, class: Option<&HeroClass>) -> &mut Self {
        let seed = random::<f32>() * 100.;
        if seed <= self.attacker.crit_rate(class) {
            self.damage =
                DamageOut::CriticalHit(self.damage.get() * self.attacker.crit_multiplier());
        }

        self
    }

    //3. scale output damage with class
    pub fn attacker_class_scaling(&mut self) -> &mut Self {
        // match attacker_class {
        //     HeroClass::Paladin => self.damage.multiply(0.5),
        //     HeroClass::Warrior => self.damage.multiply(1.5),
        //     HeroClass::Rogue => self.damage.multiply(0.5),
        // };
        self.damage.multiply(1.25);

        self
    }

    //4. take defense if normal plus class scaling to defense Eq
    pub fn defense(&mut self) -> &mut Self {
        let class_mul: fn(f32) -> f32 = |x| x;
        // if let Some(defender_class) = defender_class {
        //     // match defender_class {
        //     //     HeroClass::Paladin => |x| x.powf(2.5) / 1.5,
        //     //     HeroClass::Warrior => |x| x.powf(0.5) * 1.5,
        //     //     HeroClass::Rogue => |x| x.powf(2.),
        //     // }

        // } else {
        //     |x| x
        // };

        if let DamageOut::NormalHit(ref mut damage) = self.damage {
            *damage = *damage
                * class_mul(
                    self.attacker.str() / (self.defender.dex() + self.defender.dex().powf(0.5)),
                )
                + 1.
            // self.damage.set(
            //     self.damage
            //         * class_mul(
            //             self.attacker.str() / (self.defender.dex() + self.defender.dex().powf(0.5)),
            //         )
            //         + 1.,
            // )
        };

        self
    }

    pub fn noise(&mut self) -> &mut Self {
        self.damage.multiply(1_f32 + random::<f32>());

        self
    }

    pub fn weapon(&mut self, weapon: &EquipmentBelt) -> &mut Self {
        self.damage.add(weapon.attack());
        self
    }

    pub fn armor(&mut self, armor: &EquipmentBelt) -> &mut Self {
        self.damage.add(-armor.defense());
        self
    }

    // Always add this to the end of the damage
    pub fn innate_damage(&mut self) -> &mut Self {
        self.damage.add(1.);
        self
    }

    // Hero always has the ability to 5 level over
    pub fn hero_additional_damage(&mut self) -> &mut Self {
        self.damage.add(1.18920711500272);
        self
    }

    pub fn get_damage(&self) -> DamageOut {
        self.damage
    }

    pub fn get_damage_f32(&mut self) -> f32 {
        self.damage.get()
    }
}

#[test]
fn test_m1_damage() {
    let attacker: BaseStates = BaseStates::new_hero();
    let defender: BaseStates = BaseStates::new_monster(1);
    let mut level1 = DamageBuilder::build(&attacker, &defender);

    assert_eq!(
        level1.get_damage_f32(),
        1.07177346253629 + 1.,
        "m1 not accurate"
    );

    // DamageBuilder::build(-999999);
    // DamageBuilder::build(99999999);
    // DamageBuilder::build(1);
    // DamageBuilder::build(1000);
}

#[test]
fn test_damage_out() {
    let mut damage = DamageOut::NormalHit(2.);
    damage.set(6.);
    assert_eq!(damage.get(), 6., "Damage Out set feature has problem");

    damage.add(30.);
    assert_eq!(damage.get(), 36., "Damage Out add feature has problem");

    damage.multiply(6.);
    assert_eq!(
        damage.get(),
        6_f32 * 36_f32,
        "Damage out mul feature has problem"
    );
}

// Todo Writing unit test
// #[test]
// fn test_paladin_damage() {
//     struct DamageTestObject {
//         strength: Strength,
//         hp: HealthPoint,
//         def: Defense,
//         crit: Critical,
//         lv: Level,
//         class: Option<HeroClass>,
//     }

//     let test1_att = DamageTestObject {
//         strength: Strength::new(300.),
//         hp: HealthPoint::new(1.),
//         def: Defense::new(300.),
//         crit: Critical::new(300.),
//         lv: Level::new(300),
//         class: Some(HeroClass::Paladin),
//     };

//     // DamageOut::build(attacker_str, attacker_lv);
//     // DamageIn::build(damage_out, defender_defense, attacker_str, hero_class);
// }
