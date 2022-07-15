use serde::{Deserialize, Serialize};

use crate::ingame::resources::fixed::hero_class::HeroClass;
use crate::ingame::resources::fixed::power::Power;
use crate::ingame::resources::fixed::skill_type::SkillType;
use crate::ingame::resources::fixed::stats::Stats;
use crate::ingame::resources::fixed::weapon_type::WeaponType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hero {
    pub hero_class: HeroClass,
    pub stats: Stats,
    pub power: Power,
    pub weapon: WeaponType,
    pub skill: SkillType,
}
