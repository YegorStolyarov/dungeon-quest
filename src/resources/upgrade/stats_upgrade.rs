use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StatsUpgrade {
    pub max_health_bonus: Option<f32>,
    pub speed_percent_bonus: Option<f32>,
    pub critical_chance_bonus: Option<f32>,
    pub strength_bonus: Option<f32>,
    pub intelligence_bonus: Option<f32>,
    pub dodge_chance_bonus: Option<f32>,
    pub restore_chance_bonus: Option<f32>,
}
