use serde::{Deserialize, Serialize};

use crate::ingame::resources::fixed::effect_type::EffectType;
use crate::ingame::resources::fixed::skill_type::SkillType;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Skill {
    pub name: SkillType,
    pub cooldown: f32,
    pub required_monster: f32,
    pub required_health: f32,
    pub effect: Option<EffectType>,
}
