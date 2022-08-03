use bevy::prelude::*;

use crate::resources::{
    monster::monster_spawn_controller::MonsterSpawnController,
    player::player_dungeon_stats::PlayerDungeonStats,
};

pub fn cleared_room_check(
    monster_spawn_controller: Res<MonsterSpawnController>,
    mut player_dungeon_stats: ResMut<PlayerDungeonStats>,
) {
    if monster_spawn_controller.require_monster == monster_spawn_controller.killed_monsters {
        if !player_dungeon_stats.is_room_cleared {
            player_dungeon_stats.is_room_cleared = true;
        }
    }
}
