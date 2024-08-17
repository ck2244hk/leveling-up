use std::slice::Iter;

use bevy::{
    core::Name,
    ecs::{bundle::Bundle, component::Component},
    log::info,
    prelude::ImageBundle,
    reflect::Reflect,
    sprite::{SpriteBundle, SpriteSheetBundle, TextureAtlas},
    time::{Timer, TimerMode},
    ui::node_bundles::AtlasImageBundle,
};

use crate::{
    animation::component::{AnimationIndices, AnimationTimer},
    error::UserInputError,
    game::battle_scene::BattleHeroSprite,
    model::sub::DropEquipment,
};

const C1: f64 = 577.1124142;

#[derive(Component, Reflect, Clone, Default)]
pub struct BaseStates {
    lv: u32,
    dex: f32,
    str: f32,
    agi: f32,
    hp: f32,
    exp: f64,
}

impl BaseStates {
    pub fn new_hero() -> Self {
        let lv = 1;
        Self {
            lv,
            dex: lv as f32 / 3.,
            str: lv as f32 / 3.,
            agi: lv as f32 / 3.,
            hp: 2.,
            exp: Self::get_lv_total_exp(1),
        }
    }

    pub fn new_monster(lv: u32) -> Self {
        Self {
            lv,
            dex: lv as f32 / 3.,
            str: lv as f32 / 3.,
            agi: lv as f32 / 3.,
            hp: 0.,
            exp: Self::get_lv_total_exp(lv),
        }
    }

    pub fn dex(&self) -> f32 {
        self.dex
    }

    pub fn str(&self) -> f32 {
        self.str
    }

    pub fn agi(&self) -> f32 {
        self.agi
    }

    pub fn lv(&self) -> u32 {
        self.lv
    }

    pub fn hp(&self) -> f32 {
        self.hp + 2_f32.powf(self.lv as f32 / 20.) * 3.
    }

    pub fn lv_f32(&self) -> f32 {
        self.lv as f32
    }

    pub fn get_total_point(&self) -> f32 {
        self.str + self.dex + self.agi
    }

    pub fn total_exp_req_for_next_lv(&self) -> f64 {
        100. * (2_f64.powf((self.lv as f64 + 1.) / 10.) - 1.) / (2_f64.powf(0.1) - 1.)
    }

    pub fn exp_req_for_next_lv(&self) -> f32 {
        100. * 2_f32.powf((self.lv as f32 + 1.) / 10.)
    }

    pub fn total_exp(&self) -> f64 {
        self.exp
    }

    pub fn exp_after_lv_up(&self) -> f32 {
        (self.total_exp()
            - (self.total_exp_req_for_next_lv() - 100. * 2_f64.powf((self.lv as f64) / 10.)))
            as f32
    }

    // The exp hero earned when killing the monster
    pub fn exp_gain(&mut self, monster_lv: u32) -> u32 {
        self.exp += Self::exp_drop_by_monster(monster_lv);
        let diff = self.update_lv_with_exp();
        self.distribut_state_point();
        diff
    }

    pub fn crit_multiplier(&self) -> f32 {
        let mul = 2. + self.str / self.get_total_point() * self.agi() / self.get_total_point() * 3.;

        info!("Critical Multiplier: {}", mul);

        mul
    }

    pub fn crit_rate(&self, player_class: Option<&HeroClass>) -> f32 {
        let max_rate: f32 = if let Some(class) = player_class {
            match class {
                HeroClass::Rogue => 50.,
                _ => 40.,
            }
        } else {
            // Monster
            0.
        };

        max_rate * self.lv_f32() / self.get_total_point()
    }

    // run this everytime exp has changed
    // return how many level been upgraded since
    fn update_lv_with_exp(&mut self) -> u32 {
        let lv = (10. * (self.exp / 100. * (2_f64.powf(0.1) - 1.) + 1.).log2()).floor() as u32;
        let diff = lv - self.lv;
        self.lv = lv;
        diff
    }

    fn get_lv_total_exp(lv: u32) -> f64 {
        if lv == 1 {
            return 0_f64;
        }

        let mut total = 0.;
        for i in 1..=(lv - 1) {
            total += 2_f64.powf(i as f64 / 10.) * 100.;
        }

        total
    }

    pub fn exp_drop_by_monster(lv: u32) -> f64 {
        C1 * 2_f64.powf(lv as f64 / 10.)
    }

    fn distribut_state_point(&mut self) {
        self.lv();
        self.str = self.lv_f32() / 3. * 2.;
        self.dex = self.lv_f32() / 3.;
        self.agi = 1.;
    }
}

#[derive(Component, Reflect, Clone, Default)]
pub struct Defense(f32);

impl Defense {
    pub fn new(def: f32) -> Self {
        Defense(def)
    }

    pub fn get(&self) -> f32 {
        self.0
    }
    pub fn upgrade(&mut self, point: u32) {
        self.0 += point as f32;
    }
}

