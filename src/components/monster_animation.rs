use bevy::prelude::*;

use crate::resources::animation_state::AnimationState;

#[derive(Component)]
pub struct MonsterAnimationComponent {
    pub animation_state: AnimationState,
    pub animation_timer: Timer,
    pub total_tiles: u8,
}
