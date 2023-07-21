use bevy::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

use crate::resources::effect::effect_type::EffectType;

#[derive(Component, Debug)]
pub struct MonsterListEffectsComponent {
    pub activated_effects: HashMap<EffectType, Timer>,
}

impl MonsterListEffectsComponent {
    pub fn new() -> MonsterListEffectsComponent {
        let mut monster_list_effects = MonsterListEffectsComponent {
            activated_effects: HashMap::new(),
        };

        monster_list_effects
            .activated_effects
            .insert(EffectType::Stun, Timer::new(Duration::from_secs(0), TimerMode::Once));

        monster_list_effects
            .activated_effects
            .insert(EffectType::Slow, Timer::new(Duration::from_secs(0), TimerMode::Once));

        monster_list_effects.activated_effects.insert(
            EffectType::ReduceDamage,
            Timer::new(Duration::from_secs(0), TimerMode::Once),
        );

        monster_list_effects
    }

    pub fn activate(&mut self, effect_type: EffectType) {
        match effect_type {
            EffectType::Stun => {
                self.activated_effects
                    .insert(EffectType::Stun, Timer::new(Duration::from_secs(2), TimerMode::Once));
            }
            EffectType::ReduceDamage => {
                self.activated_effects.insert(
                    EffectType::ReduceDamage,
                    Timer::new(Duration::from_secs(10), TimerMode::Once),
                );
            }
            EffectType::Slow => {
                self.activated_effects
                    .insert(EffectType::Slow, Timer::new(Duration::from_secs(3), TimerMode::Once));
            }
            _ => {}
        }
    }
}
