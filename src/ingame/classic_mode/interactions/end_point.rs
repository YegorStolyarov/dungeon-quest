use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::ingame::resources::dungeon::end_point::EndPoint;
use crate::ingame::resources::dungeon::Dungeon;
use crate::ingame::resources::player::player_dungeon_stats::PlayerDungeonStats;
use crate::ingame::resources::player::Player;
use crate::scenes::SceneState;

pub fn end_point_interaction_handle_system(
    mut player_query: Query<(&Transform, &TextureAtlasSprite), With<Player>>,
    mut end_point_query: Query<
        (&Transform, &Sprite, &Visibility),
        (With<EndPoint>, Without<Player>),
    >,
    mut player_dungeon_stats: ResMut<PlayerDungeonStats>,
    mut state: ResMut<State<SceneState>>,
    mut dungeon: ResMut<Dungeon>,
) {
    let current_position = dungeon.current_floor.current_position;
    let end_room_position = dungeon.current_floor.end_room_position;

    if current_position == end_room_position && player_dungeon_stats.is_room_cleared {
        let (player_transform, player_sprite) = player_query.single_mut();
        let (end_point_transform, end_point_sprite, visibility) = end_point_query.single_mut();

        let p_translation = player_transform.translation;
        let p_size = player_sprite.custom_size.unwrap();
        let ep_translation = end_point_transform.translation;
        let ep_size = end_point_sprite.custom_size.unwrap();

        if visibility.is_visible {
            if collide(p_translation, p_size, ep_translation, ep_size).is_some() {
                if dungeon.current_floor.is_last_floor {
                    state
                        .set(SceneState::ResultScene)
                        .expect("Couldn't switch state to Result Scene");
                } else {
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
    }
}
