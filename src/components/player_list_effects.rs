use bevy::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

use crate::resources::effect::effect_type::EffectType;
use crate::resources::effect::Effect;
use crate::resources::upgrade::Upgrade;

#[derive(Component, Debug)]
pub struct PlayerListEffectsComponent {
    pub information: Vec<Effect>,
    pub activated_effects: HashMap<EffectType, Timer>,
}

impl PlayerListEffectsComponent {
    pub fn new(information: Vec<Effect>) -> PlayerListEffectsComponent {
        PlayerListEffectsComponent {
            information,
            activated_effects: HashMap::new(),
        }
    }

    pub fn upgrade(&mut self, upgrade: Upgrade) {
        let effect_upgrade = upgrade.effect_upgrade.unwrap();

        let mut information = self
            .information
            .iter_mut()
            .find(|effect_information| effect_information.name == effect_upgrade.name)
            .unwrap();

        let duration_bonus = effect_upgrade.duration_bonus.unwrap_or(0);
        let duration_reduce = effect_upgrade.duration_reduce.unwrap_or(0);

        let speed_percent_bonus = effect_upgrade.speed_percent_bonus.unwrap_or(0.0);
        let speed_percent_reduce = effect_upgrade.speed_percent_reduce.unwrap_or(0.0);
        let critical_chance_bonus = effect_upgrade.critical_chance_bonus.unwrap_or(0.0);
        let dodge_chance_bonus = effect_upgrade.dodge_chance_bonus.unwrap_or(0.0);

        let duration = duration_bonus - duration_reduce;
        let bonus =
            speed_percent_bonus - speed_percent_reduce + critical_chance_bonus + dodge_chance_bonus;

        information.duration = if information.duration + duration > 0 {
            information.duration + duration
        } else {
            2
        };

        information.bonus = information.bonus + bonus;
    }

    pub fn activate(&mut self, effect_type: EffectType) {
        dbg!("IN");
        let information = self
            .information
            .iter_mut()
            .find(|effect_information| effect_information.name == effect_type)
            .unwrap();

        self.activated_effects.insert(
            effect_type,
            Timer::new(Duration::from_secs(information.duration as u64), false),
        );
    }
}
