use serde::{Deserialize, Serialize};

pub mod skill_type;

use skill_type::SkillType;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Skill {
    pub name: SkillType,
    pub cooldown: f32,
    pub required_monster: f32,
    pub required_health: f32,
}
