use bevy_inspector_egui::InspectorOptions;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, InspectorOptions, Copy)]
pub struct Bullet {
    pub width: f32,
    pub height: f32,
    pub speed: f32,
    pub scale: f32,
}
