use bevy::prelude::*;
use std::time::Duration;

use crate::resources::skill::skill_type::SkillType;
use crate::resources::skill::Skill;
use crate::resources::upgrade::Upgrade;

#[derive(Component)]
pub struct SkillComponent {
    pub require_monsters: u32,
    pub monster_counter: u32,
    pub duration: Timer,
    pub cooldown: Timer,
    pub skill: Skill,
}

impl SkillComponent {
    pub fn new(skill: Skill) -> Self {
        let mut duration = Timer::new(Duration::from_secs(0), false);
        duration.tick(Duration::from_secs(0));

        SkillComponent {
            cooldown: Timer::new(Duration::from_secs(3), false),
            duration,
            require_monsters: skill.require_monsters.unwrap_or(0),
            monster_counter: 0,
            skill,
        }
    }

    pub fn upgrade(&mut self, upgrade: Upgrade) {
        let skill_upgrade = upgrade.skill_upgrade.unwrap();

        let duration_bonus = skill_upgrade.duration_bonus.unwrap_or(0);
        let cooldown_reduce = skill_upgrade.cooldown_reduce.unwrap_or(0);
        let require_monsters_reduce = skill_upgrade.require_monsters_reduce.unwrap_or(0);

        let speed_percent_bonus = skill_upgrade.speed_percent_bonus.unwrap_or(0.0);
        let critical_chance_bonus = skill_upgrade.critical_chance_bonus.unwrap_or(0.0);
        let restore_chance_bonus = skill_upgrade.restore_chance_bonus.unwrap_or(0.0);
        let dodge_chance_bonus = skill_upgrade.dodge_chance_bonus.unwrap_or(0.0);

        let skill_duration = self.skill.duration.unwrap_or(0);
        let skill_cooldown = self.skill.cooldown.unwrap_or(0);
        let speed_percent = self.skill.speed_percent_bonus.unwrap_or(0.0);
        let critical_chance = self.skill.speed_percent_bonus.unwrap_or(0.0);
        let require_monsters = self.skill.require_monsters.unwrap_or(0);
        let restore_chance = self.skill.restore_chance_bonus.unwrap_or(0.0);
        let dodge_chance = self.skill.dodge_chance_bonus.unwrap_or(0.0);

        match self.skill.name {
            SkillType::TimeToHunt => {
                self.skill.duration = Some(skill_duration + duration_bonus);
                self.skill.cooldown = Some(skill_cooldown - cooldown_reduce);
                self.skill.speed_percent_bonus = Some(speed_percent_bonus + speed_percent);
                self.skill.critical_chance_bonus = Some(critical_chance + critical_chance_bonus);
            }
            SkillType::Armor => {
                self.skill.require_monsters = Some(require_monsters - require_monsters_reduce);
            }
            SkillType::Thunderstorm => {
                self.skill.cooldown = Some(skill_cooldown - cooldown_reduce);
            }
            SkillType::AnimalInstinct => {
                self.skill.duration = Some(skill_duration + duration_bonus);
                self.skill.cooldown = Some(skill_cooldown - cooldown_reduce);
                self.skill.speed_percent_bonus = Some(speed_percent_bonus + speed_percent);
                self.skill.critical_chance_bonus = Some(critical_chance + critical_chance_bonus);
                self.skill.restore_chance_bonus = Some(restore_chance + restore_chance_bonus);
                self.skill.dodge_chance_bonus = Some(dodge_chance + dodge_chance_bonus);
            }
        };
    }
}
