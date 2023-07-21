use bevy_inspector_egui::InspectorOptions;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, InspectorOptions, PartialEq, Eq)]
pub enum Power {
    Strength,
    Intelligence,
}
