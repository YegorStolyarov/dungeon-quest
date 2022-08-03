use bevy::prelude::*;

use crate::resources::animation_state::AnimationState;

#[derive(Component)]
pub struct PlayerAnimation {
    pub animation_timer: Timer,
    pub animation_state: AnimationState,
}

impl PlayerAnimation {
    pub fn new() -> Self {
        PlayerAnimation {
            animation_timer: Timer::from_seconds(0.1, true),
            animation_state: AnimationState::Idle,
        }
    }
}
