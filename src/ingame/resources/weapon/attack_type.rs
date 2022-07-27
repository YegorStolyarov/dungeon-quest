use bevy_inspector_egui::Inspectable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Inspectable, Eq, PartialEq)]
pub enum AttackType {
    Swing,
    Shoot,
    Throw,
}
