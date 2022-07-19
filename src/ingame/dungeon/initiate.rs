use bevy::prelude::*;

use crate::ingame::resources::dungeon::Dungeon;
use crate::ingame::resources::player::player_dungeon_stats::PlayerDungeonStats;

pub fn initiate_dungeon(mut commands: Commands) {
    let dungeon = Dungeon::new();

    let player_dungeon_stats = PlayerDungeonStats {
        current_room_position: dungeon.current_floor.current_position,
        is_room_cleared: true,
        current_floor_index: 0,
    };

    commands.insert_resource(dungeon);
    commands.insert_resource(player_dungeon_stats);
}
