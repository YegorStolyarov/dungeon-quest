use bevy::prelude::*;

use crate::components::monster_list_effects::MonsterListEffectsComponent;

pub fn update_effects(
    mut monsters_effects_query: Query<&mut MonsterListEffectsComponent>,
    time: Res<Time>,
) {
    for mut monster_list_effects in monsters_effects_query.iter_mut() {
        for duration in monster_list_effects.activated_effects.values_mut() {
            if !duration.finished() {
                duration.tick(time.delta());
            }
        }
    }
}
