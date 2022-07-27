use serde::{Deserialize, Serialize};

use crate::ingame::resources::effect::effect_type::EffectType;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EffectUpgrade {
    pub name: EffectType,
    pub duration_bonus: Option<i64>,
    pub duration_reduce: Option<i64>,
    pub speed_percent_bonus: Option<f32>,
    pub speed_percent_reduce: Option<f32>,
    pub critical_chance_bonus: Option<f32>,
    pub dodge_chance_bonus: Option<f32>,
}
