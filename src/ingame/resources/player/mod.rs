use bevy::prelude::*;
use std::collections::HashMap;

use crate::ingame::resources::animation_state::AnimationState;
use crate::ingame::resources::data::Data;
use crate::ingame::resources::effect::effect_type::EffectType;
use crate::ingame::resources::hero::hero_class::HeroClass;
use crate::ingame::resources::hero::power::Power;
use crate::ingame::resources::hero::stats::Stats;
use crate::ingame::resources::weapon::Weapon;

pub mod player_available_movement;
pub mod player_dungeon_stats;
pub mod player_skill;

#[derive(Component)]
pub struct Player {
    class: HeroClass,

    current_health_points: f32,
    max_health_points: f32,
    pub speed: f32,
    strength: f32,
    intelligence: f32,
    critical_chance: f32,
    dodge_chance: f32,
    restore_chance: f32,

    power: Power,
    base_stats: Stats,
    weapon: Weapon,

    pub effects: HashMap<EffectType, Timer>,

    pub animation_timer: Timer,
    pub animation_state: AnimationState,
}

impl Player {
    pub fn new(hero_class: HeroClass, data: Data) -> Self {
        let hero = data.get_hero(hero_class.clone());
        let weapon = data.get_weapon(hero_class.clone());

        let base_stats = hero.stats;

        Player {
            class: hero_class,
            current_health_points: base_stats.health_points,
            max_health_points: base_stats.health_points,
            speed: base_stats.speed,
            strength: base_stats.strength,
            intelligence: base_stats.intelligence,
            critical_chance: base_stats.critical_chance,
            dodge_chance: base_stats.dodge_chance,
            restore_chance: base_stats.restore_chance,
            power: hero.power,
            base_stats: base_stats.clone(),
            weapon,
            effects: HashMap::new(),
            animation_timer: Timer::from_seconds(0.1, true),
            animation_state: AnimationState::Idle,
        }
    }
}
