use bevy::prelude::*;

use crate::ingame::resources::dungeon::treasure::Treasure;
use crate::ingame::resources::dungeon::Dungeon;
use crate::ingame::resources::player::player_dungeon_stats::PlayerDungeonStats;
use crate::ingame::resources::player::Player;

use bevy::sprite::collide_aabb::collide;

pub fn treasure_interaction_handle_system(
    mut player_query: Query<(&mut Player, &Transform, &TextureAtlasSprite)>,
    mut treasure_query: Query<(&Treasure, &Transform, &Sprite, &Visibility), Without<Player>>,
    player_dungeon_stats: ResMut<PlayerDungeonStats>,
    dungeon: ResMut<Dungeon>,
) {
    let current_position = dungeon.current_floor.current_position;
    let end_room_position = dungeon.current_floor.end_room_position;

    if current_position == end_room_position && player_dungeon_stats.is_room_cleared {
        let (_player, player_transform, player_sprite) = player_query.single_mut();
        let (_t, treasure_transform, treasure_sprite, visibility) = treasure_query.single_mut();

        if visibility.is_visible {
            if collide(
                player_transform.translation,
                player_sprite.custom_size.unwrap(),
                treasure_transform.translation,
                treasure_sprite.custom_size.unwrap(),
            )
            .is_some()
            {
                dbg!("Win");
            }
        }
    }
}
