use crate::ingame::resources::dungeon::position::Position;

pub struct PlayerDungeonStats {
    pub current_room_position: Position,
    pub is_room_cleared: bool,
}
