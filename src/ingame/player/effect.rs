use bevy::prelude::*;

use crate::ingame::resources::player::player_effects::PlayerEffects;

pub fn update_effects(time: Res<Time>, mut player_effects: ResMut<PlayerEffects>) {
    for duration in player_effects.activated_effects.values_mut() {
        if !duration.finished() {
            duration.tick(time.delta());
        }
    }
}
