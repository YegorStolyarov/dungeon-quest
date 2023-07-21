use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;

#[derive(Component, PartialEq, Eq, Debug, InspectorOptions)]
pub enum BlockType {
    None,
    WallTop,
    WallBottom,
    WallLeft,
    WallRight,
}
