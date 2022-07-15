use serde::{Deserialize, Serialize};

use crate::ingame::resources::fixed::effect_bonus::EffectBonus;
use crate::ingame::resources::fixed::effect_type::EffectType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Effect {
    name: EffectType,
    duration: f32,
    effect_bonus: Option<EffectBonus>,
}
