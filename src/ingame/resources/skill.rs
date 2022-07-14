use serde::{Deserialize, Serialize};

use crate::ingame::resources::effect_type::EffectType;
use crate::ingame::resources::skill_type::SkillType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Skill {
    name: SkillType,
    cooldown: f32,
    required_monster: f32,
    required_health: f32,
    effect: EffectType,
}
