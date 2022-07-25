use serde::{Deserialize, Serialize};

use crate::ingame::resources::effect::effect_type::EffectType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EffectInformation {
    pub name: EffectType,
    pub duration: u64,
    pub bonus: f32,
}
