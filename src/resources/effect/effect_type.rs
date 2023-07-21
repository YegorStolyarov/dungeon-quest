use bevy_inspector_egui::InspectorOptions;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, InspectorOptions, Copy, Hash, Eq)]
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
