use serde::{Deserialize, Serialize};

use crate::ingame::resources::fixed::effect_type::EffectType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TriggerEffect {
    chance_to_trigger: f32,
    effect: EffectType,
}