#[derive(Component, Clone, Copy, Reflect, Default)]
pub struct HealthPoint(f32);

impl HealthPoint {
    pub fn hero() -> Self {
        Self::new(2.)
    }
    pub fn monster() -> Self {
        Self::new(0.)
    }
}

impl HealthPoint {
    pub fn new(point: f32) -> Self {
        Self(point)
    }

    pub fn get_hp(&self, lv: &Level) -> f32 {
        self.0 + 2_f32.powf(lv.get() as f32 / 20.) * 3.
    }

    // pub fn upgrade(&mut self, point: u32) {
    //     self.0 += point as f32;
    // }
    // pub fn get(&self) -> f32 {
    //     self.0
    // }
}

#[derive(Component, Reflect, Clone, Default)]
pub struct Strength(f32);

impl Strength {
    // pub fn m1(&self, lv: &Level) -> f32 {
    //     (self.0 + 2_f32.powf(lv.get() as f32 / 20.)) * (1. + random::<f32>() / 10.)
    // }

    pub fn new(point: f32) -> Self {
        Self(point)
    }

    pub fn upgrade(&mut self, point: u32) {
        self.0 += point as f32;
    }
    pub fn get(&self) -> f32 {
        self.0
    }
}

#[derive(Component, Default, Reflect, Clone)]
pub struct Critical(f32);

impl Critical {
    pub fn new(point: f32) -> Self {
        Self(point)
    }

    pub fn damage_multiplier(&self, lv: &Level, attacker_str: &Strength) -> f32 {
        let mul = 2.
            + attacker_str.0 / lv.get_total_point() as f32 * self.0 / lv.get_total_point() as f32
                * 3.;

        info!("Critical Multiplier: {}", mul);

        mul
    }

    pub fn rate(&self, lv: &Level, player_class: Option<&HeroClass>) -> f32 {
        let max_rate: f32 = if let Some(class) = player_class {
            match class {
                HeroClass::Rogue => 50.,
                _ => 40.,
            }
        } else {
            // Monster
            40.
        };

        max_rate * self.0 / lv.get_total_point() as f32
    }

    pub fn upgrade(&mut self, point: u32) {
        self.0 += point as f32;
    }
    pub fn get(&self) -> f32 {
        self.0
    }
}

// Deprecated
#[derive(Component, Reflect, Clone, Default)]
pub struct Level(u64);

impl Level {
    pub fn new(lv: u64) -> Self {
        Level(lv)
    }
    pub fn get_f32(&self) -> f32 {
        self.0 as f32
    }

    pub fn get(&self) -> u64 {
        self.0
    }

    pub fn get_total_point(&self) -> u64 {
        self.0
    }

    pub fn total_exp_req_for_next_lv(&self) -> f32 {
        2_f32.powf(self.0 as f32 / 10.) * 100.
    }

    pub fn upgrade(&mut self) {
        if self.0 < u64::MAX {
            self.0 += 1;
        } else {
            self.0 = u64::MAX;
        }
    }

    pub fn get_total_exp(&self) -> f32 {
        if self.0 == 1 {
            return 0_f32;
        }

        let mut total: f32 = 0.;
        for i in 1..=(self.0 - 1) {
            total += 2_f32.powf(i as f32 / 10.) * 100.;
        }

        total
    }

    pub fn exp_gain(&self) -> f64 {
        C1 * 2_f64.powf(self.0 as f64 / 10.)
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect)]
pub enum HeroClass {
    Paladin,
    Warrior,
    Rogue,
}

