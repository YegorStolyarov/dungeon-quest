use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::ingame::resources::dungeon::ladder::Ladder;
use crate::ingame::resources::dungeon::Dungeon;
use crate::ingame::resources::player::player_dungeon_stats::PlayerDungeonStats;
use crate::ingame::resources::player::Player;

pub fn ladder_interaction_handle_system(
    mut player_query: Query<(&Transform, &TextureAtlasSprite), With<Player>>,
    mut ladder_query: Query<(&Transform, &Sprite), (With<Ladder>, Without<Player>)>,
    mut player_dungeon_stats: ResMut<PlayerDungeonStats>,
    mut dungeon: ResMut<Dungeon>,
) {
    let current_position = dungeon.current_floor.current_position;
    let end_room_position = dungeon.current_floor.end_room_position;

    if current_position == end_room_position && player_dungeon_stats.is_room_cleared {
        let (player_transform, player_sprite) = player_query.single_mut();
        let (ladder_transform, ladder_sprite) = ladder_query.single_mut();

        let p_translation = player_transform.translation;
        let p_size = player_sprite.custom_size.unwrap();
        let l_translation = ladder_transform.translation;
        let l_size = ladder_sprite.custom_size.unwrap();

        if collide(p_translation, p_size, l_translation, l_size).is_some() {
            let current_floor_index = player_dungeon_stats.current_floor_index;

            if current_floor_index < dungeon.floors.len() - 1 {
                dungeon.current_floor = dungeon.floors[current_floor_index + 1].clone();
                player_dungeon_stats.current_floor_index = current_floor_index + 1;
                let start_room_position = dungeon.current_floor.start_room_position;
                player_dungeon_stats.current_room_position = start_room_position;
            }
        }
    }
}
