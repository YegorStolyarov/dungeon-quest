use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EffectBonus {
    speed_bonus: f32,
    critical_chance_bonus: f32,
    dodge_chance_bonus: f32,
    restore_chance_bonus: f32,
    damage_bonus: f32,
}
