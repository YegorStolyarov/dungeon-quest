use bevy::prelude::*;
use std::time::Duration;

use crate::ingame::resources::skill::Skill;

#[derive(Component)]
pub struct PlayerSkill {
    skill: Skill,
    cooldown: Timer,
}

impl PlayerSkill {
    pub fn new(skill: Skill) -> Self {
        PlayerSkill {
            cooldown: Timer::new(Duration::from_secs(0), false),
            skill,
        }
    }
}
