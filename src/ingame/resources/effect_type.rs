use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EffectType {
    None,
    SpeedUp,
    QuickUp,
    Focus,
    Slow,
    Disarm,
    ReduceDamage,
    Confuse,
    Stun,
    TimeToHunt,
    AnimalInstinct,
}
