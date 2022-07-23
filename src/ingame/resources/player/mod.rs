use bevy::prelude::*;

use crate::ingame::resources::fixed::animation_state::AnimationState;
use crate::ingame::resources::fixed::data::Data;
use crate::ingame::resources::fixed::hero_class::HeroClass;
use crate::ingame::resources::fixed::power::Power;
use crate::ingame::resources::fixed::skill::Skill;
use crate::ingame::resources::fixed::stats::Stats;
use crate::ingame::resources::fixed::weapon::Weapon;

pub mod player_available_movement;
pub mod player_dungeon_stats;
pub mod player_effect;

use player_effect::PlayerEffect;

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
    skill: Skill,

    effects: Vec<PlayerEffect>,

    pub animation_timer: Timer,
    pub animation_state: AnimationState,
}

impl Player {
    pub fn new(hero_class: HeroClass, data: Data) -> Self {
        let hero = data
            .heros
            .iter()
            .find(|hero| hero.hero_class == hero_class)
            .unwrap()
            .clone();

        let weapon = data
            .weapons
            .iter()
            .find(|weapon| weapon.name == hero.weapon)
            .unwrap()
            .clone();

        let skill = data
            .skills
            .iter()
            .find(|skill| skill.name == hero.skill)
            .unwrap()
            .clone();

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
            skill,
            effects: Vec::new(),
            animation_timer: Timer::from_seconds(0.1, true),
            animation_state: AnimationState::Idle,
        }
    }
}
