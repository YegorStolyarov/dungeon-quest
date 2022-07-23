use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, PartialEq, Eq, Debug, Inspectable)]
pub enum BlockType {
    None,
    WallTop,
    WallBottom,
    WallLeft,
    WallRight,
}
