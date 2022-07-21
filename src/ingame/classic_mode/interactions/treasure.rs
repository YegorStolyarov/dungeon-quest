use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::ingame::resources::dungeon::treasure::Treasure;
use crate::ingame::resources::dungeon::Dungeon;
use crate::ingame::resources::player::player_dungeon_stats::PlayerDungeonStats;
use crate::ingame::resources::player::Player;
use crate::scenes::SceneState;

pub fn treasure_interaction_handle_system(
    mut player_query: Query<(&Transform, &TextureAtlasSprite), With<Player>>,
    mut treasure_query: Query<
        (&Transform, &Sprite, &Visibility),
        (With<Treasure>, Without<Player>),
    >,
    mut state: ResMut<State<SceneState>>,
    player_dungeon_stats: ResMut<PlayerDungeonStats>,
    dungeon: ResMut<Dungeon>,
) {
    let current_position = dungeon.current_floor.current_position;
    let end_room_position = dungeon.current_floor.end_room_position;

    if current_position == end_room_position && player_dungeon_stats.is_room_cleared {
        let (player_transform, player_sprite) = player_query.single_mut();
        let (treasure_transform, treasure_sprite, visibility) = treasure_query.single_mut();

        let p_translation = player_transform.translation;
        let p_size = player_sprite.custom_size.unwrap();
        let t_translation = treasure_transform.translation;
        let t_size = treasure_sprite.custom_size.unwrap();

        if visibility.is_visible {
            if collide(p_translation, p_size, t_translation, t_size).is_some() {
                state
                    .set(SceneState::ResultScene)
                    .expect("Couldn't switch state to Result Scene");
            }
        }
    }
}
