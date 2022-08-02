use bevy_inspector_egui::Inspectable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Inspectable)]
pub struct Stats {
    pub health_points: f32,
    pub speed: f32,
    pub strength: f32,
    pub intelligence: f32,
    pub critical_chance: f32,
    pub dodge_chance: f32,
    pub restore_chance: f32,
}
