use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::resources::game_data::GameData;
use crate::resources::hero::hero_class::HeroClass;
use crate::resources::hero::power::Power;
use crate::resources::hero::stats::Stats;
use crate::resources::upgrade::Upgrade;

#[derive(Component, Inspectable)]
pub struct PlayerComponent {
    pub class: HeroClass,
    pub current_health_points: f32,
    pub max_health_points: f32,
    pub speed: f32,
    pub strength: f32,
    pub intelligence: f32,
    pub critical_chance: f32,
    pub dodge_chance: f32,
    pub restore_chance: f32,
    pub damage_percent_bonus: f32,
    pub power: Power,
    pub base_stats: Stats,
}

impl PlayerComponent {
    pub fn new(hero_class: HeroClass, game_data: GameData) -> Self {
        let hero = game_data.get_hero(hero_class.clone());
        let base_stats = hero.stats;

        PlayerComponent {
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
            damage_percent_bonus: 0.0,
            base_stats: base_stats.clone(),
        }
    }

    pub fn upgrade_stats(&mut self, upgrade: Upgrade) {
        let stats_upgrade = upgrade.stats_upgrade.unwrap();
        let critical_chance_bonus_upgrade = stats_upgrade.critical_chance_bonus.unwrap_or(0.0);
        let dodge_chance_bonus_upgrade = stats_upgrade.dodge_chance_bonus.unwrap_or(0.0);
        let restore_chance_bonus_upgrade = stats_upgrade.restore_chance_bonus.unwrap_or(0.0);
        let intelligence_bonus_upgrade = stats_upgrade.intelligence_bonus.unwrap_or(0.0);
        let strength_bonus_upgrade = stats_upgrade.strength_bonus.unwrap_or(0.0);
        let max_health_bonus_upgrade = stats_upgrade.max_health_bonus.unwrap_or(0.0);
        let speed_percent_bonus_upgrade = stats_upgrade.speed_percent_bonus.unwrap_or(0.0);
        let speed_bonus_upgrade = speed_percent_bonus_upgrade * self.base_stats.speed;

        self.max_health_points += max_health_bonus_upgrade;
        self.base_stats.critical_chance += critical_chance_bonus_upgrade;
        self.base_stats.dodge_chance += dodge_chance_bonus_upgrade;
        self.base_stats.restore_chance += restore_chance_bonus_upgrade;
        self.intelligence += intelligence_bonus_upgrade;
        self.strength += strength_bonus_upgrade;
        self.base_stats.speed += speed_bonus_upgrade;
    }
}
