use serde::{Deserialize, Serialize};

pub mod effect_controller;
pub mod effect_type;

use effect_type::EffectType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Effect {
    name: EffectType,
    duration: f32,
    bonus: f32,
    trigger_chance: f32,
}
