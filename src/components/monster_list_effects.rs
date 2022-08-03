use bevy::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

use crate::resources::effect::effect_type::EffectType;
use crate::resources::effect::Effect;

#[derive(Component, Debug)]
pub struct MonsterListEffectsComponent {
    pub activated_effects: HashMap<Effect, Timer>,
}

impl MonsterListEffectsComponent {
    pub fn activate(&mut self, effect_type: EffectType) {
        match effect_type {
            EffectType::Stun => {
                self.activated_effects.insert(
                    Effect {
                        name: EffectType::Stun,
                        duration: 2,
                        bonus: 0.0,
                    },
                    Timer::new(Duration::from_secs(2), false),
                );
            }
            EffectType::ReduceDamage => {
                self.activated_effects.insert(
                    Effect {
                        name: EffectType::ReduceDamage,
                        duration: 10,
                        bonus: 0.5,
                    },
                    Timer::new(Duration::from_secs(2), false),
                );
            }
            EffectType::Slow => {
                self.activated_effects.insert(
                    Effect {
                        name: EffectType::Slow,
                        duration: 3,
                        bonus: 0.3,
                    },
                    Timer::new(Duration::from_secs(2), false),
                );
            }
            _ => {}
        }
    }
}
