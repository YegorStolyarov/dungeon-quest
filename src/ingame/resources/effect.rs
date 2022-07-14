use crate::ingame::resources::effect_type::EffectType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Effect {
    name: EffectType,
    duration: f32,
    speed_bonus: f32,
    critical_chance_bonus: f32,
    dodge_chance_bonus: f32,
    restore_chance_bonus: f32,
    damage_bonus: f32,
    disarm_status: bool,
    confuse_status: bool,
    stun_status: bool,
    target_status: bool,
}
