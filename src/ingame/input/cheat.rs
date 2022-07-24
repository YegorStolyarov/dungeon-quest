use bevy::prelude::*;

use crate::ingame::resources::dungeon::Dungeon;
use crate::ingame::resources::player::player_dungeon_stats::PlayerDungeonStats;

pub fn cheat_move(
    mut player_dungeon_stats: ResMut<PlayerDungeonStats>,
    keyboard_input: Res<Input<KeyCode>>,
    mut dungeon: ResMut<Dungeon>,
) {
    if keyboard_input.pressed(KeyCode::C) {
        let current_position = dungeon.current_floor.current_position;
        dungeon
            .current_floor
            .cleared_positions
            .insert(current_position, 1);
        player_dungeon_stats.is_room_cleared = true;
    }
}
