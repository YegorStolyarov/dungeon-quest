use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EffectType {
    SpeedUp,
    QuickUp,
    Focus,
    Slow,
    ReduceDamage,
    Disarm,
    Confuse,
    Stun,
    TimeToHunt,
    AnimalInstinct,
}
