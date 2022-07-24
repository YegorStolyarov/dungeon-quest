use bevy::prelude::*;
use std::time::Duration;

use crate::ingame::resources::skill::Skill;

#[derive(Component)]
pub struct PlayerSkill {
    skill: Skill,
    cooldown: Timer,
    monster_cooldown: u8,
}

impl PlayerSkill {
    pub fn new(skill: Skill) -> Self {
        PlayerSkill {
            cooldown: Timer::new(Duration::from_secs(0), false),
            monster_cooldown: skill.required_monster,
            skill,
        }
    }
}
