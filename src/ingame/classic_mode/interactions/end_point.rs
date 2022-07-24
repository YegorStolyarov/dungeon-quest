use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use std::time::Duration;

use crate::ingame::classic_mode::ui::CenterText;
use crate::ingame::resources::dungeon::end_point::EndPoint;
use crate::ingame::resources::dungeon::Dungeon;
use crate::ingame::resources::player::player_dungeon_stats::PlayerDungeonStats;
use crate::ingame::resources::player::Player;
use crate::ingame::resources::profile::Profile;

pub fn end_point_interaction_handle_system(
    mut player_query: Query<(&Transform, &TextureAtlasSprite), With<Player>>,
    mut end_point_query: Query<
        (&Transform, &Sprite, &Visibility),
        (With<EndPoint>, Without<Player>),
    >,
    mut ui_center_text_query: Query<&mut CenterText>,
    mut player_dungeon_stats: ResMut<PlayerDungeonStats>,
    mut dungeon: ResMut<Dungeon>,
    mut profile: ResMut<Profile>,
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
                    profile.is_run_completed = true;
                    profile.is_run_finished = true;
                } else {
                    let current_floor_index = player_dungeon_stats.current_floor_index;

                    if current_floor_index < dungeon.floors.len() - 1 {
                        dungeon.current_floor = dungeon.floors[current_floor_index + 1].clone();
                        player_dungeon_stats.current_floor_index = current_floor_index + 1;
                        let start_room_position = dungeon.current_floor.start_room_position;
                        player_dungeon_stats.current_room_position = start_room_position;

                        ui_center_text_query.single_mut().timer =
                            Timer::new(Duration::from_secs(1), false);
                    }
                }
            }
        }
    }
}
