use bevy::prelude::*;

use crate::{
    components::monster_list_effects::MonsterListEffectsComponent,
    resources::effect::effect_type::EffectType,
};

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

pub fn update_color_of_effects(
    mut monsters_effects_query: Query<(&MonsterListEffectsComponent, &mut TextureAtlasSprite)>,
) {
    for (monster_list_effects, mut texture) in monsters_effects_query.iter_mut() {
        for (effect_type, duration) in monster_list_effects.activated_effects.iter() {
            if !duration.finished() {
                if *effect_type == EffectType::Stun {
                    texture.color = Color::GRAY;
                } else if *effect_type == EffectType::Slow {
                    texture.color = Color::ALICE_BLUE;
                } else if *effect_type == EffectType::ReduceDamage {
                    texture.color = Color::YELLOW;
                }
            }
        }
    }
}
