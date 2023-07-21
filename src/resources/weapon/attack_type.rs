use bevy_inspector_egui::InspectorOptions;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, InspectorOptions, Eq, PartialEq, Copy)]
pub enum AttackType {
    Swing,
    Shoot,
}
