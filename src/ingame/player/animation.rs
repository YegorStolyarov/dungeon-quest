use bevy::prelude::*;

use crate::ingame::resources::fixed::animation_state::AnimationState;
use crate::ingame::resources::player::Player;

pub fn player_animation_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Player, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut player, mut sprite, texture_atlas_handle) in query.iter_mut() {
        player.animation_timer.tick(time.delta());
        if player.animation_timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            match player.animation_state {
                AnimationState::Idle => {
                    let min_index = 0;
                    let max_index = 3;
                    if sprite.index > max_index || sprite.index < min_index {
                        sprite.index = min_index;
                    } else {
                        sprite.index += 1;
                    }
                }
                AnimationState::Moving => {
                    let min_index = 4;
                    let max_index = 7;
                    if sprite.index > max_index || sprite.index < min_index {
                        sprite.index = min_index;
                    } else {
                        sprite.index += 1;
                    }
                }
                AnimationState::Hit => {
                    sprite.index = texture_atlas.textures.len() - 1;
                }
            }
        }
    }
}
