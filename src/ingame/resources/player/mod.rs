use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::ingame::resources::data::Data;

use crate::ingame::resources::hero::hero_class::HeroClass;
use crate::ingame::resources::hero::power::Power;
use crate::ingame::resources::hero::stats::Stats;
use crate::ingame::resources::weapon::Weapon;

pub mod player_animation;
pub mod player_available_movement;
pub mod player_dungeon_stats;
pub mod player_effects;
pub mod player_skill;

#[derive(Component, Inspectable)]
pub struct Player {
    pub class: HeroClass,

    pub current_health_points: f32,
    pub max_health_points: f32,
    pub speed: f32,
    pub strength: f32,
    pub intelligence: f32,

    pub critical_chance: f32,
    pub dodge_chance: f32,
    pub restore_chance: f32,

    pub bonus_damage: f32,

    pub power: Power,
    pub base_stats: Stats,
    pub weapon: Weapon,
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
            bonus_damage: 0.0,
            base_stats: base_stats.clone(),
            weapon,
        }
    }
}
