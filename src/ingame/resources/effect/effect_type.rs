use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EffectType {
    SpeedUp,
    EvasionUp,
    Focus,
    Slow,
    ReduceDamage,
    Disarm,
    Confuse,
    Stun,
}
