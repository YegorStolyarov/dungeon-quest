use bevy::prelude::*;

use crate::{
    components::monster_animation::MonsterAnimationComponent,
    resources::animation_state::AnimationState,
};

pub fn animation_handle(
    mut monsters_animation_query: Query<(&mut MonsterAnimationComponent, &mut TextureAtlasSprite)>,
    time: Res<Time>,
) {
    for (mut monster_animation, mut sprite) in monsters_animation_query.iter_mut() {
        monster_animation.animation_timer.tick(time.delta());
        if monster_animation.animation_timer.just_finished() {
            match monster_animation.animation_state {
                AnimationState::Idle => {
                    let min_index = 0;
                    let max_index = 3;
                    if sprite.index >= max_index || sprite.index < min_index {
                        sprite.index = min_index;
                    } else {
                        sprite.index += 1;
                    }
                }
                AnimationState::Moving => {
                    if monster_animation.total_tiles == 8 {
                        let min_index = 4;
                        let max_index = 7;
                        if sprite.index >= max_index || sprite.index < min_index {
                            sprite.index = min_index;
                        } else {
                            sprite.index += 1;
                        }
                    } else {
                        let min_index = 0;
                        let max_index = 3;
                        if sprite.index >= max_index || sprite.index < min_index {
                            sprite.index = min_index;
                        } else {
                            sprite.index += 1;
                        }
                    }
                }
                AnimationState::Hit => {}
            }
        }
    }
}
