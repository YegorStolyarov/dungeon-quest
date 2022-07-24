use bevy::prelude::*;
use std::time::Duration;

use crate::ingame::resources::skill::Skill;

pub struct PlayerSkill {
    pub require_monsters: u32,
    pub monster_counter: u32,
    pub duration: Timer,
    pub cooldown: Timer,
    pub skill: Skill,
}

impl PlayerSkill {
    pub fn new(skill: Skill) -> Self {
        PlayerSkill {
            cooldown: Timer::new(Duration::from_secs(0), false),
            duration: Timer::new(Duration::from_secs(0), false),
            require_monsters: skill.require_monsters.unwrap_or(0),
            monster_counter: 0,
            skill,
        }
    }
}
