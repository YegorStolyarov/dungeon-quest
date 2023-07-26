use bevy::prelude::*;

use crate::components::invinsible_cooldown::InvisibleCooldownComponent;
use crate::components::player_animation::PlayerAnimation;
use crate::resources::animation_state::AnimationState;

pub fn player_animation_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut PlayerAnimation,
        &InvisibleCooldownComponent,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut player_animation, invincible_cooldown, mut sprite, texture_atlas_handle) in
        query.iter_mut()
    {
        if !invincible_cooldown.hurt_duration.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = texture_atlas.textures.len() - 1;
        } else {
            player_animation.animation_timer.tick(time.delta());
            if player_animation.animation_timer.just_finished() {
                match player_animation.animation_state {
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
                        if sprite.index >= max_index || sprite.index < min_index {
                            sprite.index = min_index;
                        } else {
                            sprite.index += 1;
                        }
                    }
                    AnimationState::Hit => {}
                }
            }
        }
    }
}
