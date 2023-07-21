use bevy_inspector_egui::InspectorOptions;
use bevy::prelude::*;

use crate::resources::dungeon::position::Position;

#[derive(Resource, InspectorOptions, Default)]
pub struct PlayerDungeonStats {
    pub current_floor_index: usize,
    pub current_room_position: Position,
    pub is_room_cleared: bool,
}
