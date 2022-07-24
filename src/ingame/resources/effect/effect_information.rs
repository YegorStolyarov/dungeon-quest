use serde::{Deserialize, Serialize};

use crate::ingame::resources::effect::effect_type::EffectType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EffectInformation {
    pub name: EffectType,
    pub duration: f32,
    pub bonus: f32,
    pub trigger_chance: Option<f32>,
}
