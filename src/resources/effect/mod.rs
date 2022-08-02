use serde::{Deserialize, Serialize};

pub mod effect_type;

use effect_type::EffectType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Effect {
    pub name: EffectType,
    pub duration: i64,
    pub bonus: f32,
}
