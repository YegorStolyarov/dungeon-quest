use bevy_inspector_egui::Inspectable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Inspectable, PartialEq, Eq)]
pub enum Power {
    Strength,
    Intelligence,
}
