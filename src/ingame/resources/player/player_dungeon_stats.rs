use crate::ingame::resources::dungeon::position::Position;
use bevy_inspector_egui::Inspectable;

#[derive(Inspectable, Default)]
pub struct PlayerDungeonStats {
    pub current_floor_index: usize,
    pub current_room_position: Position,
    pub is_room_cleared: bool,
}
