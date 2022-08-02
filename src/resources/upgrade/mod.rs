use serde::{Deserialize, Serialize};

mod effect_upgrade;
mod skill_upgrade;
mod stats_upgrade;
pub mod upgrade_controller;
pub mod upgrade_type;

use effect_upgrade::EffectUpgrade;
use skill_upgrade::SkillUpgrade;
use stats_upgrade::StatsUpgrade;
use upgrade_type::UpgradeType;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Upgrade {
    upgrade_type: UpgradeType,
    pub skill_upgrade: Option<SkillUpgrade>,
    pub stats_upgrade: Option<StatsUpgrade>,
    pub effect_upgrade: Option<EffectUpgrade>,
}
