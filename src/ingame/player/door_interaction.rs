use bevy::prelude::*;

use crate::ingame::resources::dungeon::door::Door;
use crate::ingame::resources::dungeon::position::Position;
use crate::ingame::resources::dungeon::Dungeon;
use crate::ingame::resources::player::player_dungeon_stats::PlayerDungeonStats;
use crate::ingame::resources::player::Player;

use bevy::sprite::collide_aabb::collide;

pub fn door_interaction_handle_system(
    mut player_query: Query<(&mut Player, &mut Transform, &TextureAtlasSprite)>,
    mut door_query: Query<(&Door, &Transform, &Sprite, &Visibility), Without<Player>>,
    mut player_dungeon_stats: ResMut<PlayerDungeonStats>,
    mut dungeon: ResMut<Dungeon>,
) {
    let (_player, mut player_transform, player_sprite) = player_query.single_mut();
    let player_size = player_sprite.custom_size.unwrap();

    if player_dungeon_stats.is_room_cleared {
        let current_position = player_dungeon_stats.current_room_position;

        for (door, door_transform, door_sprite, visibility) in door_query.iter_mut() {
            if visibility.is_visible {
                if *door == Door::Left || *door == Door::Right {
                    continue;
                }
            }

            let door_size = if *door == Door::Top {
                Vec2::new(128.0, 10.0)
            } else if *door == Door::Bottom {
                Vec2::new(128.0, 36.0)
            } else {
                door_sprite.custom_size.unwrap()
            };

            if collide(
                player_transform.translation,
                player_size,
                door_transform.translation,
                door_size,
            )
            .is_some()
            {
                let new_position = Position {
                    row_index: if *door == Door::Top {
                        current_position.row_index - 1
                    } else if *door == Door::Bottom {
                        current_position.row_index + 1
                    } else {
                        current_position.row_index
                    },
                    column_index: if *door == Door::Left {
                        current_position.column_index - 1
                    } else if *door == Door::Right {
                        current_position.column_index + 1
                    } else {
                        current_position.column_index
                    },
                };

                player_dungeon_stats.current_room_position = new_position;
                dungeon.current_floor.current_position = new_position;

                if dungeon
                    .current_floor
                    .cleared_positions
                    .contains_key(&new_position)
                {
                    player_dungeon_stats.is_room_cleared = true;
                    let total_entered_time = dungeon
                        .current_floor
                        .cleared_positions
                        .get(&new_position)
                        .unwrap()
                        .clone();
                    dungeon
                        .current_floor
                        .cleared_positions
                        .insert(new_position, total_entered_time + 1);
                } else {
                    player_dungeon_stats.is_room_cleared = false;
                }

                if *door == Door::Left {
                    player_transform.translation.x = (player_transform.translation.x * -1.0) - 15.0;
                } else if *door == Door::Right {
                    player_transform.translation.x = (player_transform.translation.x * -1.0) + 15.0;
                } else if *door == Door::Top {
                    player_transform.translation.y = -130.0;
                } else {
                    player_transform.translation.y = 130.0;
                }
            }
        }
    }
}
