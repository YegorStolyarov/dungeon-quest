use bevy_inspector_egui::Inspectable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Inspectable)]
pub struct Bullet {
    pub width: f32,
    pub height: f32,
    pub speed: f32,
    pub scale: f32,
}
