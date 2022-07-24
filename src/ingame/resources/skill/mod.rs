use serde::{Deserialize, Serialize};

pub mod skill_type;

use skill_type::SkillType;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Skill {
    pub name: SkillType,
    pub duration: Option<u64>,
    pub cooldown: Option<u64>,
    pub require_health_points: Option<f32>,
    pub require_monsters: Option<u32>,
    pub speed_percent_bonus: Option<f32>,
    pub damge_precent_bonus: Option<f32>,
    pub critical_chance_bonus: Option<f32>,
    pub dodge_chance_bonus: Option<f32>,
    pub restore_chance_bonus: Option<f32>,
}
