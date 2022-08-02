use serde::{Deserialize, Serialize};

use crate::resources::skill::skill_type::SkillType;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SkillUpgrade {
    pub name: SkillType,
    pub duration_bonus: Option<i64>,
    pub cooldown_reduce: Option<i64>,
    pub require_monsters_reduce: Option<u32>,
    pub speed_percent_bonus: Option<f32>,
    pub critical_chance_bonus: Option<f32>,
    pub dodge_chance_bonus: Option<f32>,
    pub restore_chance_bonus: Option<f32>,
}
