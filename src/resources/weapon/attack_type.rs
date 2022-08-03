use bevy_inspector_egui::Inspectable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Inspectable, Eq, PartialEq, Copy)]
pub enum AttackType {
    Swing,
    Shoot,
}
