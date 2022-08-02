use bevy::prelude::*;

use crate::components::player_list_effects::PlayerListEffectsComponent;

pub fn update_effects(
    mut player_list_effects_query: Query<&mut PlayerListEffectsComponent>,
    time: Res<Time>,
) {
    let mut player_list_effects = player_list_effects_query.single_mut();
    for duration in player_list_effects.activated_effects.values_mut() {
        if !duration.finished() {
            duration.tick(time.delta());
        }
    }
}