impl HeroClass {
    pub fn iterator() -> Iter<'static, HeroClass> {
        static CLASSES: [HeroClass; 3] = [HeroClass::Paladin, HeroClass::Warrior, HeroClass::Rogue];
        CLASSES.iter()
    }

    pub fn to_string(&self) -> String {
        match self {
            HeroClass::Paladin => "Paladin".to_string(),
            HeroClass::Warrior => "Warrior".to_string(),
            HeroClass::Rogue => "Rogue".to_string(),
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum MonsterType {
    #[default]
    Monster1,
    HellBat,
    Monster3,
}

impl MonsterType {
    pub fn iterator() -> Iter<'static, MonsterType> {
        static CLASSES: [MonsterType; 3] = [
            MonsterType::Monster1,
            MonsterType::HellBat,
            MonsterType::Monster3,
        ];
        CLASSES.iter()
    }

    pub fn to_string(&self) -> String {
        match self {
            MonsterType::Monster1 => "Monster1".to_string(),
            MonsterType::HellBat => "HellBat".to_string(),
            MonsterType::Monster3 => "Monster3".to_string(),
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "Monster1" => MonsterType::Monster1,
            "HellBat" => MonsterType::HellBat,
            "Monster3" => MonsterType::Monster3,
            _ => panic!("Monster type does not exist"),
        }
    }
}

#[derive(Component)]
pub struct HeroSprite;

#[derive(Bundle)]
pub struct HeroSpriteBundle {
    pub name: Name,
    pub hero_sripte: HeroSprite,
    pub sprite_sheet_bundle: SpriteBundle,
    pub texture_atlas: TextureAtlas,
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
    pub hero_class: HeroClass,
}

#[derive(Bundle)]
pub struct HeroSpriteNodeBundle {
    pub name: Name,
    pub hero_sripte: BattleHeroSprite,
    pub sprite_sheet_bundle: ImageBundle,
    pub texture_atlas: TextureAtlas,
    pub hero_class: HeroClass,
    // pub action: BattleHeroAction,
}

#[derive(Bundle)]
pub struct MonsterSpriteNodeBundle {
    pub name: Name,
    pub sprite_sheet_bundle: ImageBundle,
    pub texture_atlas: TextureAtlas,
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
    pub monster_type: MonsterType,
    pub ui_monster_sprite: BattleMonsterSprite,
}

#[derive(Bundle)]
pub struct MonsterSpriteBundle {
    pub name: Name,
    pub monster_sprite: MonsterSprite,
    pub sprite_sheet_bundle: SpriteBundle,
    pub texture_atlas: TextureAtlas,
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
    pub monster_type: MonsterType,
}

#[derive(Component)]
pub struct MonsterSprite;

#[derive(Component)]
pub struct BattleMonsterSprite;

#[derive(Component, PartialEq, Eq, Reflect)]
pub struct DespawnBossFlag;

#[derive(Component, Default)]
pub struct Experience(pub f32);

#[derive(Component)]
pub struct Hero {}

#[derive(Component, Default)]
pub struct StatePoint(u32);

impl StatePoint {
    pub fn plus_one(&mut self) {
        self.0 += 1
    }
    pub fn plus(&mut self, point: u32) {
        self.0 += point
    }

    // pub fn minus(&mut self) -> Result<&StatePoint, UserInputError> {
    //     if self.0 == 0 {
    //         Err(UserInputError("Not Enough State Point".to_string()))
    //     } else {
    //         self.0 -= 1;
    //         Ok(self)
    //     }
    // }

    pub fn half(&mut self) -> Result<u32, UserInputError> {
        if self.0 == 0 {
            Err(UserInputError("Not Enough State Point".to_string()))
        } else if self.0 == 1 {
            self.0 -= 1;
            Ok(1)
        } else {
            let half = (self.0 as f32 / 2.0).floor() as u32;
            self.0 -= half;
            Ok(half)
        }
    }

    pub fn all(&mut self) -> Result<u32, UserInputError> {
        if self.0 == 0 {
            Err(UserInputError("Not Enough State Point".to_string()))
        } else {
            let old = self.0;
            self.0 = 0;
            Ok(old)
        }
    }

    pub fn get(&self) -> u32 {
        self.0
    }
}

#[derive(Component)]
pub struct Turns(u32);

impl Default for Turns {
    fn default() -> Self {
        Self(12)
    }
}

impl Turns {
    pub fn get(&self) -> u32 {
        self.0
    }
    pub fn minus(&mut self) {
        if self.0 > u32::MIN {
            self.0 = self.0 - 1
        }
    }
}

#[derive(Component, Clone, Default)]
pub struct Boss;

#[derive(Component)]
pub struct BossRespawnTimer(pub Timer);

impl Default for BossRespawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(2., TimerMode::Once))
    }
}

#[derive(Component)]
pub struct BossRespawnBlinker(pub Timer);

impl Default for BossRespawnBlinker {
    fn default() -> Self {
        Self(Timer::from_seconds(0.2, TimerMode::Repeating))
    }
}

#[derive(Component, Default)]
pub struct Bag(pub Vec<DropEquipment>);

impl Bag {
    pub fn push(&mut self, equipment: &DropEquipment) {
        self.0.push(equipment.clone());
    }
}

#[test]
fn level_system_validation() {
    let level_40 = Level(40);
    let level_1 = Level(1);
    assert_eq!(level_1.total_exp_req_for_next_lv().floor(), 107.);
    assert_eq!(level_1.get_total_exp().floor(), 0.);

    assert_eq!(level_40.total_exp_req_for_next_lv().floor(), 1600.);
    assert_eq!(level_40.get_total_exp().floor(), 20799.);
}

// Todo Unit Testing
// #[test]
// fn critical_rate_validation() {
//     let crit_0 = Critical(0.);
//     let crit_all = Critical(60.);
//     let crit_over = Critical(60.);

//     let mul = crit_0.damage_multiplier(&Level(1), &Strength(1.));

//     // when crit point equal to 0, mul and rate eq to ?
//     // assert_eq!(crit_0.damage_multiplier(&Level(1), &Strength(1.)), 1.)

//     // when crit point evenly distributed, mul and rate?
//     // when crit point monopolized
// }
