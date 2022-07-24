use serde::{Deserialize, Serialize};

use crate::ingame::resources::effect::effect_type::EffectType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EffectInformation {
    name: EffectType,
    duration: f32,
    bonus: f32,
    trigger_chance: Option<f32>,
}
