use bevy::prelude::*;

use crate::ingame::resources::dungeon::ladder::Ladder;
use crate::ingame::resources::dungeon::Dungeon;
use crate::ingame::resources::player::player_dungeon_stats::PlayerDungeonStats;
use crate::ingame::resources::player::Player;

use bevy::sprite::collide_aabb::collide;

pub fn ladder_interaction_handle_system(
    mut player_query: Query<(&mut Player, &Transform, &TextureAtlasSprite)>,
    mut ladder_query: Query<(&Ladder, &Transform, &Sprite), Without<Player>>,
    mut player_dungeon_stats: ResMut<PlayerDungeonStats>,
    mut dungeon: ResMut<Dungeon>,
) {
    let current_position = dungeon.current_floor.current_position;
    let end_room_position = dungeon.current_floor.end_room_position;

    if current_position == end_room_position && player_dungeon_stats.is_room_cleared {
        let (_player, player_transform, player_sprite) = player_query.single_mut();
        let (_ladder, ladder_transform, ladder_sprite) = ladder_query.single_mut();

        if collide(
            player_transform.translation,
            player_sprite.custom_size.unwrap(),
            ladder_transform.translation,
            ladder_sprite.custom_size.unwrap(),
        )
        .is_some()
        {
            let current_floor_index = player_dungeon_stats.current_floor_index;

            if current_floor_index < dungeon.floors.len() - 1 {
                dungeon.current_floor = dungeon.floors[current_floor_index + 1].clone();
                player_dungeon_stats.current_floor_index = current_floor_index + 1;
                player_dungeon_stats.current_room_position =
                    dungeon.current_floor.start_room_position;
            }
        }
    }
}
