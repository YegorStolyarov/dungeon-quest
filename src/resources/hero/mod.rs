use serde::{Deserialize, Serialize};

use crate::resources::skill::skill_type::SkillType;
use crate::resources::weapon::weapon_type::WeaponType;

pub mod gender;
pub mod hero_class;
pub mod power;
pub mod stats;

use hero_class::HeroClass;
use power::Power;
use stats::Stats;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hero {
    pub hero_class: HeroClass,
    pub stats: Stats,
    pub power: Power,
    pub weapon: WeaponType,
    pub skill: SkillType,
}
